mod db;
mod exif;
mod models;
mod schema;

// Required for dsync for some reason
pub use diesel;
use diesel::{Connection, SqliteConnection};
use eyre::Context;
use models::{
    files::{CreateFiles, Files},
    tag_file::{CreateTagFile, TagFile},
    tags::{self, CreateTags, TagId, Tags},
};

use std::{
    collections::HashSet,
    env,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
    sync::{
        mpsc::{channel, Sender},
        LazyLock,
    },
    time::{SystemTime, UNIX_EPOCH},
};

use tokio::runtime::Runtime;
use tracing::{debug, error};

pub fn initialize_thread() {
    static RT: LazyLock<Runtime> =
        LazyLock::new(|| Runtime::new().expect("Cannot initialize runtime"));
    thread_local! {
        static HANDLE: tokio::runtime::EnterGuard<'static> = {
            install_tracing();
            RT.enter()
        }
    }
    HANDLE.with(|_| {});
}

pub fn connection() -> SqliteConnection {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn install_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("trace"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}
struct Entry {
    created: Option<SystemTime>,
    path: String,
}

fn walkdir_full_scan(dir: &impl AsRef<Path>, tx: Sender<Entry>, paths: HashSet<String>) {
    let wd = walkdir::WalkDir::new(dir);
    for entry in wd {
        match entry {
            Ok(entry) => {
                let mdata = match entry.metadata() {
                    Ok(e) => e,
                    Err(err) => {
                        error!("Error reading {:?}: {err}", entry.path());
                        continue;
                    }
                };
                if !mdata.is_file() {
                    continue;
                }

                let path = entry.into_path().into_os_string().into_string();
                let path = match path {
                    Ok(o) => o,
                    Err(e) => String::from_utf8_lossy(e.as_bytes()).to_string(),
                };
                if paths.contains(&path) {
                    continue;
                }

                let created = mdata.created().ok();
                let entry = Entry { created, path };
                tx.send(entry).expect("rx dropped?!");
            }
            Err(e) => {
                error!("Error reading {:?}: {e}", dir.as_ref());
            }
        }
    }
}

pub fn scan_directory(dir: impl Into<PathBuf>) {
    let dir = dir.into();
    let (tx, rx) = channel();

    let db = &mut connection();

    let paths = Files::get_all_paths(db).unwrap_or_default();

    rayon::spawn(move || {
        walkdir_full_scan(&dir, tx, paths);
    });

    while let Ok(entry) = rx.recv() {
        let create = std::iter::once(entry)
            .chain(rx.try_iter())
            .map(|entry| CreateFiles {
                path: entry.path,
                created: entry
                    .created
                    .and_then(|x| x.duration_since(UNIX_EPOCH).ok())
                    .map(|x| x.as_secs() as i64),
                deleted: false,
            })
            .collect::<Vec<_>>();

        match Files::insert_many(db, &create) {
            Ok(num) => {
                debug!("Inserted {num} fields");
            }
            Err(e) => {
                error!("Error inserting fields into the db: {e}")
            }
        }
    }
}

pub fn create_tag(
    name: &str,
    description: Option<&str>,
    parent: Option<i32>,
    db: &mut SqliteConnection,
) -> eyre::Result<i32> {
    Tags::create(
        db,
        &CreateTags {
            parent,
            name,
            description,
        },
    )
    .context("Database error creating tag")
}

pub fn tag_file(file: i32, tag: i32, db: &mut SqliteConnection) -> eyre::Result<()> {
    TagFile::create(
        db,
        &CreateTagFile {
            label_id: file,
            tag_id: tag,
        },
    )
    .context("Database error while tagging file")
}

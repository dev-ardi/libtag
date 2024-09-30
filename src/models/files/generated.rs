/* @generated and managed by dsync */

use std::collections::HashSet;

use connection::DefaultLoadingMode;

#[allow(unused)]
use crate::diesel::*;
use crate::schema::*;

pub type ConnectionType = diesel::sqlite::SqliteConnection;

/// Struct representing a row in table `files`
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::Queryable,
    diesel::Selectable,
    diesel::QueryableByName,
    diesel::Identifiable,
)]
#[diesel(table_name=files, primary_key(id))]
pub struct Files {
    /// Field representing column `id`
    pub id: i32,
    /// Field representing column `path`
    pub path: String,
    /// Field representing column `name`
    pub name: Option<String>,
    /// Field representing column `author`
    pub author: Option<String>,
    /// Field representing column `artist`
    pub artist: Option<String>,
    /// Field representing column `description`
    pub description: Option<String>,
    /// Field representing column `notes`
    pub notes: Option<String>,
    /// Field representing column `created`
    pub created: Option<i64>,
    /// Field representing column `gps_altitude`
    pub gps_altitude: Option<i32>,
    /// Field representing column `gps_latitude`
    pub gps_latitude: Option<i32>,
    /// Field representing column `x_res`
    pub x_res: Option<i32>,
    /// Field representing column `y_res`
    pub y_res: Option<i32>,
    /// Field representing column `deleted`
    pub deleted: bool,
    pub is_exif: Option<bool>,
    /// Field representing column `updated_at`
    pub updated_at: Option<i64>,
}

/// Create Struct for a row in table `files` for [`Files`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[diesel(table_name=files)]
pub struct CreateFiles {
    pub path: String,
    pub created: Option<i64>,
    pub deleted: bool,
}

/// Result of a `.paginate` function
#[derive(Debug, serde::Serialize)]
pub struct PaginationResult<T> {
    /// Resulting items that are from the current page
    pub items: Vec<T>,
    /// The count of total items there are
    pub total_items: i64,
    /// Current page, 0-based index
    pub page: i64,
    /// Size of a page
    pub page_size: i64,
    /// Number of total possible pages, given the `page_size` and `total_items`
    pub num_pages: i64,
}

impl Files {
    pub fn insert_many(
        db: &mut ConnectionType,
        item: &[CreateFiles],
    ) -> diesel::QueryResult<usize> {
        use crate::schema::files::dsl::*;

        diesel::insert_into(files).values(item).execute(db)
    }

    pub fn get_all_paths(db: &mut ConnectionType) -> QueryResult<HashSet<String>> {
        use crate::schema::files::dsl::*;
        files
            .select(path)
            .load_iter::<String, DefaultLoadingMode>(db)?
            .collect()
    }

    pub fn get_path_id(db: &mut ConnectionType) -> QueryResult<HashSet<String>> {
        use crate::schema::files::dsl::*;
        files
            .select(path)
            .load_iter::<String, DefaultLoadingMode>(db)?
            .collect()
    }
}

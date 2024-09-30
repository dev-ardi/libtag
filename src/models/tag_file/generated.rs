/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::models::files::Files;
use crate::models::tags::Tags;
use crate::schema::*;

pub type ConnectionType = diesel::sqlite::SqliteConnection;

/// Struct representing a row in table `tag_file`
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::Queryable,
    diesel::Selectable,
    diesel::QueryableByName,
    diesel::Associations,
    diesel::Identifiable,
)]
#[diesel(table_name=tag_file, primary_key(id), belongs_to(Files, foreign_key=label_id) , belongs_to(Tags, foreign_key=tag_id))]
pub struct TagFile {
    /// Field representing column `id`
    pub id: i32,
    /// Field representing column `label_id`
    pub label_id: i32,
    /// Field representing column `tag_id`
    pub tag_id: i32,
    /// Field representing column `created_at`
    pub created_at: Option<i64>,
}

/// Create Struct for a row in table `tag_file` for [`TagFile`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[diesel(table_name=tag_file)]
pub struct CreateTagFile {
    /// Field representing column `label_id`
    pub label_id: i32,
    /// Field representing column `tag_id`
    pub tag_id: i32,
    /// Field representing column `created_at`
    pub created_at: Option<i64>,
}

/// Update Struct for a row in table `tag_file` for [`TagFile`]
#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, diesel::AsChangeset, PartialEq, Default,
)]
#[diesel(table_name=tag_file)]
pub struct UpdateTagFile {
    /// Field representing column `label_id`
    pub label_id: Option<i32>,
    /// Field representing column `tag_id`
    pub tag_id: Option<i32>,
    /// Field representing column `created_at`
    pub created_at: Option<Option<i64>>,
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

impl TagFile {
    /// Insert a new row into `tag_file` with a given [`CreateTagFile`]
    pub fn create(db: &mut ConnectionType, item: &CreateTagFile) -> diesel::QueryResult<Self> {
        use crate::schema::tag_file::dsl::*;

        diesel::insert_into(tag_file)
            .values(item)
            .get_result::<Self>(db)
    }

    pub fn insert(db: &mut ConnectionType, item: &CreateTagFile) -> diesel::QueryResult<usize> {
        use crate::schema::tag_file::dsl::*;

        diesel::insert_into(tag_file).values(item).execute(db)
    }

    /// Get a row from `tag_file`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: i32) -> diesel::QueryResult<Self> {
        use crate::schema::tag_file::dsl::*;

        tag_file.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Update a row in `tag_file`, identified by the primary key with [`UpdateTagFile`]
    pub fn update(
        db: &mut ConnectionType,
        param_id: i32,
        item: &UpdateTagFile,
    ) -> diesel::QueryResult<Self> {
        use crate::schema::tag_file::dsl::*;

        diesel::update(tag_file.filter(id.eq(param_id)))
            .set(item)
            .get_result(db)
    }

    /// Delete a row in `tag_file`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: i32) -> diesel::QueryResult<usize> {
        use crate::schema::tag_file::dsl::*;

        diesel::delete(tag_file.filter(id.eq(param_id))).execute(db)
    }
}

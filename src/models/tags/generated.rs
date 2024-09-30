/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::schema::*;

pub type ConnectionType = diesel::sqlite::SqliteConnection;

pub struct TagId(pub i32);

/// Struct representing a row in table `tags`
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
#[diesel(table_name=tags, primary_key(id))]
pub struct Tags {
    /// Field representing column `id`
    pub id: i32,
    /// Field representing column `parent`
    pub parent: Option<i32>,
    /// Field representing column `name`
    pub name: String,
    /// Field representing column `description`
    pub description: Option<String>,
    /// Field representing column `created_at`
    pub created_at: Option<i64>,
    /// Field representing column `updated_at`
    pub updated_at: Option<i64>,
}

/// Create Struct for a row in table `tags` for [`Tags`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[diesel(table_name=tags)]
pub struct CreateTags<'a> {
    /// Field representing column `parent`
    pub parent: Option<i32>,
    /// Field representing column `name`
    pub name: &'a str,
    /// Field representing column `description`
    pub description: Option<&'a str>,
}

/// Update Struct for a row in table `tags` for [`Tags`]
#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, diesel::AsChangeset, PartialEq, Default,
)]
#[diesel(table_name=tags)]
pub struct UpdateTags {
    /// Field representing column `parent`
    pub parent: Option<i32>,
    /// Field representing column `name`
    pub name: Option<String>,
    /// Field representing column `description`
    pub description: Option<Option<String>>,
    /// Field representing column `created_at`
    pub created_at: Option<Option<i64>>,
    /// Field representing column `updated_at`
    pub updated_at: Option<Option<i64>>,
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

impl Tags {
    /// Insert a new row into `tags` with a given [`CreateTags`]
    pub fn create(db: &mut ConnectionType, item: &CreateTags) -> diesel::QueryResult<i32> {
        use crate::schema::tags::dsl::*;

        diesel::insert_into(tags)
            .values(item)
            .returning(id)
            .get_result::<i32>(db)
    }

    pub fn insert(db: &mut ConnectionType, item: &CreateTags) -> diesel::QueryResult<usize> {
        use crate::schema::tags::dsl::*;

        diesel::insert_into(tags).values(item).execute(db)
    }

    /// Get a row from `tags`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: i32) -> diesel::QueryResult<Self> {
        use crate::schema::tags::dsl::*;

        tags.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Update a row in `tags`, identified by the primary key with [`UpdateTags`]
    pub fn update(
        db: &mut ConnectionType,
        param_id: i32,
        item: &UpdateTags,
    ) -> diesel::QueryResult<Self> {
        use crate::schema::tags::dsl::*;

        diesel::update(tags.filter(id.eq(param_id)))
            .set(item)
            .get_result(db)
    }

    /// Delete a row in `tags`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: i32) -> diesel::QueryResult<usize> {
        use crate::schema::tags::dsl::*;

        diesel::delete(tags.filter(id.eq(param_id))).execute(db)
    }
}

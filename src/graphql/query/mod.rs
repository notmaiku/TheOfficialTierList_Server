use entity::async_graphql;

pub mod tiers;
pub mod lists;

pub use tiers::TierQuery;
pub use lists::ListQuery;

// Add your other ones here to create a unified Query object
// e.x. Query(NoteQuery, OtherQuery, OtherOtherQuery)
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(TierQuery, ListQuery);

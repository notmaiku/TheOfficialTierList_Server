use entity::async_graphql;

pub mod lists;
pub mod tiers;

pub use lists::ListMutation;
pub use tiers::TiersMutation;



// Add your other ones here to create a unified Mutation object
// e.x. Mutation(NoteMutation, OtherMutation, OtherOtherMutation)
#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(TiersMutation, ListMutation);

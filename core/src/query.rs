use ::entity::{tiers, tiers::Entity as Tier};
use ::entity::{lists, lists::Entity as List};
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_tier_by_id(db: &DbConn, id: i32) -> Result<Option<tiers::Model>, DbErr> {
        Tier::find_by_id(id).one(db).await
    }

     pub async fn find_list_by_id(db: &DbConn, id: i32) -> Result<Option<lists::Model>, DbErr> {
        List::find_by_id(id).one(db).await
    }

    pub async fn get_all_lists(db: &DbConn, user_id: String) -> Result<Vec<lists::Model>, DbErr> {
        List::find().filter(Condition::all().add(lists::Column::UserId.eq(user_id))).all(db).await
    }

    pub async fn get_all_tiers(db: &DbConn) -> Result<Vec<tiers::Model>, DbErr> {
        Tier::find().all(db).await
    }

    pub async fn get_tiers_by_list(
        db: &DbConn,
        list_id: i32,
    ) -> Result<Vec<tiers::Model>, DbErr> {
        Tier::find().filter(Condition::all().add(tiers::Column::ListId.eq(list_id))).all(db).await
    }

    /// If ok, returns (tier models, num pages).
    pub async fn find_tiers_in_page(
        db: &DbConn,
        page: u64,
        tiers_per_page: u64,
    ) -> Result<(Vec<tiers::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = Tier::find()
            .order_by_asc(tiers::Column::Id)
            .paginate(db, tiers_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated tiers
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}

mod hello_world;
mod create_tier;
mod get_tiers;
mod delete_tier;
mod update_tier;
mod create_list;
mod get_lists;
mod delete_list;
mod update_list;

use sea_orm::{DatabaseConnection};
use axum::{
    response::Html,
    routing::{get, post, put, delete},
    Router, Extension, 
};
use crate::database::tiers;
use dotenvy::dotenv;
use http::{Request, Response, Method, header, HeaderValue};
use tower_http::cors::{Any, CorsLayer};
use hello_world::hello_world;
use create_tier::{create_tier, create_multiple_tiers};
use create_list::create_list;
use get_tiers::get_one_tier;
use get_tiers::get_all_tiers;
use get_lists::get_one_list;
use get_lists::get_list_by_name_n_game;
use get_lists::get_users_list;
use get_lists::get_list_rows;
use update_list::update_list;
use delete_list::delete_list;
use delete_tier::delete_tier;
use update_tier::atomic_update;
use update_tier::update_multiple_tiers;

pub fn create_routes(database: DatabaseConnection) -> Router{

    Router::new().route("/", get(hello_world))
        .route("/tiers/one", post(create_tier))
        .route("/tiers/multiple", post(create_multiple_tiers))
        .route("/tiers/list/:list_id", get(get_all_tiers))
        .route("/tiers/list", get(get_all_tiers))
        .route("/tiers/:tier_id", get(get_one_tier))
        .route("/tiers/:tier_id", put(atomic_update))
        .route("/tiers/:tier_id", delete(delete_tier))
        .route("/tiers/update",put(update_multiple_tiers))
        .route("/lists/user/:user_id", get(get_users_list))
        .route("/lists/:list_id", get(get_one_list))
        .route("/lists/name/:title/game/:game", get(get_list_by_name_n_game))
        .route("/lists/:list_id", delete(delete_list))
        .route("/lists/:list_id", put(update_list))
        .route("/lists", post(create_list))
        .route("/rows", get(get_list_rows))
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST, Method::PUT])
                .allow_headers(Any)
                .allow_origin(Any),
        )
        .layer(Extension(database))
}
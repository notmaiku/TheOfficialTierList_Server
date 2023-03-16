mod database;
mod routes;

use sea_orm::Database;
use sea_orm::ConnectOptions;
use tokio::time::Duration;
use std::net::SocketAddr;

pub async fn run(database_uri: &str){

    let database_url = database_uri.to_owned();
    let db = match Database::connect(database_url).await {
        Ok(db) => db,
        Err(error) => {
            eprintln!("Error connecting to the database: {:?}", error);
            panic!();
        }
    };
    let app = routes::create_routes(db);
    let addr = SocketAddr::from(([0,0,0,0], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

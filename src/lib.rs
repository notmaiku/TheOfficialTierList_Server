mod database;
mod routes;

use sea_orm::Database;
use std::net::SocketAddr;
use dotenvy::dotenv;

pub async fn run(database_uri: &str){
    dotenv().ok();
    let database_url = database_uri.to_owned();
    let db = match Database::connect(database_url).await {
        Ok(db) => db,
        Err(error) => {
            eprintln!("Error connecting to the database: {:?}", error);
            panic!();
        }
    };
    let port = dotenvy::var("PORT").unwrap().parse().unwrap();
    let app = routes::create_routes(db);
    let addr = SocketAddr::from(([0,0,0,0], port));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

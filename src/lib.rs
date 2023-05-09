mod db;
mod graphql;

use entity::async_graphql;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
    routing::get,
    Router, http::Method,
};
use http::HeaderValue;
use tower_http::cors::{Any, CorsLayer};
use graphql::schema::{build_schema, AppSchema};

#[cfg(debug_assertions)]
use dotenvy::dotenv;

async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        "/api/graphql",
    )))
}

pub async fn run() {
    #[cfg(debug_assertions)]
    dotenv().ok();

    let schema = build_schema().await;

    let cors = CorsLayer::new()
    // allow `GET` and `POST` when accessing the resource
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    // allow requests from any origin
    .allow_origin("https://theofficialtierlist.com".parse::<HeaderValue>().unwrap());
    // .allow_origin(Any);

    let app = Router::new()
        .route(
            "/api/graphql",
            get(graphql_playground).post(graphql_handler),
        )
        .layer(Extension(schema))
        .layer(cors)
        ;

    println!("Playground: http://localhost:3000/api/graphql");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

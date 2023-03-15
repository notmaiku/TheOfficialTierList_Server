use dotenvy::dotenv;
use totl_backend::run;

#[tokio::main]
async fn main() {
    dotenv().ok();
    run("postgres://postgres:postgres@ip-172-31-1-69/postgres").await;
}
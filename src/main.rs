use cedar_authorizer::routes::api_error::ApiError;
pub mod dto;
pub mod routes;
mod server;
pub mod utils;

#[actix_web::main]
async fn main() -> Result<(), ApiError> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv::dotenv().ok();
    server::server().await
}

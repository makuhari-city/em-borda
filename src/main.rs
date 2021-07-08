use em_borda::{calculate, http_server};
use std::env;

const MODULE_NAME: &str = "/borda";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=trace,em_borda=debug");
    std::env::set_var("VOTE_PORT", "8381");
    env_logger::init();
    http_server(MODULE_NAME).await
}

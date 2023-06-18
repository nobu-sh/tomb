use colored::*;
use git_version::git_version;
use actix_web::{
    web,
    HttpServer,
    App,
};

// Modules
mod config;

const GIT_VERSION: &str = git_version!();

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Setup Logs
    std::env::set_var("RUST_LOG", "actix_web=info,tomb=info");
    env_logger::init();

    println!("{} {}{}", "TOMB".cyan(), "v".purple(), GIT_VERSION.to_string().purple());

    let config = config::Config::from_env();

    log::info!("Starting server on port {}", config.port);

    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(|| async { "Hello world!" }))
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await
}

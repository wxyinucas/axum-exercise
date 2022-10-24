use axum::middleware::from_extractor;
use axum::{Extension, Router};
use sqlx::PgPool;

use blog_system::middleware::Auth;
use blog_system::{BlogError, Config};

#[tokio::main]
async fn main() -> blog_system::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "server=debug,blog_system=debug");
    }
    tracing_subscriber::fmt().pretty().init();

    let config = Config::load().unwrap();
    let pool = PgPool::connect(&config.postgres_config.make_address())
        .await
        .map_err(BlogError::from)?;

    // let frontend_routers = blog_system::frontend::router();
    let backend_routers = blog_system::backend::router().layer(from_extractor::<Auth>());
    let app = Router::new()
        // .nest("/", frontend_routers)
        .nest("/admin", backend_routers)
        .layer(Extension(pool));

    let addr = &config.web_config.addr;
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

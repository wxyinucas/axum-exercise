use axum::Router;

use blog_system::Config;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "blog_system=debug");
    }
    tracing_subscriber::fmt::init();

    let config = Config::load().unwrap();
    let frontend_routers = blog_system::frontend::router();
    let backend_routers = blog_system::backend::router();

    let app = Router::new()
        .nest("/", frontend_routers)
        .nest("/admin", backend_routers);

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

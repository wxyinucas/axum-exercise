use askama::Template;
use axum::response::Html;
use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    name: String,
}

async fn index() -> Result<Html<String>, String> {
    let name = "rex_wang".to_owned();
    let tpl = IndexTemplate { name };
    let html = tpl.render().map_err(|e| e.to_string())?;
    Ok(Html(html))
}

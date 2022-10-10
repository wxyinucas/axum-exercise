use axum::extract::{FromRequest, RequestParts};
use axum::http::StatusCode;
use axum::middleware::from_extractor;
use axum::routing::get;
use axum::{async_trait, Router};
use tower_http::trace::TraceLayer;
use tracing::debug;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "basic_middleware=debug,tower_http=debug");
        // std::env::set_var("RUST_LOG", "debug");
    }
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/foo", get(foo))
        .route("/bar", get(bar))
        .layer(TraceLayer::new_for_http())
        .layer(from_extractor::<UserAgentInfo>());

    debug!("serve ready!");
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn foo() -> &'static str {
    "Welcome to axum.rs"
}

async fn bar() -> &'static str {
    "Powered by axum.rs"
}

struct UserAgentInfo;

#[async_trait]
impl<B> FromRequest<B> for UserAgentInfo
where
    B: Send,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let user_agent = req
            .headers()
            .get(axum::http::header::USER_AGENT)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("");
        tracing::debug!("The agent is {}", user_agent);
        if !user_agent.contains("Firefox") {
            tracing::error!("非Firefox浏览器，禁止访问");
            return Err((
                StatusCode::BAD_REQUEST,
                "You MUST use Firefox to visit this page.".to_string(),
            ));
        }
        Ok(UserAgentInfo {})
    }
}

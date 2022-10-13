use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::{routing, Json, Router, TypedHeader};
use jsonwebtoken as jwt;
use serde::{Deserialize, Serialize};
use tracing::info;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "with_jwt=debug,tower_http=debug");
    }
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/authorize", routing::post(authorize))
        .route("/protected", routing::get(protected));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

const JWT_SECRET: &str = "AXUM.RS";

// TODO: 弄清楚这个流程究竟都做了什么，怎么做的。
struct Keys {
    encoding: jwt::EncodingKey,
    decoding: jwt::DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: jwt::EncodingKey::from_secret(secret),
            decoding: jwt::DecodingKey::from_secret(secret),
        }
    }

    fn global() -> Self {
        Self::new(JWT_SECRET.as_bytes())
    }
}

#[derive(Serialize, Deserialize)]
struct Claims {
    // put in token, which is in AuthBody
    sub: String,
    company: String,
    exp: usize,
}

#[derive(Deserialize)]
struct AuthPayload {
    // used in extractor in authorize()
    client_id: String,
    client_secret: String,
}

#[derive(Serialize)]
struct AuthBody {
    // used as output in authorize()
    access_token: String,
    token_type: String,
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

async fn authorize(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, Json<String>> {
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(Json("Missing Credentials".to_string()));
    }

    if payload.client_id != "rex_wang" || payload.client_secret != "axum.rs" {
        return Err(Json("Wrong Credentials".to_string()));
    }

    let claims = Claims {
        sub: "team@axum.rs".to_string(),
        company: "axum.rs".to_string(),
        exp: 100000000000000000,
    };

    let token = jwt::encode(&jwt::Header::default(), &claims, &Keys::global().encoding)
        .map_err(|err| Json(err.to_string()))?;

    Ok(Json(AuthBody::new(token)))
}

async fn protected(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> Result<String, String> {
    info!("{:?}", bearer.token());
    let token_data = jwt::decode::<Claims>(
        bearer.token(),
        &Keys::global().decoding,
        &jwt::Validation::default(),
    )
    .map_err(|err| format!("invalid token: {}", err.to_string()))?;

    let claims = token_data.claims;
    Ok(format!(
        "Welcome, your email {}, company {}!",
        claims.sub, claims.company
    ))
}

use axum::extract::Path;
use axum::routing::get;
use axum::{Extension, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::pool::PoolConnection;
use sqlx::{Executor, Pool, Postgres};

#[tokio::main]
async fn main() {
    // TODO: Pool 的正确用法？
    let pool = Pool::<Postgres>::connect("postgresql://localhost:5432/new_db")
        .await
        .unwrap();

    let app = Router::new()
        .route("/", get(list))
        .route("/insert/:username", get(insert))
        .route("/update/:id/:balance", get(update))
        .route("/transfer/:form_id/:to_id/:balance", get(transfer))
        .layer(Extension(pool));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn _get_client(
    Extension(pool): Extension<Pool<Postgres>>,
) -> Result<PoolConnection<Postgres>, String> {
    let client = pool.acquire().await.map_err(|_| "get client wrong")?;
    Ok(client)
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
struct UserInfo {
    id: i32,
    username: String,
    balance: i32,
}

struct NewUser {
    username: String,
    balance: i32,
}

async fn insert(
    Path(username): Path<String>,
    Extension(pool): Extension<Pool<Postgres>>,
) -> Result<&'static str, String> {
    // 创建新用户
    let new_user = NewUser {
        username,
        balance: 0,
    };

    let sql = "INSERT INTO account (username, balance) VALUES ($1, $2)";
    let count = sqlx::query(sql)
        .bind(new_user.username)
        .bind(new_user.balance)
        .execute(&pool)
        .await
        .map_err(|_e| "Insert query wrong".to_string())?;
    if count.rows_affected() < 1 {
        return Err("Insert failed".to_string());
    }
    Ok("Successfully inserted")
}

async fn update(
    Path((id, balance)): Path<(i32, i32)>,
    Extension(pool): Extension<Pool<Postgres>>,
) -> Result<&'static str, String> {
    let sql = "UPDATE account SET balance = $2 WHERE id = $1";
    let count = sqlx::query(sql)
        .bind(id)
        .bind(balance)
        .execute(&pool)
        .await
        .map_err(|_| "Update query failed")?;

    if count.rows_affected() != 1 {
        return Err("Update failed".to_string());
    }

    Ok("Successfully updated")
}

async fn list(Extension(pool): Extension<Pool<Postgres>>) -> Result<Json<Vec<UserInfo>>, String> {
    // 注意此处如何处理多个返回值的方法fetch_all()。
    let sql = "SELECT id,username,balance FROM account ORDER BY id DESC";
    let result = sqlx::query_as::<Postgres, UserInfo>(sql)
        .fetch_all(&pool)
        .await
        .map_err(|_| "List query failed")?;

    Ok(Json(result))
}

async fn transfer(
    Path((from_id, to_id, balance)): Path<(i32, i32, i32)>,
    Extension(pool): Extension<Pool<Postgres>>,
) -> Result<String, String> {
    let mut tx = pool.begin().await.map_err(|_| "transfer start failed")?;

    let sql = " UPDATE account SET balance=balance-$1 WHERE id=$2 and balance>$1";
    let query = sqlx::query(sql).bind(balance).bind(from_id);
    let count = tx
        .execute(query)
        .await
        .map_err(|_| "update from_id query failed".to_string())?;

    if count.rows_affected() < 1 {
        tx.rollback().await.map_err(|_| "rollback from_id failed")?;
        return Err("update from_id failed".to_string());
    }

    let sql = " UPDATE account SET balance=balance+$1 WHERE id=$2";
    let query = sqlx::query(sql).bind(balance).bind(to_id);
    let count = tx
        .execute(query)
        .await
        .map_err(|_| "update from_id query failed")?;
    if count.rows_affected() < 1 {
        tx.rollback().await.map_err(|_| "rollback to_id failed")?;
        return Err("update from_id failed".to_string());
    }

    tx.commit().await.map_err(|_| "commit error!".to_string())?;

    Ok("Successfully transferred!".to_string())
}

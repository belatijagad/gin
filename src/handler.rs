use gin::{
    model::WalletModel,
    schema::{CreateWalletSchema, FilterOptions, UpdateWalletSchema},
    AppState,
};
use actix::web::{delete, get, patch, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use serde_json::json;

#[get("/wallets")]
pub async fn wallet_list_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        WalletModel,
        "SELECT * FROM wallets ORDER by id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32,
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let message = "Something bad happened while fetching all wallets.";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error", "message": message}));
    }

    let wallets = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": wallets.len(),
        "wallets": wallets,
    });
    HttpResponse::Ok().json(json_response)
}

#[post("/wallets")]
async fn create_wallet_handler(
    body: web::Json<CreateWalletSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        WalletModel,
        "INSERT INTO wallets (wallet_name, balance) VALUES ($1, $2) RETURNING *",
        body.wallet_name.to_string(),
        body.balance,
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(wallet) => {
            let wallet_response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "wallet": wallet,
            })});

            return HttpResponse::Ok().json(wallet_response);
        }
        Err (e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                return HttpResponse::BadRequest()
                    .json(serde_json::json!({"status": "fail", "message": "Wallet with that name already exist"}));
            }

            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error", "message": format!("{:?}", e)}));
        }
    }
}

#[get("/wallets/{id}")]
async fn get_wallet_handler(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let wallet_id = path.into_inner();
    let query_result = sqlx::query_as!(WalletModel, "SELECT * FROM wallets WHERE id = $1", wallet_id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(wallet) => {
            let wallet_response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "wallet": wallet,
            })});

            return HttpResponse::Ok().json(wallet_response);
        }
        Err(_) => {
            let message = format!("Note with ID: {} not found", wallet_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail", "message": message}));
        }
    }
}

#[patch("/wallets/{id}")]
async fn edit_wallet_handler(
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateWalletSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let wallet_id = path.into_inner();
    let query_result = sqlx::query_as!(WalletModel, "SELECT * FROM wallets WHERE id = $1", wallet_id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let message = format!("Wallet with ID: {} not found", wallet_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }

    let now = Utc::now();
    let wallet = query_result.unwrap();

    let query_result = sqlx::query_as!(
        WalletModel,
        "UPDATE wallets SET wallet_name = $1, balance = $2, updated_at = $3 WHERE id = $4 RETURNING *",
        body.wallet_name.to_owned().unwrap_or(wallet.wallet_name),
        body.balance.to_owned().unwrap_or(wallet.balance),
        now,
        wallet_id,
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(wallet) => {
            let wallet_response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "wallet": wallet
            })});

            return HttpResponse::Ok().json(wallet_response);
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error", "message": message}));
        }
    }
}
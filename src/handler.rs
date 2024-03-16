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
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub page: Option<size>,
    pub limit: Option<size>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateWalletSchema {
    pub wallet_name: String,
    pub balance: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatedWalletSchema {
    pub wallet_name: Option<String>,
    pub balance: Option<i64>,
}
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

// #[derive(Deserialize, Debug)]
// pub struct CreateWalletSchema {
//     pub 
// }
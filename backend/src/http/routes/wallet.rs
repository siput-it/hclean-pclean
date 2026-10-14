use axum::Json;

use crate::{
    core::api_response::{success, ApiSuccess},
    domain::models::wallet::{WalletSummary, WalletTransactions},
};

pub(super) async fn summary() -> Json<ApiSuccess<WalletSummary>> {
    success(WalletSummary {
        balance_rupiah: 0,
        simulated: true,
        is_real_money: false,
        message: "Saldo contoh; bukan dana sungguhan dan tidak dapat digunakan.",
    })
}

pub(super) async fn transactions() -> Json<ApiSuccess<WalletTransactions>> {
    success(WalletTransactions {
        simulated: true,
        transactions: Vec::new(),
    })
}

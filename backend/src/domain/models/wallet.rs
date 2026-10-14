use serde::Serialize;

#[derive(Serialize)]
pub struct WalletSummary {
    pub balance_rupiah: u64,
    pub simulated: bool,
    pub is_real_money: bool,
    pub message: &'static str,
}

#[derive(Serialize)]
pub struct WalletTransactions {
    pub simulated: bool,
    pub transactions: Vec<SimulatedTransaction>,
}

#[derive(Serialize)]
pub struct SimulatedTransaction {
    pub id: &'static str,
    pub amount_rupiah: u64,
    pub transaction_type: &'static str,
}

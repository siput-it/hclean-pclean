pub const CUSTOMER_SERVICE_FEE_PERCENT: u64 = 20;
pub const WORKER_SERVICE_FEE_PERCENT: u64 = 20;

pub fn service_fee_rupiah(service_price_rupiah: u64) -> u64 {
    service_price_rupiah / 5
}

#[cfg(test)]
mod tests {
    use super::{service_fee_rupiah, CUSTOMER_SERVICE_FEE_PERCENT, WORKER_SERVICE_FEE_PERCENT};

    #[test]
    fn service_fees_are_twenty_percent_and_use_integer_rupiah() {
        let service_price_rupiah = 100_000;
        let customer_fee = service_fee_rupiah(service_price_rupiah);
        let worker_fee = service_fee_rupiah(service_price_rupiah);

        assert_eq!(CUSTOMER_SERVICE_FEE_PERCENT, 20);
        assert_eq!(WORKER_SERVICE_FEE_PERCENT, 20);
        assert_eq!(customer_fee, 20_000);
        assert_eq!(worker_fee, 20_000);
        assert_eq!(customer_fee + worker_fee, 40_000);
        assert_eq!(service_fee_rupiah(101), 20);
    }
}

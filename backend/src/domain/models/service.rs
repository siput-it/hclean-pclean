use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
pub struct Service {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub price_rupiah: u64,
    pub price_unit: &'static str,
    pub simulated: bool,
}

pub const SERVICES: [Service; 7] = [
    Service {
        id: "toilet_cleaning",
        name: "Membersihkan toilet",
        description: "Contoh layanan pembersihan toilet.",
        price_rupiah: 50_000,
        price_unit: "per_order",
        simulated: true,
    },
    Service {
        id: "mattress_cleaning",
        name: "Membersihkan kasur",
        description: "Contoh layanan pembersihan kasur.",
        price_rupiah: 75_000,
        price_unit: "per_order",
        simulated: true,
    },
    Service {
        id: "laundry",
        name: "Mencuci pakaian",
        description: "Contoh layanan mencuci pakaian.",
        price_rupiah: 30_000,
        price_unit: "per_order",
        simulated: true,
    },
    Service {
        id: "ironing",
        name: "Menyetrika",
        description: "Contoh layanan menyetrika pakaian.",
        price_rupiah: 25_000,
        price_unit: "per_order",
        simulated: true,
    },
    Service {
        id: "wash_and_iron",
        name: "Cuci dan setrika",
        description: "Contoh layanan mencuci dan menyetrika pakaian.",
        price_rupiah: 45_000,
        price_unit: "per_order",
        simulated: true,
    },
    Service {
        id: "gardening",
        name: "Berkebun",
        description: "Contoh layanan perawatan kebun.",
        price_rupiah: 80_000,
        price_unit: "per_order",
        simulated: true,
    },
    Service {
        id: "home_maintenance",
        name: "Maintenance/perbaikan rumah",
        description: "Contoh layanan maintenance atau perbaikan rumah.",
        price_rupiah: 100_000,
        price_unit: "per_order",
        simulated: true,
    },
];

pub fn find_service(service_id: &str) -> Option<&'static Service> {
    SERVICES.iter().find(|service| service.id == service_id)
}

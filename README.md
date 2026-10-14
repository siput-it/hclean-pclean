# Hclean & Pclean

Fondasi awal untuk dua aplikasi layanan kebersihan dan satu backend bersama.

- `hclean_app`: aplikasi Flutter untuk pengguna jasa (belum dibuat karena Flutter SDK tidak tersedia saat Tahap 1).
- `pclean_app`: aplikasi Flutter untuk pekerja (belum dibuat karena Flutter SDK tidak tersedia saat Tahap 1).
- `backend`: API Rust modular untuk kontrak Hclean dan Pclean.

Backend tetap berjalan dalam mode simulasi tanpa database saat `DATABASE_URL` belum dikonfigurasi. Pada tahap 3, dukungan PostgreSQL sudah disiapkan melalui SQLx dan connection pool yang dibagikan. Backend tidak boleh mengklaim database sehat hanya karena HTTP server sudah hidup. Autentikasi produksi, pembayaran, dan transaksi blockchain tetap ditunda.

## Menjalankan backend

Pastikan Rust stable dan Cargo tersedia, lalu jalankan dari direktori repositori:

```sh
cd backend
cargo run
```

Server secara default mendengarkan `127.0.0.1:3000`. Periksa endpoint dari terminal lain:

```sh
curl http://127.0.0.1:3000/health
```

Respons `GET /health` mencakup status backend dan informasi kesiapan database secara terpisah. Endpoint yang bergantung pada database dapat tetap berfungsi dalam mode simulasi jika database belum terkonfigurasi.

Informasi API tersedia di `GET /api/v1`. Kontrak lengkap, status simulasi, dan contoh request/response ada di [docs/api/KONTRAK_API.md](docs/api/KONTRAK_API.md) dan [docs/api/openapi.yaml](docs/api/openapi.yaml).

Untuk konfigurasi lokal, salin `.env.example` menjadi `.env` di root repositori atau `backend/.env`, lalu ubah nilai sesuai lingkungan. `.env` diabaikan Git. Variabel yang relevan:

```env
BIND_ADDR=127.0.0.1:3000
DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/hclean_pclean
DATABASE_MAX_CONNECTIONS=5
DATABASE_CONNECT_TIMEOUT_SECONDS=5
```

## Migrasi database

SQLx migration disimpan di `backend/migrations/001_init_schema.sql`, `backend/migrations/002_auth_sessions.sql`, dan `backend/migrations/003_addresses.sql`. Migration membuat tabel users, worker_profiles, service_categories, orders, wallet_accounts, wallet_transactions, user_sessions, dan addresses lengkap dengan constraint, indeks, dan seed data awal untuk kategori layanan. Data alamat mengikuti batasan kepemilikan per pengguna dan memastikan hanya satu alamat utama per pengguna.

## Pengujian

```sh
cd backend
cargo fmt --check
cargo test
```

Jika PostgreSQL belum tersedia di lingkungan pengujian, database integration test secara otomatis dilewati dengan pesan jelas. Semua tes non-database tetap dijalankan.

## Dokumentasi

- [Rencana kerja](docs/planning/RENCANA_KERJA.md)
- [Persiapan Termux](docs/setup/PERSIAPAN_TERMUX.md)
- [Arsitektur](docs/architecture/ARSITEKTUR.md)
- [Kontrak API](docs/api/KONTRAK_API.md)

## Kondisi verifikasi

Pada lingkungan saat ini PostgreSQL belum dijalankan, sehingga integrasi database nyata belum diverifikasi end-to-end. Implementasi tetap mencakup konfigurasi, migrasi, repository, dan mode fallback aman agar server tetap berjalan. Tes Rust yang dapat dijalankan pada lingkungan ini sudah dijalankan dengan `cargo test` dari direktori `backend`.
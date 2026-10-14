# Arsitektur

## Komponen Tahap 5

- **Hclean**: aplikasi Flutter untuk pengguna jasa; belum dibuat karena Flutter SDK tidak tersedia.
- **Pclean**: aplikasi Flutter untuk pekerja; belum dibuat karena Flutter SDK tidak tersedia.
- **Backend**: server HTTP Rust berbasis Axum dan Tokio. Modul routes menangani health, versi API, auth, profil pengguna, profil pekerja, alamat, layanan, pesanan, dan wallet.
- **Database**: SQLx + PostgreSQL siap melalui connection pool dan migrations, tetapi tetap dapat berjalan dalam mode simulasi saat `DATABASE_URL` belum diatur.

Kedua aplikasi menggunakan backend bersama. Alamat bind backend diatur melalui `BIND_ADDR` dengan nilai default `127.0.0.1:3000`. Endpoint `GET /health` hanya mengonfirmasi proses backend merespons dan melaporkan kesiapan database secara terpisah. Kontrak endpoint v1 dijelaskan di [KONTRAK_API.md](KONTRAK_API.md).

Respons API sukses menggunakan envelope `success/data`; error menggunakan `success/error`. Katalog layanan dan wallet dapat berasal dari data simulasi atau data PostgreSQL jika konfigurasi database tersedia. Profil pengguna, profil pekerja, dan alamat menyertakan validasi kepemilikan data per pengguna dan melewatkan database jika PostgreSQL belum tersedia dengan respons `503 DATABASE_UNAVAILABLE`.

## Batas keamanan dan lingkup

Tidak ada wallet Kaspa, private key, transaksi on-chain, pemotongan biaya layanan, atau saldo nyata. Angka biaya layanan hanya aturan bisnis dan wallet menandai `simulated: true` bila masih dalam mode simulasi. Integrasi wallet dan ledger internal harus dibuat pada lapisan repositori yang terpisah agar pencatatan saldo tetap auditable dan integrasi jaringan Kaspa tidak menggantikan keseluruhan backend.

Konfigurasi lokal dapat dimuat dari `.env`; berkas tersebut tidak boleh masuk Git. `.env.example` hanya berisi nilai contoh, bukan credential yang dapat digunakan. Variabel utama: `BIND_ADDR`, `DATABASE_URL`, `DATABASE_MAX_CONNECTIONS`, dan `DATABASE_CONNECT_TIMEOUT_SECONDS`.
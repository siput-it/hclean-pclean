# Persiapan Termux

## Backend Rust

Gunakan Termux dari F-Droid atau sumber resmi Termux. Perbarui paket dan pasang toolchain:

```sh
pkg update
pkg upgrade
pkg install git rust curl
```

Pastikan `rustc --version` dan `cargo --version` berhasil. Ambil repositori jika belum ada, kemudian dari root repositori:

```sh
cd backend
cargo fmt --check
cargo test
cargo run
```

Server berjalan di `127.0.0.1:3000` secara default dan tidak memerlukan PostgreSQL. Buka sesi Termux lain untuk menguji:

```sh
curl http://127.0.0.1:3000/health
```

Respons yang diharapkan:

```json
{"status":"ok","service":"hclean-pclean-backend"}
```

Untuk mengubah alamat atau port, atur environment sebelum menjalankan server. Contoh:

```sh
BIND_ADDR=127.0.0.1:8080 cargo run
```

`BIND_ADDR=0.0.0.0:3000` dapat dipakai untuk pengujian dari perangkat lain di jaringan lokal, tetapi batasi akses jaringan dan jangan gunakan konfigurasi tersebut pada jaringan publik tanpa pengamanan.

Jika `cargo fmt` tidak tersedia pada paket Rust yang digunakan, pasang komponen rustfmt sesuai petunjuk toolchain Termux. Jika dependensi Cargo tidak dapat diunduh, periksa koneksi dan sumber paket sebelum mengulang tes.

## PostgreSQL

PostgreSQL belum menjadi prasyarat server Tahap 1 dan tidak ada koneksi yang dibuat. `DATABASE_URL` di `.env.example` hanya placeholder. Jangan mengisi credential sungguhan ke berkas yang dilacak Git; gunakan `backend/.env` lokal yang sudah diabaikan Git saat integrasi database mulai dikerjakan.

## Aplikasi Flutter

Flutter SDK tidak tersedia pada lingkungan Tahap 1 dan proyek Flutter tidak dibuat. Flutter secara resmi menargetkan host pengembangan desktop seperti Linux, macOS, dan Windows; Termux di Android bukan lingkungan host Flutter yang didukung secara resmi. Buat aplikasi menggunakan Flutter CLI pada komputer Linux/macOS/Windows atau lingkungan CI/remote yang mendukung Flutter:

```sh
flutter create --project-name hclean_app hclean_app
flutter create --project-name pclean_app pclean_app
```

Jalankan masing-masing proyek dari direktori aplikasinya dengan `flutter run`. Konfigurasi URL backend sebaiknya diberikan melalui konfigurasi build/runtime, bukan alamat server yang ditanam permanen. Implementasinya akan dibuat bersama kerangka aplikasi pada tahap berikutnya.
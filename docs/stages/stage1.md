# TAHAP 1 — FONDASI PROYEK HCLEAN & PCLEAN

Lanjutkan proyek Hclean dan Pclean secara bertahap, aman, dan terstruktur. Sebelum bekerja, periksa kondisi repositori yang sebenarnya. Repositori dapat kosong atau sudah berisi sebagian implementasi; jangan mengasumsikan salah satunya.

Gunakan bahasa Indonesia untuk penjelasan, dokumentasi, komentar kode yang penting, dan laporan. Identifier kode boleh menggunakan bahasa Inggris sesuai konvensi pemrograman.

## 1. Gambaran Proyek

Proyek terdiri dari dua aplikasi Android dan satu backend bersama:

- Hclean: aplikasi Flutter untuk pelanggan.
- Pclean: aplikasi Flutter untuk pekerja.
- Backend: Rust.
- Database mendatang: PostgreSQL.
- Lingkungan pengembangan awal: Termux di Android.
- Lingkungan produksi mendatang: VPS.
- Wallet Kaspa, jika kelak dibutuhkan, tidak termasuk tahap ini.

## 2. Tujuan Tahap

Fokus pada fondasi yang sederhana, dapat dijalankan, dan mudah dikembangkan:

1. Periksa repositori, sistem operasi, dan toolchain yang tersedia.
2. Susun struktur direktori sesuai kondisi repo dan kebutuhan tahap awal.
3. Buat backend Rust minimal dengan endpoint health.
4. Siapkan konfigurasi database tanpa mewajibkan PostgreSQL untuk menjalankan server dasar.
5. Buat proyek Flutter dengan Flutter CLI resmi hanya jika SDK tersedia.
6. Dokumentasikan instalasi dan pengujian, termasuk instruksi Termux.
7. Jalankan pemeriksaan dan laporkan hasil yang benar-benar diperoleh.

Jangan membuat seluruh sistem bisnis pada tahap ini.

## 3. Struktur Direktori

Gunakan path relatif terhadap root repositori. Struktur fondasi harus mengikuti tata letak modular berikut dan dapat berkembang pada tahap selanjutnya:

```text
hclean-pclean/
├── .env.example
├── .gitignore
├── README.md
├── docs/
│   ├── api/
│   ├── architecture/
│   ├── planning/
│   ├── setup/
│   └── stages/
│       ├── stage1.md
│       └── ...
├── backend/
│   ├── Cargo.toml
│   ├── migrations/
│   ├── src/
│   │   ├── core/
│   │   │   └── config.rs
│   │   ├── http/
│   │   │   └── routes/
│   │   │       ├── mod.rs
│   │   │       └── health.rs
│   │   ├── lib.rs
│   │   └── main.rs
│   └── tests/
│       └── health.rs
├── hclean_app/    # hanya dibuat dengan Flutter CLI resmi
└── pclean_app/    # hanya dibuat dengan Flutter CLI resmi
```

Dokumentasi proyek ditempatkan sesuai fungsi: rencana di `docs/planning/RENCANA_KERJA.md`, panduan setup di `docs/setup/PERSIAPAN_TERMUX.md`, arsitektur di `docs/architecture/ARSITEKTUR.md`, kontrak API di `docs/api/KONTRAK_API.md`, spesifikasi OpenAPI di `docs/api/openapi.yaml`, dan dokumen tahap di `docs/stages/`.

Jika Flutter SDK tidak tersedia, jangan membuat file proyek Flutter palsu. Catat cara membuat aplikasi pada dokumentasi setup.

## 4. Ketentuan Backend Rust

Gunakan Rust stable dan dependensi yang kompatibel. Axum, Tokio, Serde, dan tracing dapat digunakan untuk fondasi HTTP. Siapkan konfigurasi PostgreSQL untuk tahap berikutnya tanpa mengharuskan koneksi database tersedia pada tahap ini.

Backend minimal menyediakan `GET /health` dengan respons JSON yang menyatakan status backend. Jangan mengklaim database sehat jika koneksi database belum diperiksa.

Persyaratan tambahan:

- Alamat bind dan port dapat dikonfigurasi.
- Secret tidak boleh ditulis langsung di source code.
- `.env.example` hanya berisi placeholder, bukan credential sungguhan.
- Pesan error harus jelas dan tidak membocorkan detail internal.
- Struktur route mengikuti `backend/src/http/routes/`; konfigurasi dasar mengikuti `backend/src/core/`.
- Server dapat berjalan tanpa wallet Kaspa atau transaksi on-chain.
- Tambahkan test health jika lingkungan mendukung.

## 5. Ketentuan Flutter

Hclean dan Pclean merupakan dua aplikasi berbeda yang memakai backend yang sama. Jika Flutter SDK tersedia, buat keduanya menggunakan Flutter CLI resmi dan pastikan dapat dijalankan. Halaman awal cukup menampilkan identitas aplikasi masing-masing.

Jangan membuat fitur wallet, pesanan, chat, atau pembayaran pada tahap ini. Jangan menanam alamat backend lokal secara permanen. Splash screen final dapat dikerjakan pada tahap UI berikutnya.

## 6. Batasan Wallet dan Keamanan

Pada tahap ini:

- Tidak ada transaksi Kaspa sungguhan atau integrasi blockchain.
- Tidak ada private key, seed phrase, pemotongan biaya, atau pembayaran.
- Jangan menyajikan saldo atau transaksi simulasi seolah-olah dana sungguhan.
- Jangan menambahkan secret, token, atau credential ke Git.

Arsitektur boleh disiapkan agar dapat dikembangkan kemudian, tetapi jangan membangun fitur tersebut sekarang.

## 7. Urutan Pengerjaan dan Validasi

1. Periksa repositori, OS, versi Rust/Cargo, ketersediaan Flutter/Dart, dan PostgreSQL.
2. Laporkan kondisi awal serta kendala.
3. Buat atau lengkapi fondasi backend dan dokumentasi tanpa menghapus implementasi yang berfungsi.
4. Buat aplikasi Flutter hanya jika toolchain tersedia.
5. Jalankan formatter, pemeriksaan kompilasi, dan test yang tersedia.
6. Uji `GET /health` dengan cara yang terdokumentasi.
7. Periksa konsistensi path dan dokumentasi sebelum melaporkan hasil.

Jika tool atau layanan tidak tersedia, sebutkan pemeriksaan yang tidak dapat dijalankan; jangan mengklaim sukses tanpa hasil nyata.

## 8. Laporan Akhir

Laporkan dalam bahasa Indonesia:

- File dan direktori yang dibuat atau diubah.
- Fungsi tiap komponen dan dependensi yang digunakan.
- Perintah untuk menjalankan backend dan menguji `/health`.
- Hasil pemeriksaan dan test yang benar-benar dijalankan.
- Kendala yang belum terselesaikan.
- Langkah berikutnya untuk Tahap 2.

Kerjakan Tahap 1 saja. Jangan melanjutkan ke tahap berikutnya.

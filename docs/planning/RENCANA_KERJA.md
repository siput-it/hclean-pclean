# Rencana Kerja

## Tahap 1: Fondasi

- Menyediakan backend Rust modular dengan `GET /health`.
- Menyiapkan konfigurasi listener dan placeholder konfigurasi PostgreSQL.
- Mendokumentasikan setup dan pengujian di Termux.
- Menunda kerangka Flutter jika Flutter SDK tidak tersedia.
- Tidak membuat fitur bisnis, wallet, saldo, atau transaksi.

## Tahap 2: Persiapan aplikasi dan data

- Membuat proyek Flutter Hclean dan Pclean dengan Flutter CLI resmi.
- Menetapkan kontrak API dan konfigurasi alamat backend untuk kedua aplikasi.
- Menyiapkan PostgreSQL dan migrasi skema setelah kebutuhan data disepakati.
- Menambahkan autentikasi dan alur pengguna secara terpisah dari wallet.

## Tahap berikutnya

- Mengembangkan ledger internal dengan pencatatan yang dapat diaudit sebelum integrasi pembayaran.
- Mengevaluasi integrasi Kaspa dan pengelolaan kunci secara terpisah, dengan batas keamanan dan proses operasional yang jelas.
- Menjalankan pengujian, pencadangan, dan pengerasan konfigurasi sebelum deployment VPS.

Tidak ada saldo simulasi yang ditampilkan sebagai saldo nyata, private key, atau transaksi on-chain dalam Tahap 1.
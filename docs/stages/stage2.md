TAHAP 2 — KONTRAK API HCLEAN & PCLEAN

Lanjutkan proyek Hclean dan Pclean yang sudah menyelesaikan Tahap 1. Pertahankan backend Rust modular berbasis Axum dan Tokio yang sudah ada.

Tujuan

Bangun fondasi kontrak API untuk dua aplikasi:

- Hclean: aplikasi pelanggan.
- Pclean: aplikasi pekerja.
- Backend: satu layanan Rust yang dipakai kedua aplikasi.

Fokus tahap ini adalah mendefinisikan format permintaan, format respons, model data, validasi dasar, dan dokumentasi API. Jangan membangun seluruh fitur bisnis sekaligus.

1. Pertahankan implementasi yang sudah ada

- Periksa struktur proyek dan gunakan modul yang sudah tersedia.
- Jangan menggandakan endpoint health atau membuat konfigurasi server kedua.
- Jangan menghapus atau menulis ulang kode yang sudah berfungsi tanpa alasan kuat.
- Pertahankan endpoint "GET /health" dan tes yang sudah lulus.
- Ikuti gaya penamaan dan struktur modular yang telah digunakan.

2. Buat struktur modul API

Sesuaikan dengan struktur yang sudah ada. Target modul:

"backend/src/http/routes/"

- "mod.rs"
- "health.rs"
- "auth.rs"
- "users.rs"
- "services.rs"
- "orders.rs"
- "wallet.rs"

Buat modul model dan respons bersama jika diperlukan, misalnya:

- "backend/src/domain/models/"
- "backend/src/core/api_response.rs"

Jangan membuat file duplikat apabila struktur yang sama sudah ada.

3. Standarkan respons API

Gunakan JSON dengan format konsisten.

Respons berhasil harus memiliki data yang jelas. Respons gagal harus menyertakan kode kesalahan dan pesan yang aman untuk pengguna.

Contoh respons berhasil:

```json
{"success":true,"data":{"message":"API ready"}}
```

Contoh respons gagal:

```json
{"success":false,"error":{"code":"VALIDATION_ERROR","message":"Data permintaan tidak valid"}}
```

Sesuaikan endpoint "/health" yang lama hanya jika benar-benar diperlukan; jangan merusak tes yang sudah ada.

Jangan mengirim stack trace, informasi rahasia, private key, seed phrase, atau detail internal server kepada pengguna.

4. Siapkan kontrak endpoint

Rancang dan implementasikan endpoint dasar berikut dengan data simulasi sementara jika dibutuhkan:

Umum

- "GET /health" — status backend.
- "GET /api/v1" — informasi versi API.

Autentikasi

- "POST /api/v1/auth/register" — kontrak pendaftaran akun.
- "POST /api/v1/auth/login" — kontrak masuk.
- "POST /api/v1/auth/logout" — kontrak keluar.

Untuk tahap ini, pendaftaran dan login boleh menggunakan validasi serta respons simulasi yang dinyatakan jelas. Jangan berpura-pura bahwa akun sudah tersimpan secara permanen. Jangan membuat autentikasi palsu yang dianggap aman untuk produksi.

Profil

- "GET /api/v1/users/me" — kontrak profil pengguna saat autentikasi nyata tersedia.
- "PATCH /api/v1/users/me" — kontrak perubahan profil.

Jika autentikasi belum tersedia, kembalikan status yang sesuai dan jelaskan bahwa autentikasi belum diimplementasikan. Jangan menerima identitas pengguna dari request lalu memperlakukannya sebagai bukti autentikasi.

Katalog layanan

- "GET /api/v1/services" — daftar kategori layanan.
- "GET /api/v1/services/{service_id}" — detail layanan.

Siapkan kategori awal:

1. Membersihkan toilet.
2. Membersihkan kasur.
3. Mencuci pakaian.
4. Menyetrika.
5. Cuci dan setrika.
6. Berkebun.
7. Maintenance/perbaikan rumah.

Harga harus diwakili dalam rupiah sebagai bilangan bulat, bukan floating point. Beri label bahwa katalog dan harga masih berupa data simulasi sampai konfigurasi backend/admin dan penyimpanan persisten tersedia.

Pesanan

- "POST /api/v1/orders" — kontrak pembuatan pesanan.
- "GET /api/v1/orders" — kontrak daftar pesanan.
- "GET /api/v1/orders/{order_id}" — kontrak detail pesanan.
- "POST /api/v1/orders/{order_id}/accept" — kontrak pekerja menerima pesanan.
- "POST /api/v1/orders/{order_id}/start" — kontrak memulai pekerjaan.
- "POST /api/v1/orders/{order_id}/complete" — kontrak menyelesaikan pekerjaan.
- "POST /api/v1/orders/{order_id}/cancel" — kontrak pembatalan.

Pada tahap ini, cukup tentukan struktur data dan aturan transisi status. Jangan mengklaim pesanan telah disimpan permanen. Jangan mengizinkan perubahan status yang tidak valid.

Wallet

- "GET /api/v1/wallet" — ringkasan wallet simulasi.
- "GET /api/v1/wallet/transactions" — daftar transaksi simulasi.

Saldo harus diberi penanda eksplisit "simulated: true". Jangan membuat wallet Kaspa nyata, seed phrase, private key, transaksi blockchain, atau mengklaim saldo simulasi sebagai dana sungguhan.

5. Aturan model bisnis yang harus didokumentasikan

Harga jasa menjadi dasar penghitungan biaya layanan:

- Biaya layanan pelanggan = 20% dari harga jasa.
- Biaya layanan pekerja = 20% dari harga jasa.
- Pelanggan membayar harga jasa langsung secara tunai kepada pekerja.
- Biaya layanan kedua pihak direncanakan untuk diproses melalui wallet setelah integrasi wallet yang aman tersedia.
- Biaya jaringan blockchain, jika kelak ada, merupakan komponen terpisah.

Contoh untuk harga jasa Rp100.000:

- Biaya layanan pelanggan: Rp20.000.
- Biaya layanan pekerja: Rp20.000.
- Total biaya layanan platform: Rp40.000.
- Pembayaran jasa tunai kepada pekerja: Rp100.000.

Semua perhitungan uang menggunakan integer rupiah. Untuk sementara, angka ini hanya aturan bisnis yang terdokumentasi; jangan memotong saldo atau melakukan pembayaran nyata.

6. Dokumentasi API

Buat "docs/api/KONTRAK_API.md" yang menjelaskan:

- Daftar endpoint dan metode HTTP.
- Tujuan setiap endpoint.
- Bentuk request dan response JSON.
- Kode status HTTP.
- Format kesalahan.
- Model data utama.
- Status simulasi dan batasan tahap ini.
- Alur pelanggan dan pekerja.
- Rumus biaya layanan 20% masing-masing pihak.

Jika memungkinkan, tambahkan spesifikasi OpenAPI di "docs/api/openapi.yaml" agar dokumentasi nantinya dapat digunakan untuk menguji API dan menghasilkan klien. Jangan membuat spesifikasi yang tidak sesuai dengan implementasi.

7. Pengujian

Tambahkan tes untuk:

- "GET /health" tetap berfungsi.
- Informasi versi API.
- Format respons JSON.
- Katalog layanan dan kategori awal.
- Validasi input dasar.
- Status HTTP untuk endpoint yang belum didukung.
- Perhitungan biaya layanan dalam unit test, tanpa transaksi nyata.

Jalankan "cargo fmt --check", "cargo check", dan "cargo test" jika lingkungan mendukungnya. Laporkan hasil yang benar-benar diperoleh.

8. Batasan wajib

Jangan mengimplementasikan PostgreSQL atau SQLx pada tahap ini. Jangan membuat integrasi wallet Kaspa sungguhan, autentikasi produksi, transfer dana, pembayaran, atau transaksi blockchain. Jangan menambahkan saldo nyata atau menyimpan kredensial rahasia.

Jika ada endpoint yang belum bisa diterapkan dengan aman, dokumentasikan kontraknya dan kembalikan status yang sesuai daripada berpura-pura fiturnya sudah berjalan.

9. Laporan akhir

Setelah selesai, jelaskan dalam bahasa Indonesia:

1. Berkas yang dibuat atau diubah.
2. Endpoint yang sudah berfungsi.
3. Endpoint yang baru berupa kontrak.
4. Bentuk data request dan response yang penting.
5. Hasil pemeriksaan dan tes yang benar-benar dijalankan.
6. Batasan yang masih ada.
7. Tahap berikutnya yang direkomendasikan.

Kerjakan Tahap 2 saja. Jangan langsung melanjutkan ke Tahap 3.
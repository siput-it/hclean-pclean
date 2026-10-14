# Kontrak API Hclean & Pclean

Versi dasar API adalah `v1`. Kedua aplikasi menggunakan backend yang sama. Semua respons endpoint menggunakan JSON. Respons sukses berbentuk `{"success":true,"data":{...}}`; respons gagal berbentuk `{"success":false,"error":{"code":"...","message":"..."}}`. Pesan error aman untuk pengguna dan tidak membocorkan stack trace atau detail internal.

Spesifikasi OpenAPI tersedia di [openapi.yaml](openapi.yaml).

## Endpoint

| Metode | Endpoint | Tujuan | Status saat ini |
| --- | --- | --- | --- |
| GET | `/health` | Memeriksa proses backend | Berfungsi; tidak memeriksa database |
| GET | `/api/v1` | Informasi versi API | Berfungsi |
| POST | `/api/v1/auth/register` | Kontrak pendaftaran | Validasi dasar; data tidak disimpan, respons `501` |
| POST | `/api/v1/auth/login` | Kontrak login | Validasi dasar; kredensial tidak diverifikasi, respons `501` |
| POST | `/api/v1/auth/logout` | Kontrak logout | Autentikasi belum ada, respons `501` |
| GET | `/api/v1/users/me` | Membaca profil pengguna terautentikasi | Berfungsi dengan autentikasi jika database tersedia; tanpa token `401` |
| PATCH | `/api/v1/users/me` | Mengubah profil pengguna terautentikasi | Berfungsi dengan validasi nama/telepon dan ownership check |
| GET | `/api/v1/workers/me` | Membaca profil pekerja terautentikasi | Hanya untuk role `worker`; tanpa token `401`; tanpa role `403` |
| PATCH | `/api/v1/workers/me` | Mengubah bio dan status ketersediaan pekerja | Hanya untuk role `worker` |
| GET | `/api/v1/addresses` | Daftar alamat milik pengguna | Menyediakan data dari PostgreSQL bila tersedia, tanpa DB `503` |
| POST | `/api/v1/addresses` | Menambah alamat pengguna | Validasi label, nomor telepon, alamat, dan koordinat |
| GET | `/api/v1/addresses/{address_id}` | Detail alamat milik pengguna | Ownership check di query |
| PATCH | `/api/v1/addresses/{address_id}` | Mengubah alamat pengguna | Menjaga satu alamat utama per pengguna |
| DELETE | `/api/v1/addresses/{address_id}` | Menghapus alamat milik pengguna | Hanya alamat milik pengguna yang bisa dihapus |
| GET | `/api/v1/services` | Daftar kategori layanan | Berfungsi memakai katalog simulasi atau PostgreSQL |
| GET | `/api/v1/services/{service_id}` | Detail kategori layanan | Berfungsi memakai katalog simulasi |
| POST | `/api/v1/orders` | Kontrak pembuatan pesanan | Validasi dasar; pesanan tidak disimpan, respons `501` |
| GET | `/api/v1/orders` | Kontrak daftar pesanan | Belum tersedia, respons `501` |
| GET | `/api/v1/orders/{order_id}` | Kontrak detail pesanan | Belum tersedia, respons `501` |
| POST | `/api/v1/orders/{order_id}/accept` | Pekerja menerima pesanan | Belum tersedia, tidak mengubah status, respons `501` |
| POST | `/api/v1/orders/{order_id}/start` | Memulai pekerjaan | Belum tersedia, tidak mengubah status, respons `501` |
| POST | `/api/v1/orders/{order_id}/complete` | Menyelesaikan pekerjaan | Belum tersedia, tidak mengubah status, respons `501` |
| POST | `/api/v1/orders/{order_id}/cancel` | Membatalkan pesanan | Belum tersedia, tidak mengubah status, respons `501` |
| GET | `/api/v1/wallet` | Ringkasan wallet simulasi | Berfungsi; bukan dana sungguhan |
| GET | `/api/v1/wallet/transactions` | Daftar transaksi wallet simulasi | Berfungsi; daftar kosong |

## Bentuk data

### Informasi API dan health

`GET /api/v1` mengembalikan `data.version` (`v1`), `data.service`, dan `data.message` (`API ready`). Health menambahkan `data.status` (`ok`) dan `data.service`; tidak ada klaim status database.

### Pendaftaran dan login

Request pendaftaran:

```json
{
  "name": "Nama pengguna",
  "email": "user@example.test",
  "password": "minimal-delapan-karakter",
  "role": "customer"
}
```

`role` bernilai `customer` atau `worker`. Nama harus berisi 1-100 karakter, email melewati pemeriksaan format dasar, dan password minimal 8 karakter. Login menerima `email` dan `password`. Input valid belum membuat akun atau sesi; kedua endpoint membalas `501 AUTHENTICATION_NOT_IMPLEMENTED`. Password tidak dikembalikan atau dicatat.

### Profil

PATCH menerima minimal satu dari `name` (1-100 karakter) dan `phone_number` (1-32 karakter). Identitas tidak diterima dari request sebagai bukti autentikasi. Kedua endpoint profil membalas `501 AUTHENTICATION_NOT_IMPLEMENTED` sampai autentikasi yang nyata tersedia.

### Layanan

`GET /api/v1/services` mengembalikan `data.simulated: true` dan `data.items`. Setiap item memiliki `id`, `name`, `description`, `price_rupiah` (integer), `price_unit`, dan `simulated: true`.

Kategori yang tersedia: `toilet_cleaning`, `mattress_cleaning`, `laundry`, `ironing`, `wash_and_iron`, `gardening`, dan `home_maintenance`. Harga adalah angka contoh per pesanan, bukan penawaran harga produksi. ID layanan tidak dikenal menghasilkan `404 SERVICE_NOT_FOUND`.

### Pesanan

Request pembuatan pesanan menerima `service_id`, `scheduled_at`, `address`, dan `notes` opsional. `service_id` harus ada di katalog simulasi, alamat dan waktu harus berisi nilai, dan panjang alamat/waktu/catatan dibatasi. Identitas pelanggan tidak dapat ditentukan dari body. Request valid saat ini menghasilkan `501 ORDER_WORKFLOW_NOT_IMPLEMENTED`; tidak ada pesanan yang disimpan.

Status model: `pending`, `accepted`, `in_progress`, `completed`, dan `cancelled`. Transisi yang direncanakan: `pending -> accepted` atau `cancelled`; `accepted -> in_progress` atau `cancelled`; `in_progress -> completed` atau `cancelled`. `completed` dan `cancelled` terminal. Fungsi model dan unit test menolak transisi lain. Endpoint aksi saat ini tidak melakukan transisi apa pun karena belum ada penyimpanan dan otorisasi.

Model order yang disiapkan berisi `id`, `service_id`, `status`, `scheduled_at`, `address`, `price_rupiah`, `customer_service_fee_rupiah`, `worker_service_fee_rupiah`, dan `simulated`. Ini model internal kontrak, bukan respons endpoint saat ini; tidak ada order yang dibuat atau dikembalikan.

### Wallet

Ringkasan berisi `balance_rupiah: 0`, `simulated: true`, dan `is_real_money: false`. Riwayat berisi `simulated: true` dan array transaksi kosong. Tidak ada saldo, ledger, seed phrase, private key, transfer, atau transaksi Kaspa sungguhan.

## Status HTTP dan error

- `200 OK`: request baca berhasil, termasuk health, versi, katalog, dan wallet simulasi.
- `400 Bad Request`: JSON tidak valid secara bentuk atau gagal validasi; kode `VALIDATION_ERROR`.
- `404 Not Found`: endpoint/kategori tidak ditemukan; kode `NOT_FOUND` atau `SERVICE_NOT_FOUND`.
- `405 Method Not Allowed`: metode tidak tersedia; kode `METHOD_NOT_ALLOWED`.
- `501 Not Implemented`: alur yang belum aman/belum tersedia, dengan kode `AUTHENTICATION_NOT_IMPLEMENTED` atau `ORDER_WORKFLOW_NOT_IMPLEMENTED`.

Contoh error:

```json
{
  "success": false,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Data permintaan tidak valid"
  }
}
```

## Alur dan biaya layanan

Rancangan alur: pelanggan memilih layanan dan mengajukan waktu/alamat; pekerja menerima pesanan; pekerjaan dimulai lalu diselesaikan. Autentikasi nyata, penyimpanan, otorisasi per peran, dan mutasi pesanan belum tersedia, sehingga alur tersebut belum dieksekusi API.

Untuk aturan bisnis mendatang, biaya layanan pelanggan adalah 20% dari harga jasa dan biaya layanan pekerja juga 20%. Pelanggan membayar harga jasa langsung secara tunai kepada pekerja. Kedua biaya layanan direncanakan diproses melalui wallet hanya setelah integrasi yang aman tersedia. Biaya jaringan blockchain, bila ada, merupakan komponen terpisah. Tidak ada pemotongan atau pembayaran yang dilakukan saat ini.

Contoh harga jasa Rp100.000: biaya pelanggan Rp20.000, biaya pekerja Rp20.000, total biaya platform Rp40.000, dan pembayaran tunai jasa kepada pekerja Rp100.000. Perhitungan memakai integer rupiah; pembagian 20% membulatkan ke bawah untuk nilai yang tidak habis dibagi lima.
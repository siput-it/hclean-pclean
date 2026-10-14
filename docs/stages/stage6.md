TAHAP 6 — WORKFLOW PESANAN HCLEAN & PCLEAN

Lanjutkan repositori Hclean dan Pclean yang telah menyelesaikan Tahap 1 sampai Tahap 5.

Fokus hanya pada backend Rust dan PostgreSQL. Jangan membuat aplikasi Flutter.

Pertahankan arsitektur Axum, Tokio, SQLx, autentikasi, otorisasi, envelope JSON, model, repository, migrasi, dan endpoint yang sudah ada. Jangan menulis ulang proyek dari awal.

1. Tujuan

Implementasikan workflow pesanan yang nyata dan persisten:

1. Pelanggan membuat pesanan.
2. Pelanggan melihat daftar dan detail pesanan miliknya.
3. Pekerja melihat pesanan yang tersedia.
4. Pekerja menerima pesanan.
5. Pekerja memulai pekerjaan.
6. Pekerja menandai pekerjaan selesai.
7. Pelanggan atau pekerja dapat meminta pembatalan sesuai aturan bisnis.

Semua operasi harus memakai PostgreSQL. Jika database tidak tersedia, endpoint terkait mengembalikan "503 DATABASE_UNAVAILABLE". Jangan menggunakan penyimpanan memori sebagai fallback otomatis.

2. Endpoint

Implementasikan atau sempurnakan endpoint berikut dengan format envelope JSON yang sudah ada:

Pesanan pelanggan

- "POST /api/v1/orders"
- "GET /api/v1/orders"
- "GET /api/v1/orders/{order_id}"
- "POST /api/v1/orders/{order_id}/cancel"

Pesanan pekerja

- "GET /api/v1/worker/orders/available"
- "GET /api/v1/worker/orders"
- "GET /api/v1/worker/orders/{order_id}"
- "POST /api/v1/orders/{order_id}/accept"
- "POST /api/v1/orders/{order_id}/start"
- "POST /api/v1/orders/{order_id}/complete"
- "POST /api/v1/orders/{order_id}/cancel"

Jika route yang sudah ada berbeda, pertahankan kompatibilitas dan dokumentasikan route kanonis yang dipilih. Jangan membuat route duplikat tanpa alasan.

Semua endpoint harus memerlukan autentikasi yang sesuai. Gunakan identitas pengguna terverifikasi dari middleware, bukan "customer_id" atau "worker_id" yang dikirim klien.

3. Pembuatan pesanan

Request pembuatan pesanan harus memuat data yang diperlukan, misalnya:

- "service_category_id"
- "address_id"
- "notes" opsional
- waktu layanan jika penjadwalan sudah didukung

Backend wajib:

- Memastikan pengguna memiliki peran "customer".
- Memastikan kategori layanan tersedia dan aktif.
- Memastikan alamat benar-benar milik pelanggan yang sedang login.
- Mengambil harga aktif dari PostgreSQL.
- Menghitung biaya layanan pelanggan dan pekerja di backend.
- Menyimpan snapshot harga serta biaya saat pesanan dibuat.
- Membuat ID pesanan di backend.
- Menyimpan pesanan menggunakan transaksi PostgreSQL jika ada beberapa operasi terkait.

Jangan menerima harga, biaya layanan, status awal, atau identitas pelanggan sebagai nilai yang dipercaya dari klien.

Status awal pesanan adalah "pending".

4. Rumus biaya layanan

Pertahankan aturan:

- "customer_fee_idr = 20% × service_price_idr"
- "worker_fee_idr = 20% × service_price_idr"

Gunakan integer rupiah dan kebijakan pembulatan yang konsisten. Gunakan aritmetika aman untuk menghindari overflow.

Contoh harga jasa Rp100.000:

- Biaya pelanggan Rp20.000.
- Biaya pekerja Rp20.000.
- Total biaya layanan platform Rp40.000.
- Pembayaran jasa tunai kepada pekerja Rp100.000.

Biaya ini hanya dicatat sebagai kewajiban atau perhitungan bisnis. Jangan mengurangi saldo wallet, memindahkan dana, atau mengklaim biaya layanan telah dibayar.

5. Transisi status pesanan

Gunakan status yang eksplisit dan konsisten:

- "pending"
- "accepted"
- "in_progress"
- "completed"
- "cancelled"

Transisi yang diizinkan:

- "pending → accepted": hanya pekerja yang memenuhi syarat.
- "accepted → in_progress": hanya pekerja yang menerima pesanan tersebut.
- "in_progress → completed": hanya pekerja yang ditugaskan.
- "pending → cancelled": pelanggan pemilik pesanan dapat membatalkan.
- "accepted → cancelled": hanya jika aturan pembatalan yang terdokumentasi mengizinkan.
- Pembatalan setelah pekerjaan dimulai harus mengikuti kebijakan eksplisit; jangan mengizinkannya secara otomatis tanpa aturan.

Status terminal "completed" dan "cancelled" tidak boleh diubah kembali melalui endpoint biasa.

Validasi semua transisi di backend.

6. Pencegahan penerimaan ganda dan konkurensi

Dua pekerja mungkin mencoba menerima pesanan yang sama secara bersamaan.

Gunakan transaksi PostgreSQL dan mekanisme konkurensi yang tepat, seperti row locking atau conditional update, untuk memastikan hanya satu pekerja yang berhasil menerima pesanan.

Jangan mengandalkan pemeriksaan status di memori sebelum update database.

Jika pesanan sudah diterima pekerja lain, kembalikan respons konflik yang konsisten, misalnya "409 ORDER_ALREADY_ACCEPTED".

Pastikan perubahan status dan identitas pekerja ditulis secara atomik.

7. Daftar dan detail pesanan

Pelanggan hanya dapat membaca pesanan miliknya.

Pekerja hanya dapat membaca:

- Pesanan tersedia yang memang boleh ditampilkan.
- Pesanan yang ditugaskan kepadanya.
- Detail yang diperlukan untuk mengerjakan pesanan sesuai status dan kebijakan privasi.

Jangan mengungkapkan alamat lengkap atau nomor telepon pelanggan kepada semua pekerja yang belum menerima pesanan. Batasi informasi pribadi sesuai kebutuhan dan status pekerjaan.

Tambahkan pagination dengan batas maksimum yang wajar. Urutkan hasil berdasarkan waktu pembuatan secara konsisten.

8. Pembatalan

Implementasikan pembatalan sesuai aturan:

- Pastikan pengguna terautentikasi dan memiliki izin terhadap pesanan.
- Validasi status pesanan saat pembatalan.
- Simpan status dan waktu pembatalan jika kolom tersedia.
- Catat alasan pembatalan opsional dengan batas panjang yang wajar.
- Gunakan transaksi database jika beberapa perubahan diperlukan.
- Kembalikan konflik jika pesanan tidak dapat dibatalkan pada statusnya saat ini.

Jangan melakukan pengembalian dana atau perubahan saldo karena belum ada pembayaran sungguhan.

9. Skema database

Periksa apakah tabel "orders" sudah tersedia dari migrasi sebelumnya.

Tambahkan file migrasi baru berurutan di "backend/migrations/" jika diperlukan untuk:

- Status dan constraint status pesanan.
- Waktu penerimaan, mulai, selesai, dan pembatalan.
- Alamat atau snapshot informasi layanan yang memang diperlukan.
- Indeks untuk pencarian pesanan pelanggan, pekerja, dan pesanan pending.
- Relasi serta constraint foreign key.

Jangan mengubah migrasi lama yang sudah diterapkan secara destruktif; gunakan file migrasi baru di "backend/migrations/".

Snapshot harga dan biaya pesanan tidak boleh berubah jika harga kategori layanan berubah kemudian.

Gunakan transaksi SQLx untuk perubahan yang melibatkan lebih dari satu operasi database.

10. Respons dan kesalahan

Pertahankan envelope JSON yang sudah ada.

Gunakan status HTTP yang sesuai:

- "201" untuk pesanan berhasil dibuat.
- "200" untuk pembacaan atau perubahan status berhasil.
- "400" untuk input tidak valid.
- "401" untuk tidak terautentikasi.
- "403" untuk peran atau izin yang tidak sesuai.
- "404" untuk pesanan yang tidak ditemukan atau tidak dapat diakses sesuai kebijakan privasi.
- "409" untuk konflik status atau penerimaan ganda.
- "503" ketika PostgreSQL tidak tersedia.

Jangan membocorkan detail query SQL atau stack trace.

11. Wallet placeholder

Wallet tetap menggunakan "simulated: true".

Belum ada pemotongan biaya layanan, penahanan saldo nyata, deposit, penarikan, private key, seed phrase, atau transaksi Kaspa.

Jangan mengklaim biaya layanan sudah dibayar hanya karena pesanan dibuat, diterima, dimulai, atau selesai.

Jika nantinya dibutuhkan status pembayaran, pisahkan status pembayaran dari status pekerjaan. Pada tahap ini, dokumentasikan saja rancangan yang diperlukan; jangan membuat status pembayaran palsu.

12. Pengujian

Tambahkan tes untuk:

- Pembuatan pesanan valid.
- Kategori tidak aktif atau tidak ditemukan.
- Alamat milik pengguna lain.
- Pengguna dengan peran yang salah.
- Pembacaan pesanan oleh pelanggan lain.
- Pekerja melihat pesanan tersedia.
- Penerimaan pesanan oleh satu pekerja.
- Dua pekerja mencoba menerima pesanan yang sama secara bersamaan.
- Transisi status yang valid dan tidak valid.
- Pembatalan berdasarkan status dan kepemilikan.
- Harga dan biaya layanan tetap menjadi snapshot.
- Database tidak tersedia.
- Tidak ada saldo yang dipotong atau pembayaran palsu.

Tes integrasi harus menggunakan PostgreSQL khusus pengujian. Jika PostgreSQL tidak tersedia, jangan mengklaim tes integrasi database lulus; tandai sebagai belum dijalankan.

Pertahankan tes tahap sebelumnya.

13. Dokumentasi

Perbarui:

- "docs/api/KONTRAK_API.md"
- "docs/api/openapi.yaml"
- "docs/architecture/ARSITEKTUR.md"
- "README.md"

Dokumentasikan request dan response, transisi status, aturan kepemilikan, rumus biaya layanan, kebijakan pembatalan, dan batasan wallet simulasi.

Pastikan OpenAPI sesuai dengan route yang benar-benar tersedia.

Batasan wajib

Jangan membuat aplikasi Flutter. Jangan membuat sistem audit baru. Jangan mengintegrasikan pembayaran atau transaksi Kaspa nyata. Jangan menambahkan pemotongan saldo atau mengaktifkan escrow.

Kerjakan Tahap 6 saja. Jangan langsung melanjutkan ke tahap berikutnya.

Laporan akhir

Laporkan dalam bahasa Indonesia:

1. Endpoint yang benar-benar berfungsi.
2. Perubahan model dan migrasi.
3. Aturan transisi status.
4. Pencegahan penerimaan ganda.
5. Hasil tes yang benar-benar dijalankan.
6. Tes yang belum dapat dijalankan.
7. Batasan yang masih tersisa.
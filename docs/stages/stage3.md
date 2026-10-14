TAHAP 3 — INTEGRASI POSTGRESQL DAN PENYIMPANAN DATA HCLEAN & PCLEAN

Lanjutkan proyek Hclean dan Pclean dari Tahap 1 dan Tahap 2. Pertahankan implementasi backend Rust berbasis Axum dan Tokio, kontrak API, envelope JSON bersama, katalog layanan simulasi, wallet simulasi, dokumentasi, dan semua pengujian yang sudah ada.

Pada tahap ini, fokus pada PostgreSQL dan fondasi penyimpanan data persisten.

1. Tujuan utama

Implementasikan:

- Koneksi PostgreSQL melalui SQLx.
- Pengelolaan konfigurasi database melalui environment variables.
- Connection pool yang digunakan bersama oleh handler backend.
- Migrasi database dengan versi yang terkontrol.
- Repository atau lapisan akses data yang terpisah dari handler HTTP.
- Penanganan kesalahan database yang aman.
- Pengujian penyimpanan data apabila lingkungan pengembangan mendukung PostgreSQL.

Gunakan SQLx dengan dukungan PostgreSQL dan Tokio. Pertahankan struktur modul yang sudah ada dan sesuaikan seperlunya; jangan membuat ulang proyek dari awal.

2. Konfigurasi database

Tambahkan konfigurasi untuk:

- "DATABASE_URL"
- "DATABASE_MAX_CONNECTIONS"
- "DATABASE_CONNECT_TIMEOUT_SECONDS"

Sediakan ".env.example" yang berisi placeholder saja. Jangan menaruh kata sandi database sungguhan di repositori.

Jika database belum dikonfigurasi:

- Backend boleh tetap berjalan dalam mode tanpa database hanya jika mekanisme ini sesuai dengan arsitektur yang ada.
- Endpoint "GET /health" tetap melaporkan status backend saja.
- Tambahkan informasi kesiapan database secara terpisah jika diperlukan.
- Endpoint yang membutuhkan database harus memberikan respons kesalahan yang jelas, bukan berpura-pura berhasil menyimpan data.

Jangan pernah mengklaim database sehat hanya karena server HTTP berhasil berjalan.

3. Buat migrasi database

Simpan semua migrasi SQL berurutan di "backend/migrations/", sesuai dengan lokasi yang digunakan SQLx pada backend ini.

Siapkan tabel berikut dengan primary key, foreign key, batasan data, dan indeks yang sesuai.

A. "users"

Kolom minimal:

- "id": UUID sebagai primary key.
- "email": unik dan dapat bernilai kosong apabila metode pendaftaran lain kelak didukung.
- "phone": unik dan dapat bernilai kosong.
- "password_hash": nullable sampai autentikasi diterapkan.
- "role": "customer", "worker", atau "admin".
- "display_name": nama tampilan.
- "is_active": status akun.
- "created_at" dan "updated_at": timestamp.

Pastikan email atau nomor telepon yang diperlukan untuk identifikasi akun tidak bisa digunakan berulang secara tidak sengaja. Terapkan batasan unik yang tepat untuk nilai yang tersedia.

Jangan menyimpan kata sandi dalam bentuk teks biasa.

B. "worker_profiles"

Kolom minimal:

- "user_id": foreign key ke "users".
- "bio".
- "is_available".
- "created_at" dan "updated_at".

Profil pekerja hanya boleh dikaitkan dengan akun pekerja.

C. "service_categories"

Kolom minimal:

- "id": UUID.
- "name".
- "description".
- "base_price_idr": bilangan bulat rupiah.
- "is_active".
- "created_at" dan "updated_at".

Masukkan tujuh kategori awal yang telah ditentukan pada Tahap 2:

1. Membersihkan toilet.
2. Membersihkan kasur.
3. Mencuci pakaian.
4. Menyetrika.
5. Cuci dan setrika.
6. Berkebun.
7. Maintenance/perbaikan rumah.

Gunakan seed atau migrasi yang dapat dijalankan berulang tanpa membuat kategori duplikat. Harga awal boleh berupa nilai konfigurasi awal yang jelas ditandai untuk pengembangan, bukan klaim harga final bisnis.

D. "orders"

Kolom minimal:

- "id": UUID.
- "customer_id": foreign key ke "users".
- "worker_id": nullable sampai pekerja menerima pesanan.
- "service_category_id": foreign key ke "service_categories".
- "service_price_idr": bilangan bulat rupiah.
- "customer_fee_idr": bilangan bulat rupiah.
- "worker_fee_idr": bilangan bulat rupiah.
- "status": status pesanan yang terdefinisi.
- "address" atau referensi alamat yang sesuai.
- "notes".
- "created_at" dan "updated_at".

Simpan snapshot harga dan biaya layanan pada saat pesanan dibuat. Perubahan harga kategori di kemudian hari tidak boleh mengubah nilai pesanan lama.

Untuk tahap ini, pembuatan dan perubahan pesanan melalui API tetap tidak aktif jika autentikasi belum tersedia. Jangan menerima "customer_id" dari request sebagai bukti identitas pengguna yang sah.

E. "wallet_accounts"

Kolom minimal:

- "id": UUID.
- "user_id": foreign key ke "users".
- "balance_idr": bilangan bulat.
- "wallet_type": "simulated".
- "created_at" dan "updated_at".

Batasi tipe wallet pada tahap ini ke wallet simulasi. Jangan menyimpan private key, seed phrase, atau kredensial wallet blockchain.

F. "wallet_transactions"

Kolom minimal:

- "id": UUID.
- "wallet_account_id": foreign key ke "wallet_accounts".
- "order_id": nullable.
- "transaction_type".
- "amount_idr": bilangan bulat.
- "status": status transaksi simulasi.
- "idempotency_key": nullable dengan batasan unik yang sesuai jika digunakan.
- "created_at" dan "updated_at".

Tabel ini merupakan pencatatan simulasi. Jangan menganggap transaksi database sebagai transaksi blockchain.

4. Lapisan akses data

Buat repository atau modul data terpisah untuk:

- Pengguna.
- Kategori layanan.
- Pesanan.
- Wallet dan transaksi simulasi.

Handler HTTP tidak boleh menyusun query database secara tersebar di seluruh file. Pisahkan tanggung jawab menjadi:

- Routes/handlers.
- Model dan tipe data.
- Repository.
- Konfigurasi database.
- Migrasi.

Gunakan query parameterized SQL. Jangan menggabungkan nilai input pengguna langsung ke string SQL.

5. Integrasi katalog layanan

Hubungkan "GET /api/v1/services" dan "GET /api/v1/services/{service_id}" ke PostgreSQL ketika database tersedia.

Ketentuannya:

- Data katalog berasal dari tabel "service_categories".
- Data hasil database tidak boleh diam-diam diganti dengan data hardcoded apabila query gagal.
- Jika database belum tersedia, gunakan respons kesalahan yang konsisten dan terdokumentasi.
- Pertahankan format envelope JSON yang sudah ada.
- Tandai katalog sebagai data simulasi atau data pengembangan jika harga awal belum final.

6. Model biaya layanan

Pertahankan aturan bisnis:

- Biaya layanan pelanggan = 20% dari harga jasa.
- Biaya layanan pekerja = 20% dari harga jasa.
- Pelanggan membayar harga jasa secara tunai langsung kepada pekerja.
- Biaya jaringan blockchain kelak dipisahkan dari biaya layanan.

Contoh harga jasa Rp100.000:

- Biaya pelanggan Rp20.000.
- Biaya pekerja Rp20.000.
- Total biaya layanan platform Rp40.000.

Gunakan perhitungan integer yang aman dari overflow. Definisikan kebijakan pembulatan secara konsisten apabila harga tidak habis dibagi lima.

Pada tahap ini, biaya hanya disimpan sebagai snapshot pesanan jika ada alur internal yang mengizinkannya. Jangan melakukan pemotongan saldo atau transfer uang sungguhan.

7. Konsistensi dan keamanan data

- Gunakan transaksi PostgreSQL untuk operasi yang melibatkan beberapa perubahan data.
- Jangan pernah mempercayai identitas pengguna yang dikirim klien sebelum autentikasi tersedia.
- Jangan membuat endpoint pendaftaran atau pemesanan tampak aktif jika alur autentikasi dan otorisasi belum siap.
- Jangan mengembalikan pesan error internal SQL kepada pengguna.
- Gunakan foreign key dan constraint untuk menjaga konsistensi data.
- Hindari saldo negatif apabila saldo simulasi kelak diubah.
- Rancang pencatatan wallet supaya perubahan saldo dan pencatatan transaksi dapat dilakukan atomik pada tahap berikutnya.
- Jangan mengklaim row lock di PostgreSQL mengunci dana atau UTXO di jaringan Kaspa.

8. Status API yang belum tersedia

Pertahankan status yang sudah ditentukan pada Tahap 2 untuk:

- Pendaftaran dan login/logout.
- Profil pengguna.
- Pembuatan, penerimaan, pemulaan, penyelesaian, dan pembatalan pesanan.
- Operasi wallet yang memerlukan autentikasi.

Jangan mengaktifkan operasi tersebut hanya karena tabelnya sudah tersedia. Endpoint baru boleh diaktifkan pada tahap selanjutnya setelah autentikasi, otorisasi, dan aturan transisi status selesai diterapkan.

9. Pengujian

Tambahkan tes yang relevan untuk:

- Validasi model dan perhitungan uang.
- Migrasi dan constraint database.
- Pembuatan serta pembacaan kategori layanan.
- Constraint unik pengguna.
- Foreign key pesanan dan wallet.
- Transaksi database dan rollback jika terjadi kegagalan.
- Perilaku backend ketika database tidak dikonfigurasi.

Tes integrasi database boleh menggunakan database PostgreSQL khusus pengujian. Jangan menjalankan tes yang menghapus tabel atau data pada database produksi.

Pertahankan tes Tahap 1 dan Tahap 2. Jangan menghapus tes hanya agar rangkaian tes lulus.

10. Dokumentasi

Perbarui:

- "README.md".
- "docs/setup/PERSIAPAN_TERMUX.md" jika tersedia.
- "docs/architecture/ARSITEKTUR.md".
- "docs/api/KONTRAK_API.md" apabila perilaku katalog layanan berubah.

Jelaskan cara menyiapkan PostgreSQL, mengatur environment variables, menjalankan migrasi, dan memulai backend. Bedakan instruksi Termux Android dan lingkungan Docker/VPS jika diperlukan.

11. Batasan wajib

Jangan membuat integrasi wallet Kaspa nyata, transaksi blockchain, penyimpanan private key, pembayaran, autentikasi produksi, atau sistem escrow on-chain pada tahap ini.

Jangan membuat sistem audit baru pada tahap ini.

Jangan mengklaim integrasi database atau pengujian berhasil jika sebenarnya tidak dijalankan. Jika PostgreSQL tidak tersedia di lingkungan pengembangan, implementasikan bagian yang bisa dilakukan dan dokumentasikan pengujian yang belum dapat dijalankan.

Kerjakan Tahap 3 saja. Jangan langsung melanjutkan ke Tahap 4.
TAHAP 5 — BACKEND PROFIL, ALAMAT, KATALOG LAYANAN, DAN FONDASI FITUR HCLEAN & PCLEAN

Lanjutkan proyek Hclean dan Pclean dari Tahap 1 sampai Tahap 4. Fokus hanya pada backend Rust. Jangan membuat atau mengintegrasikan aplikasi Flutter pada tahap ini.

Tujuan

Implementasikan fondasi backend yang benar-benar menggunakan PostgreSQL untuk:

1. Profil pengguna.
2. Profil pekerja.
3. Alamat pengguna dan alamat layanan.
4. Katalog layanan dan harga.
5. Struktur kontrak API untuk wallet placeholder dan fitur berikutnya, tanpa mengaktifkan transaksi keuangan.

Pertahankan Axum, Tokio, SQLx, envelope JSON bersama, autentikasi, otorisasi, dan seluruh pengujian yang sudah ada.

1. Integrasi dengan autentikasi

Gunakan identitas pengguna yang sudah diverifikasi oleh middleware autentikasi Tahap 4.

Jangan menerima "user_id" dari body atau query sebagai pengganti identitas terautentikasi.

Jika PostgreSQL belum tersedia, endpoint yang membutuhkan penyimpanan harus mengembalikan kesalahan yang sesuai. Jangan menyimpan data secara permanen dalam memori dan mengklaim operasi berhasil.

2. Profil pengguna

Implementasikan atau sempurnakan:

- "GET /api/v1/users/me"
- "PATCH /api/v1/users/me"

Field yang dapat diperbarui hanya field profil yang memang diizinkan, seperti nama tampilan dan nomor telepon.

Jangan izinkan perubahan ID, peran, status aktif, password hash, atau hak administratif melalui endpoint profil.

Validasi input, batas panjang teks, format nomor telepon, dan konflik nomor telepon.

Respons tidak boleh mengandung password hash atau informasi internal autentikasi.

3. Profil pekerja

Implementasikan atau sempurnakan endpoint profil pekerja, misalnya:

- "GET /api/v1/workers/me"
- "PATCH /api/v1/workers/me"

Hanya pengguna dengan peran "worker" yang dapat mengakses endpoint ini.

Field awal:

- Bio.
- Status ketersediaan.
- Informasi profil pekerja yang aman.

Jangan izinkan pekerja mengubah ID akun, peran, status verifikasi administratif, atau data sensitif.

Pastikan profil pekerja terhubung ke akun pekerja yang benar menggunakan foreign key dan constraint PostgreSQL.

4. Alamat

Rancang tabel alamat dan, bila diperlukan, tambahkan file migrasi baru berurutan di "backend/migrations/".

Field yang disarankan:

- ID UUID.
- User ID.
- Label alamat.
- Nama penerima.
- Nomor telepon penerima.
- Alamat lengkap.
- Detail tambahan opsional.
- Latitude dan longitude opsional.
- Penanda alamat utama.
- Timestamp pembuatan dan pembaruan.

Implementasikan:

- "GET /api/v1/addresses"
- "POST /api/v1/addresses"
- "GET /api/v1/addresses/{address_id}"
- "PATCH /api/v1/addresses/{address_id}"
- "DELETE /api/v1/addresses/{address_id}"

Setiap pengguna hanya boleh melihat dan mengubah alamat miliknya sendiri. Terapkan ownership check di query atau repository, bukan hanya di aplikasi klien.

Jika satu alamat ditandai sebagai alamat utama, pastikan perubahan dilakukan secara atomik sehingga pengguna tidak memiliki beberapa alamat utama secara tidak sengaja.

Jangan mewajibkan koordinat GPS. Validasi rentang latitude dan longitude jika nilainya diberikan.

5. Katalog layanan dan harga

Pastikan endpoint berikut menggunakan PostgreSQL:

- "GET /api/v1/services"
- "GET /api/v1/services/{service_id}"

Kategori awal:

1. Membersihkan toilet.
2. Membersihkan kasur.
3. Mencuci pakaian.
4. Menyetrika.
5. Cuci dan setrika.
6. Berkebun.
7. Maintenance/perbaikan rumah.

Gunakan bilangan bulat untuk harga rupiah.

Jika database aktif, jangan diam-diam mengganti hasil query yang gagal dengan data hardcoded.

Jika harga belum final, tandai sebagai harga pengembangan dalam dokumentasi atau metadata yang sesuai.

Jangan membuat endpoint administrasi pengubahan harga yang terbuka bagi pengguna biasa. Pengelolaan admin hanya boleh ditambahkan jika pemeriksaan peran admin sudah benar-benar tersedia.

6. Fondasi pesanan

Untuk tahap ini, jangan mengaktifkan seluruh alur pesanan. Pastikan model, migrasi, dan kontrak yang ada siap mendukung:

- Pelanggan membuat pesanan.
- Pekerja melihat pesanan yang tersedia.
- Pekerja menerima pesanan.
- Pekerja memulai pekerjaan.
- Pekerja menyelesaikan pekerjaan.
- Pembatalan sesuai aturan.

Jika model pesanan atau transisi status belum aman, pertahankan endpoint terkait dalam keadaan belum tersedia.

Jangan menerima ID pelanggan atau pekerja dari klien sebagai bukti identitas. Jangan mengizinkan pekerja menerima pesanan milik sendiri. Snapshot harga dan biaya harus disusun backend.

7. Wallet placeholder

Pertahankan endpoint wallet yang sudah ada:

- "GET /api/v1/wallet"
- "GET /api/v1/wallet/transactions"

Jika endpoint membutuhkan autentikasi, gunakan ID pengguna dari sesi terverifikasi.

Semua data wallet harus menampilkan "simulated: true".

Jangan menambahkan:

- Pembuatan wallet Kaspa nyata.
- Seed phrase atau private key.
- Transfer blockchain.
- Pemotongan saldo nyata.
- Klaim bahwa biaya layanan sudah dibayar.
- Deposit atau penarikan uang sungguhan.

Dokumentasikan bahwa saldo simulasi bukan aset KAS dan tidak dapat ditarik.

8. Struktur data fitur berikutnya

Jika masih sesuai dengan arsitektur saat ini, siapkan desain skema untuk:

- Referensi percakapan antara pelanggan dan pekerja.
- Pesan terkait pesanan.
- Notifikasi status pesanan.

Jangan mengimplementasikan layanan real-time yang besar pada tahap ini. Prioritaskan migrasi dan kontrak yang sederhana serta konsisten. Jangan menyimpan informasi pribadi yang tidak diperlukan.

9. Keamanan dan konsistensi

- Gunakan query parameterized melalui SQLx.
- Gunakan transaksi database untuk perubahan multi-tabel.
- Validasi ownership pada setiap operasi data pribadi.
- Pertahankan pembatasan peran.
- Jangan mengungkapkan detail SQL atau stack trace kepada klien.
- Jangan menambahkan sistem audit baru pada tahap ini.
- Jangan mengubah migrasi lama yang sudah diterapkan secara destruktif; tambahkan file migrasi baru di "backend/migrations/".
- Jangan menghapus tes yang sudah ada.

10. Pengujian dan dokumentasi

Tambahkan tes untuk:

- Pembacaan dan pembaruan profil.
- Penolakan akses profil tanpa autentikasi.
- Pembatasan endpoint pekerja berdasarkan peran.
- Kepemilikan alamat.
- Validasi alamat dan koordinat.
- Konsistensi alamat utama.
- Pembacaan katalog dari database.
- Penanganan database yang belum tersedia.

Perbarui "docs/api/KONTRAK_API.md", "docs/api/openapi.yaml", "README.md", dan "docs/architecture/ARSITEKTUR.md" sesuai implementasi yang benar-benar tersedia.

Jalankan pengujian yang dapat dijalankan di lingkungan saat ini. Jika PostgreSQL tidak tersedia, pisahkan hasil tes unit dari tes integrasi yang belum bisa dijalankan.

Batasan

Fokus backend saja. Jangan membuat Flutter. Jangan mengimplementasikan pembayaran, transaksi Kaspa, autentikasi baru yang terpisah, atau sistem audit baru. Jangan mengaktifkan endpoint yang belum memiliki otorisasi dan penyimpanan yang benar.

Kerjakan Tahap 5 saja. Jangan melanjutkan ke tahap berikutnya.

Laporan akhir

Laporkan dalam bahasa Indonesia:

1. Berkas dan migrasi yang ditambahkan.
2. Endpoint yang benar-benar berfungsi.
3. Endpoint yang masih belum tersedia.
4. Hasil pengujian yang benar-benar dijalankan.
5. Ketergantungan pada PostgreSQL.
6. Batasan wallet placeholder.
7. Rekomendasi tahap berikutnya.
TAHAP 7 — CHAT DAN NOTIFIKASI BACKEND HCLEAN & PCLEAN

Lanjutkan proyek Hclean dan Pclean dari tahap-tahap sebelumnya. Fokus hanya pada backend Rust dan PostgreSQL. Jangan membangun aplikasi Flutter.

Pertahankan arsitektur Axum, Tokio, SQLx, autentikasi, otorisasi, envelope JSON, repository, migrasi, workflow pesanan, dan pengujian yang sudah ada.

Sebelum menambahkan integrasi, gunakan model dan endpoint pesanan yang benar-benar tersedia saat ini. Jangan mengasumsikan route atau field tertentu sudah ada jika belum diimplementasikan.

1. Tujuan

Implementasikan:

- Chat yang terkait dengan pesanan.
- Penyimpanan pesan di PostgreSQL.
- Daftar percakapan yang boleh diakses pengguna.
- Notifikasi dalam aplikasi.
- Notifikasi otomatis untuk peristiwa penting pesanan dan pesan.
- Penanda notifikasi sudah dibaca.
- Pagination dan kontrol akses.
- Fondasi arsitektur yang memungkinkan real-time ditambahkan kemudian.

Jangan membuat sistem audit baru.

2. Migrasi database

Tambahkan file migrasi baru berurutan di "backend/migrations/" tanpa mengubah migrasi lama secara destruktif.

Buat tabel "order_conversations" atau model percakapan yang setara dengan:

- "id": UUID.
- "order_id": foreign key ke pesanan.
- "created_at" dan "updated_at".

Jika satu pesanan hanya memiliki satu percakapan, terapkan unique constraint pada "order_id".

Buat tabel "messages" dengan:

- "id": UUID.
- "conversation_id": foreign key ke percakapan.
- "sender_id": foreign key ke pengguna.
- "body": isi pesan teks.
- "created_at".
- "updated_at" jika pesan dapat diedit.

Tambahkan indeks untuk mengambil pesan berdasarkan percakapan dan waktu pembuatan.

Buat tabel "notifications" dengan:

- "id": UUID.
- "user_id": foreign key penerima.
- "type": jenis notifikasi.
- "title": judul.
- "body": ringkasan aman.
- "resource_type" dan "resource_id" atau referensi yang setara.
- "read_at": nullable.
- "created_at".

Tambahkan indeks untuk daftar notifikasi pengguna berdasarkan waktu, serta notifikasi yang belum dibaca.

Jangan menyimpan token autentikasi, private key, seed phrase, atau data rahasia di pesan maupun notifikasi.

3. API chat

Implementasikan endpoint dengan pola URL yang konsisten dengan API yang ada. Endpoint yang disarankan:

- "GET /api/v1/orders/{order_id}/messages"
- "POST /api/v1/orders/{order_id}/messages"

Jika arsitektur lebih sesuai menggunakan "conversation_id", dokumentasikan dan gunakan pola tersebut secara konsisten.

Ketentuan:

- Pengguna harus terautentikasi.
- Hanya pelanggan pemilik pesanan dan pekerja yang ditugaskan ke pesanan tersebut yang dapat mengakses percakapan.
- Jangan izinkan pekerja yang belum menerima pesanan membaca alamat atau percakapan privat.
- Jangan mempercayai "sender_id" dari request; tentukan pengirim berdasarkan sesi terverifikasi.
- Batasi panjang pesan, misalnya maksimal 4.000 karakter.
- Tolak pesan kosong atau yang hanya berisi spasi.
- Terapkan validasi input dan pagination.
- Kembalikan pesan secara berurutan dengan pengurutan yang stabil.
- Gunakan query berparameter melalui SQLx.
- Jangan mengembalikan informasi akun atau data privat yang tidak diperlukan.

Jangan mengimplementasikan upload file, foto, video, panggilan suara, atau enkripsi end-to-end dalam tahap ini.

4. Aturan status percakapan

Percakapan harus terkait dengan pesanan yang valid.

Tentukan kapan percakapan dapat dibuat dan diakses. Jika percakapan dibuat otomatis saat pesanan diterima, lakukan secara atomik bersama perubahan status pesanan.

Jangan membuat percakapan duplikat untuk pesanan yang sama apabila modelnya satu percakapan per pesanan.

Tentukan dan dokumentasikan apakah pesan masih dapat dibaca setelah pesanan selesai atau dibatalkan. Gunakan kebijakan konsisten dan jangan menghapus riwayat hanya karena status pesanan berubah.

5. API notifikasi

Implementasikan endpoint:

- "GET /api/v1/notifications"
- "GET /api/v1/notifications/unread-count"
- "PATCH /api/v1/notifications/{notification_id}/read"
- "POST /api/v1/notifications/read-all"

Semua endpoint hanya boleh mengakses notifikasi milik pengguna yang terautentikasi.

Gunakan pagination dengan batas maksimum yang wajar. Operasi penandaan sudah dibaca harus memeriksa kepemilikan notifikasi dan tidak boleh mengubah data pengguna lain.

Jika notifikasi tidak ditemukan atau bukan milik pengguna, gunakan respons yang sesuai dengan kebijakan privasi API.

Jangan menerima "user_id" penerima sebagai identitas yang dipercaya dari klien.

6. Pemicu notifikasi

Integrasikan notifikasi dengan workflow yang benar-benar sudah ada, jika workflow pesanan telah diimplementasikan.

Jenis peristiwa yang disarankan:

- Pesanan baru tersedia bagi pekerja yang relevan.
- Pekerja menerima pesanan.
- Pekerjaan dimulai.
- Pekerjaan selesai.
- Pesanan dibatalkan.
- Pesan chat baru diterima.

Notifikasi hanya boleh dibuat untuk penerima yang benar-benar relevan.

Jangan membuat notifikasi palsu jika perubahan status pesanan gagal.

Jika perubahan pesanan dan pembuatan notifikasi terjadi dalam satu operasi, gunakan transaksi PostgreSQL agar keduanya konsisten.

Jika notifikasi kelak dikirim ke sistem eksternal, pertimbangkan pola transactional outbox. Pada tahap ini, implementasikan hanya jika sesuai dengan kompleksitas arsitektur yang sudah ada; jangan menambahkan broker pesan yang tidak diperlukan.

Jangan mengirim notifikasi ke semua pekerja tanpa mempertimbangkan relevansi dan privasi.

7. Keandalan notifikasi

Penyimpanan notifikasi harus terpisah dari mekanisme pengiriman push notification.

Pada tahap ini, implementasikan notifikasi yang dapat dibaca melalui API. Jangan mengklaim bahwa notifikasi sudah muncul di perangkat Android atau iOS.

Siapkan model agar layanan push notification dapat ditambahkan kemudian.

Pastikan kegagalan database tidak menghasilkan respons sukses palsu.

8. Real-time dan WebSocket

Jangan membangun infrastruktur WebSocket lengkap jika belum dibutuhkan oleh fondasi API.

Pastikan model dan API dapat dikembangkan untuk mendukung real-time nanti.

Jika WebSocket diimplementasikan pada tahap ini, wajib:

- Memverifikasi autentikasi saat koneksi dibuat.
- Memeriksa izin akses percakapan.
- Memvalidasi ukuran dan format pesan.
- Tidak menerima identitas pengguna dari payload sebagai bukti autentikasi.
- Membatasi koneksi dan menangani disconnect.
- Tetap menyimpan pesan ke PostgreSQL.
- Tidak menganggap pengiriman melalui WebSocket sebagai bukti bahwa pesan telah tersimpan.

Prioritaskan REST API yang stabil dan penyimpanan permanen apabila real-time belum diperlukan.

9. Privasi dan keamanan

- Terapkan pemeriksaan kepemilikan di backend.
- Gunakan transaksi SQLx bila beberapa tabel berubah bersama.
- Jangan mencatat isi pesan privat ke log aplikasi secara tidak perlu.
- Jangan menyertakan isi pesan lengkap dalam notifikasi apabila ringkasan singkat sudah cukup.
- Validasi dan batasi ukuran input.
- Jangan mengizinkan pengguna mengubah pengirim, penerima, atau pemilik percakapan melalui request.
- Pertahankan middleware autentikasi dan otorisasi yang sudah ada.
- Jangan menambahkan sistem audit baru.

10. Kontrak API dan dokumentasi

Pertahankan envelope JSON yang telah digunakan.

Perbarui:

- "docs/api/KONTRAK_API.md"
- "docs/api/openapi.yaml"
- "docs/architecture/ARSITEKTUR.md"
- "README.md"

Dokumentasikan:

- Endpoint chat.
- Endpoint notifikasi.
- Model pesan dan notifikasi.
- Aturan kepemilikan.
- Pagination.
- Batas panjang pesan.
- Pemicu notifikasi.
- Perilaku ketika PostgreSQL tidak tersedia.
- Batasan push notification dan real-time.

Pastikan OpenAPI sesuai dengan endpoint yang benar-benar diimplementasikan.

11. Pengujian

Tambahkan pengujian untuk:

- Membaca dan mengirim pesan.
- Penolakan pesan kosong dan pesan terlalu panjang.
- Penolakan akses oleh pengguna yang bukan pihak pesanan.
- Penolakan akses tanpa autentikasi.
- Pagination dan urutan pesan.
- Membuat dan membaca notifikasi.
- Menghitung notifikasi belum dibaca.
- Menandai notifikasi dibaca.
- Penolakan akses notifikasi milik pengguna lain.
- Notifikasi hanya dibuat ketika perubahan pesanan berhasil.
- Konsistensi transaksi jika pembuatan notifikasi gagal.
- Perilaku ketika database tidak tersedia.

Tes integrasi harus menggunakan PostgreSQL khusus pengujian. Jika database tidak tersedia, laporkan tes integrasi sebagai belum dijalankan; jangan menyebutnya lulus hanya karena dilewati.

Pertahankan seluruh pengujian tahap sebelumnya.

Batasan wajib

Jangan membangun Flutter. Jangan mengintegrasikan wallet Kaspa nyata, pembayaran, atau transfer saldo. Jangan mengklaim push notification atau real-time berfungsi jika hanya API dan database yang tersedia.

Jangan membuat sistem audit baru.

Kerjakan Tahap 7 saja. Jangan lanjut ke tahap berikutnya.

Laporan akhir

Laporkan dalam bahasa Indonesia:

1. Migrasi dan model yang ditambahkan.
2. Endpoint chat dan notifikasi yang benar-benar berfungsi.
3. Aturan akses percakapan.
4. Pemicu notifikasi yang terhubung ke workflow pesanan.
5. Pengujian yang benar-benar dijalankan.
6. Pengujian yang belum dapat dijalankan.
7. Batasan real-time dan push notification.
8. Rekomendasi tahap selanjutnya.
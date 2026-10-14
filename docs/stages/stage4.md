TAHAP 4 — AUTENTIKASI DAN OTORISASI HCLEAN & PCLEAN

Lanjutkan proyek yang telah menyelesaikan Tahap 1, Tahap 2, dan Tahap 3. Pertahankan backend Rust berbasis Axum dan Tokio, kontrak API, konfigurasi PostgreSQL, connection pool, migrasi database, repository kategori layanan, envelope JSON bersama, dan pengujian yang sudah ada.

Kerjakan Tahap 4 saja. Jangan membuat seluruh aplikasi Flutter sekaligus.

1. Tujuan

Implementasikan sistem akun dan autentikasi yang dapat digunakan oleh:

- Hclean: aplikasi pelanggan.
- Pclean: aplikasi pekerja.
- Admin: peran khusus yang tidak boleh diberikan melalui pendaftaran publik.

Satu backend melayani kedua aplikasi. Peran pengguna ditentukan dan diverifikasi oleh backend, bukan dipercaya dari input klien.

Gunakan PostgreSQL untuk menyimpan akun. Jika database belum tersedia, endpoint autentikasi harus gagal secara jelas dan aman, bukan membuat akun sementara yang terlihat seperti akun permanen.

2. Pendaftaran akun

Implementasikan "POST /api/v1/auth/register".

Dukung data dasar:

- Email atau nomor telepon sebagai identitas akun.
- Kata sandi.
- Nama tampilan.
- Peran "customer" atau "worker", dengan aturan validasi yang jelas.

Aturan:

- Jangan mengizinkan pendaftaran publik sebagai "admin".
- Validasi input di backend.
- Normalisasi email dengan aturan konsisten sebelum menyimpan dan memeriksa keunikan.
- Nomor telepon harus dinormalisasi atau divalidasi dengan aturan yang terdokumentasi.
- Tolak akun duplikat dengan respons yang konsisten.
- Simpan kata sandi hanya dalam bentuk hash yang aman.
- Gunakan Argon2id untuk hashing kata sandi, dengan salt unik per kata sandi dan parameter yang wajar.
- Jangan mencatat kata sandi, hash kata sandi, token, atau data rahasia ke log.
- Gunakan transaksi database jika pendaftaran juga membuat profil pekerja atau wallet simulasi.

Pendaftaran berhasil harus menghasilkan ID pengguna dan informasi akun yang aman untuk dikirim kepada klien. Jangan mengirim hash kata sandi.

Jika email atau nomor telepon wajib digunakan, pastikan setidaknya satu identitas tersedia dan constraint database tetap mencegah duplikasi.

3. Login dan sesi

Implementasikan "POST /api/v1/auth/login".

Ketentuan:

- Verifikasi kata sandi menggunakan hash yang tersimpan.
- Jangan mengungkapkan apakah email, nomor telepon, atau kata sandi tertentu yang salah.
- Gunakan pesan kesalahan umum untuk kredensial yang tidak valid.
- Terapkan pembatasan percobaan login yang wajar dan desain yang siap untuk rate limiting.
- Jangan menganggap ID pengguna dari request sebagai bukti autentikasi.

Gunakan mekanisme sesi yang aman, dengan token akses berumur pendek dan mekanisme pembaruan sesi yang aman. Pilih implementasi yang kompatibel dengan aplikasi Flutter dan dokumentasikan keputusan arsitekturnya.

Jika menggunakan access token dan refresh token:

- Validasi tanda tangan, masa berlaku, dan klaim yang diperlukan.
- Batasi algoritme tanda tangan yang diterima.
- Simpan hanya hash refresh token di database.
- Terapkan rotasi refresh token dan pencabutan sesi.
- Jangan memasukkan kata sandi atau rahasia sensitif ke dalam token.
- Jangan menyimpan token atau secret hardcoded di kode sumber.

Jika menggunakan JWT, ambil secret dari konfigurasi aman dan validasi semua klaim yang diperlukan. Jangan menyediakan fallback secret yang lemah pada lingkungan produksi.

4. Logout dan pengelolaan sesi

Implementasikan:

- "POST /api/v1/auth/logout"
- Endpoint pembaruan token jika diperlukan.
- Penyimpanan sesi atau refresh token yang dapat dicabut.

Logout harus mencabut sesi atau refresh token yang sesuai di server. Jangan sekadar mengembalikan pesan sukses tanpa perubahan yang nyata.

Tentukan masa berlaku token dan perilaku setelah refresh token digunakan kembali atau dicabut. Dokumentasikan keputusan tersebut.

5. Middleware autentikasi dan otorisasi

Buat middleware atau extractor Axum yang dapat:

- Membaca kredensial autentikasi dari header "Authorization: Bearer ...".
- Memvalidasi token.
- Memastikan sesi belum dicabut.
- Mengambil identitas dan peran pengguna yang sudah terverifikasi.
- Menyediakan identitas terverifikasi bagi handler.

Implementasikan pemeriksaan peran di backend:

- "customer": untuk fungsi pelanggan.
- "worker": untuk fungsi pekerja.
- "admin": hanya jika akun admin dibuat melalui proses administratif yang aman.

Jangan mempercayai peran dari body JSON, parameter URL, atau header buatan klien.

6. Profil pengguna

Aktifkan:

- "GET /api/v1/users/me"
- "PATCH /api/v1/users/me"

Endpoint tersebut harus menggunakan identitas dari middleware autentikasi.

Pengguna hanya boleh membaca atau mengubah profil miliknya sendiri. Jangan izinkan klien mengubah:

- ID pengguna.
- Peran.
- Status aktif.
- Password hash.
- Hak administratif.
- Data keamanan sesi.

Jika perubahan kata sandi belum masuk cakupan, jangan membuat field perubahan kata sandi yang tampak berfungsi tetapi sebenarnya tidak aman.

7. Profil pekerja

Integrasikan tabel "worker_profiles" yang telah dirancang pada Tahap 3.

- Pembuatan profil pekerja dilakukan melalui transaksi yang konsisten dengan pendaftaran.
- Hanya akun berperan "worker" yang boleh memiliki profil pekerja.
- Jangan mengizinkan pelanggan mengubah ketersediaan pekerja.
- Jangan mengaktifkan penerimaan atau pelaksanaan pesanan pada tahap ini.

8. Struktur database

Tambahkan migrasi baru untuk sesi dan refresh token sebagai file berurutan di "backend/migrations/".

Pastikan:

- Foreign key ke pengguna.
- Hash token tidak disimpan sebagai token mentah.
- Ada masa berlaku dan status pencabutan.
- Ada indeks untuk pencarian sesi aktif.
- Constraint dan transaksi menjaga konsistensi data.

Jangan menghapus data migrasi lama atau mengubah migrasi yang sudah diterapkan dengan cara yang merusak. Semua perubahan skema harus menggunakan file migrasi baru di "backend/migrations/".

9. Wallet simulasi

Pertahankan endpoint wallet simulasi yang sudah ada. Jangan mengaktifkan transfer saldo atau operasi yang membutuhkan autentikasi jika belum memiliki kontrol akses yang benar.

Jika wallet simulasi dihubungkan dengan akun:

- Hubungkan berdasarkan ID pengguna dari sesi yang terverifikasi.
- Jangan menerima "user_id" sebagai identitas yang dipercaya dari klien.
- Tandai saldo dan transaksi sebagai simulasi.
- Jangan mengklaim saldo sebagai dana KAS nyata.
- Jangan membuat atau menyimpan seed phrase dan private key.

Jangan melakukan transaksi blockchain atau pemotongan saldo nyata.

10. Respons API dan keamanan

Pertahankan envelope JSON bersama dari Tahap 2.

Gunakan kode status HTTP yang sesuai, misalnya:

- "201" untuk pendaftaran berhasil.
- "200" untuk login atau pembacaan profil berhasil.
- "400" untuk input yang tidak valid.
- "401" untuk kredensial atau autentikasi tidak valid.
- "403" untuk pengguna terautentikasi yang tidak memiliki izin.
- "409" untuk konflik identitas akun.
- "503" jika layanan membutuhkan database tetapi database tidak tersedia.

Jangan membocorkan detail SQL, stack trace, token, secret, hash kata sandi, atau informasi internal server.

Pastikan CORS tidak mengizinkan semua origin secara sembarangan dalam konfigurasi produksi. Jangan menganggap CORS sebagai pengganti autentikasi.

11. Konfigurasi dan dokumentasi

Tambahkan placeholder konfigurasi yang diperlukan ke ".env.example", misalnya:

- Konfigurasi kunci penandatanganan token.
- Masa berlaku access token.
- Masa berlaku refresh token.
- Parameter hashing jika perlu.
- Konfigurasi rate limiting jika didukung.

Jangan menaruh secret nyata ke repositori.

Perbarui:

- "README.md".
- "docs/architecture/ARSITEKTUR.md".
- "docs/api/KONTRAK_API.md".
- "docs/api/openapi.yaml".

Dokumentasikan:

- Alur daftar dan masuk.
- Bentuk request dan response.
- Header autentikasi.
- Masa berlaku dan pembaruan sesi.
- Logout dan pencabutan sesi.
- Perbedaan peran customer dan worker.
- Perilaku ketika PostgreSQL tidak tersedia.
- Konfigurasi lokal dan catatan keamanan deployment.

Pastikan dokumentasi sesuai dengan endpoint yang benar-benar diimplementasikan.

12. Pengujian

Tambahkan pengujian untuk:

- Pendaftaran pelanggan dan pekerja.
- Penolakan pendaftaran admin.
- Validasi input.
- Penolakan akun duplikat.
- Verifikasi kata sandi yang benar dan salah.
- Login dengan kredensial tidak valid.
- Akses tanpa token.
- Token kedaluwarsa atau tidak valid.
- Logout dan pencabutan sesi.
- Akses profil milik sendiri.
- Penolakan perubahan peran melalui profil.
- Penolakan akses berdasarkan peran.
- Kegagalan aman saat database tidak tersedia.
- Tidak adanya token dan hash kata sandi di respons.

Gunakan database khusus pengujian untuk tes integrasi. Jangan menghapus atau mereset database produksi.

Pertahankan seluruh pengujian dari tahap sebelumnya.

13. Batasan tahap

Jangan mengaktifkan pembuatan atau penerimaan pesanan, pembayaran, pemotongan saldo, transfer wallet, transaksi Kaspa nyata, atau fungsi escrow.

Jangan membuat sistem audit baru pada tahap ini.

Jangan mengklaim fitur aman atau berfungsi jika belum diimplementasikan dan diuji. Jika suatu bagian belum selesai, dokumentasikan secara eksplisit dan jangan mengembalikan respons sukses palsu.

Kerjakan Tahap 4 saja.

14. Laporan akhir

Laporkan dalam bahasa Indonesia:

1. Berkas yang dibuat dan diubah.
2. Endpoint autentikasi yang sudah berfungsi.
3. Mekanisme hashing dan sesi yang dipilih.
4. Migrasi database yang ditambahkan.
5. Aturan akses customer, worker, dan admin.
6. Pengujian yang benar-benar dijalankan dan hasilnya.
7. Bagian yang masih belum tersedia.
8. Langkah berikutnya yang direkomendasikan.
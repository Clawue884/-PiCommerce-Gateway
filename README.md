---

# PiCommerce Gateway  
### Enterprise-Ready Payment Gateway for Pi Apps  
Integrasi Pi SDK | Laravel API | React + Vite | Webhook HMAC | Audit Log

---

## 🧭 Ringkasan Proyek
**PiCommerce Gateway** adalah sistem pembayaran enterprise untuk aplikasi Pi Network yang menggunakan Pi SDK.  
Proyek ini menyediakan alur pembayaran end-to-end:

- Pembuatan Purchase Order (PO)
- Proses pembayaran Pi (A2U Payment)
- Verifikasi Webhook aman berbasis HMAC
- Penyelesaian PO otomatis
- Logging transaksi & audit trail
- Standar keamanan setara aplikasi finansial modern

Proyek mengikuti **Pi Platform Developer Guidelines** serta praktik terbaik industri.

---

## ⭐ Fitur Kelas Enterprise

### ✔ Purchase Order (PO) System  
- Status otomatis: `created → pending → paid → settled`  
- ID unik (`merchantRef`) per transaksi  
- Validasi & anti-replay

### ✔ A2U Payment (Pi SDK)  
- Menggunakan SDK resmi Pi Browser  
- Integrasi React wrapper yang bersih  
- Mendukung pembayaran direct dan via PO

### ✔ Webhook Aman (HMAC SHA256)  
- Verifikasi signature: `X-Pi-Signature`  
- Sanitasi payload  
- Anti replay + rate limiting

### ✔ Frontend React (Vite)  
- Ringan dan cepat  
- Siap langsung berjalan di Pi Browser  
- Wrapper SDK aman & minimal

### ✔ Backend Laravel 10  
- API bersih dengan controller modular  
- Validasi request ketat  
- Audit log (dibangun di middleware)

### ✔ Infrastruktur Modern  
- Docker-ready  
- Struktur clean architecture  
- CI/CD friendly  
- Dokumentasi lengkap

---

## 🏗 Struktur Direktori

pi-commerce-gateway/ │── frontend/ │   ├── src/ │   │   ├── App.jsx │   │   ├── components/ │   │   └── services/pi-sdk.js │   ├── index.html │   └── package.json │ │── backend/ │   ├── app/ │   │   └── Http/Controllers/PurchaseOrderController.php │   ├── database/migrations/ │   ├── routes/api.php │   └── composer.json │ │── docs/ │   ├── architecture.md │   ├── SECURITY.md │   └── CONTRIBUTING.md │ │── infra/ │   ├── Dockerfile │   └── nginx.conf │ ├── logo.png ├── LICENSE └── README.md

---

## 🧰 Teknologi yang Digunakan
- Pi SDK (Pi App Platform)  
- React + Vite  
- Laravel 10 (PHP 8+)  
- MySQL / MariaDB  
- Docker & Nginx  
- Node 18+  

---

## 📡 Cara Install

### 1. Frontend
```bash
cd frontend
npm install
npm run dev

2. Backend

cd backend
composer install
php artisan migrate
php artisan serve


---

🔐 Keamanan Singkat

HTTPS wajib

HMAC verification

Sanitasi input

Tidak menyimpan credential di localStorage

CSP & rate-limiting

Audit log


Detail lengkap ada di SECURITY.md


---

🤝 Kontribusi

Lihat CONTRIBUTING.md.


---

📘 Lisensi

Proyek ini berlisensi MIT — bebas digunakan untuk tujuan pribadi atau komersial.


---

# ✅ **2. CONTRIBUTING.md (Profesional + Standar Pi Developer)**

```md
# Contributing Guidelines

Terima kasih atas minat Anda berkontribusi pada **PiCommerce Gateway**.  
Proyek ini mengikuti standar industri dan pedoman Developer Pi Network.

---

## 🧱 Prinsip Utama Kontribusi
1. Keamanan adalah prioritas utama  
2. Tidak ada klaim harga, nilai, atau spekulasi Pi  
3. Kode harus bersih, modular, dan dapat diuji  
4. Semua perubahan harus mengikuti arsitektur proyek  
5. Dokumentasi wajib untuk setiap PR

---

## 🛠 Cara Berkontribusi
### 1. Fork repository
### 2. Buat branch baru

git checkout -b feature/nama-fitur

### 3. Buat perubahan dan commit

git commit -m "Menambahkan fitur X"

### 4. Push dan buat Pull Request
Ikuti template PR yang telah disediakan.

---

## 🧪 Standar Kode
### Backend (Laravel)
- PSR-12  
- Validasi request menggunakan FormRequest  
- Tidak ada query non-prepared  
- Gunakan service layer bila perlu  

### Frontend (React)
- Hindari state global yang tidak perlu  
- Pi SDK hanya dipanggil via wrapper `pi-sdk.js`  
- Jangan memuat library tidak aman  

---

## 🔐 Keamanan & Compliance
Kontribusi ditolak jika:
- Mengandung klaim harga Pi  
- Mengubah mekanisme signature  
- Melanggar standar Pi App Platform  

---

## 📄 Dokumentasi
Semua fitur baru *wajib* menambah atau memperbarui:
- README.md  
- docs/architecture.md  
- Comment pada kode  

---

Terima kasih telah membantu membangun ekosistem Pi!


---

✅ 3. SECURITY.md (Standar Keamanan Lengkap)

# Security Policy

Keamanan adalah fondasi utama PiCommerce Gateway.

---

## 🔐 Prinsip Keamanan
- Defense in Depth  
- Zero Trust Model  
- No sensitive data at rest  
- Semua endpoint divalidasi  

---

## 🧩 Bagian yang Dilindungi

### 1. Webhook
- Verifikasi HMAC SHA256  
- Header wajib: `X-Pi-Signature`  
- Timestamp validation untuk anti-replay  

### 2. Backend Laravel
- Prepared statements  
- Rate limiting (5 req/detik)  
- CSRF (untuk non-API)  
- Sanitasi input dan output  

### 3. Frontend React
- Tidak menyimpan credential/token  
- CSP ketat  
- HTTPS-only  
- Pi SDK dari domain resmi  

### 4. Infrastruktur
- Docker minimal privilege  
- Nginx security headers  
- Logging & audit trail  

---

## 🚨 Melaporkan Kerentanan
Laporkan melalui email:

security@picommerce.dev

Berikan:
- Deskripsi masalah  
- Cara reproduksi  
- Dampak potensial  
- Saran mitigasi  

---

## 📌 Catatan penting
Proyek ini **tidak**:
- Menyediakan harga Pi  
- Mengaitkan nilai Pi dengan mata uang fiat  
- Melakukan transaksi di luar App Platform


---

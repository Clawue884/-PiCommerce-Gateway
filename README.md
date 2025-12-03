---


# PiCommerce Gateway

### 🚀 Enterprise-Ready Payment Gateway for Pi Network Applications  
**Laravel 10 + React + Pi SDK + Webhook HMAC + CI/CD + Audit Trail**

---

## 🧭 Ringkasan Proyek

**PiCommerce Gateway** adalah *payment layer resmi open-source* yang dirancang untuk aplikasi dalam ekosistem **Pi Network**.

Sistem ini menyediakan alur pembayaran lengkap menggunakan **Pi SDK** dan standard **A2U Payment Flow**, mencakup:

- Manajemen Purchase Order (PO)
- Otorisasi & verifikasi pembayaran
- Webhook signature HMAC SHA256
- Logging transaksi dan audit trail
- Dashboard admin terintegrasi
- CI/CD untuk build, test, security scanning & auto release

Proyek ini mengikuti **Pi Platform Developer Guidelines** serta prinsip keamanan fintech modern.

---

## ⭐ Fitur Utama

| Fitur | Status |
|-------|--------|
| A2U Payment Integration | ✅ |
| Purchase Order System | ✅ |
| Secure Webhook (HMAC SHA256) | ✅ |
| React Frontend + Vite | ✅ |
| Laravel 10 API Backend | ✅ |
| Audit Logging | 🚧 |
| CI/CD Pipeline (GitHub Actions) | 🚧 |
| Admin Dashboard | 🚧 |
| Semantic Versioning + Auto Release | 🚧 |
| Plugin Mode / Extend API | 🚧 |

---

## 🏗 Arsitektur Sistem

Frontend (React + Pi SDK) ↓ REST API (Laravel 10) ↓ Webhook HMAC Verification ↓ Database (MySQL/MariaDB) ↓ Audit Log + Event Bus

Dokumentasi lengkap di:  
📄 `docs/architecture.md`  
📄 `docs/api-reference.yml`  

---

## 📡 Instalasi

### 1️⃣ Clone Repository

```sh
git clone https://github.com/Clawue884/-PiCommerce-Gateway
cd -PiCommerce-Gateway


---

2️⃣ Install Backend (Laravel)

cd backend
composer install
cp .env.example .env
php artisan migrate
php artisan serve


---

3️⃣ Install Frontend (React + Vite)

cd frontend
npm install
npm run dev


---

🔐 Keamanan

Proyek menggunakan model Zero Trust + Defense-In-Depth.

Keamanan mencakup:

HTTPS Only

Anti-replay timestamp

Webhook signature validation

CSP strict mode

NO sensitive data stored locally


Detail lengkap: SECURITY.md

> ⚠️ Catatan Penting:
Proyek ini tidak mengklaim harga Pi, tidak menetapkan nilai fiat, dan hanya memproses transaksi melalui Pi App Platform resmi.




---

🧪 Testing

Layer	Framework

Backend	PHPUnit / Pest
Frontend	Jest / Vitest
Integration	Playwright
Security	Bandit + Secret Scanner


Jalankan:

npm test
php artisan test


---

⚙️ CI/CD

Pipeline mencakup:

Build

Lint

Unit Test

Secret Scan

Security Audit

Auto-Release (Semantic Versioning)


Workflow ada di:

.github/workflows/ci.yml
.github/workflows/release.yml


---

🧰 Teknologi yang Digunakan

Pi SDK (Official)

React + Vite

Laravel 10 + PHP 8.1+

MySQL / MariaDB

Docker + Nginx

Github Actions CI/CD



---

🤝 Kontribusi

Kontribusi terbuka untuk developer Pi Network.

Silakan baca:

CONTRIBUTING.md

CODE_OF_CONDUCT.md



---

📌 Roadmap

Tahap	Status

v1.0 — Payment Core	🟢 Rilis
v1.2 — Dashboard Admin	🟡 On Development
v2.0 — Plugin API + Multi-Merchant	🔵 Planned



---

📄 Lisensi

Proyek dirilis di bawah lisensi:

MIT License — Free for Personal and Commercial Use


---

⭐ Status Proyek

> Stage: Public Beta
Community-maintained and actively improving.




---

💛 Kredit

Proyek ini dibuat untuk mendukung ekosistem Pi Network dan developer yang membangun ekonomi digital global yang adil dan terbuka.

🪙 Pi is for Utility — Not Speculation


---

🔗 Hubungi

Jika menemukan bug, laporkan melalui:

📍 Issues → GitHub: /issues
📧 security@picommerce.dev (hanya untuk kerentanan keamanan)


---

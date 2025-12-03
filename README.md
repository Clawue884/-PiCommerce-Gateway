<!-- README.md -->

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
| Audit Logging | 🟡 |
| CI/CD Pipeline (GitHub Actions) | 🟡 |
| Admin Dashboard | 🟡 |
| Semantic Versioning + Auto Release | 🟡 |
| Plugin Mode / Extend API | 🟡 |

> 🟡 = Dalam pengembangan  
> 🟢 = Stable  
> 🔵 = Planned upgrade

---

## 🏗 Arsitektur Sistem

```
Frontend (React + Pi SDK)
        ↓
REST API (Laravel 10)
        ↓
Webhook HMAC Verification
        ↓
Database (MySQL/MariaDB)
        ↓
Audit Log + Event Bus
```

📁 Dokumentasi lengkap tersedia di:

- `docs/architecture.md`
- `docs/api-reference.yml`

---

## 📡 Instalasi

### 1️⃣ Clone Repository

```sh
git clone https://github.com/Clawue884/-PiCommerce-Gateway
cd -PiCommerce-Gateway
```

---

### 2️⃣ Backend (Laravel)

```sh
cd backend
composer install
cp .env.example .env
php artisan migrate
php artisan serve
```

---

### 3️⃣ Frontend (React + Vite)

```sh
cd frontend
npm install
npm run dev
```

---

## 🔐 Keamanan

Proyek ini menerapkan sistem keamanan **Zero Trust Model** seperti:

- HTTPS only
- Anti-replay timestamp validation
- HMAC signature verification
- Secret rotation policy
- CSP Security headers

Detail keamanan: `SECURITY.md`

⚠️ Proyek ini **tidak menetapkan harga atau nilai fiat Pi.**  
Sistem hanya mengikuti standar developer resmi Pi Network.

---

## 🧪 Testing

```sh
npm test
php artisan test
```

| Layer | Framework |
|-------|-----------|
| Backend | PHPUnit / Pest |
| Frontend | Jest / Vitest |
| Integration | Playwright |
| Security | Secret Scanner |

---

## ⚙️ CI/CD

GitHub Actions mencakup:

- Build & Lint
- Unit Test
- Secret Scan
- Auto Release (Semantic Versioning)

File workflow ada di:

```
.github/workflows/ci.yml
.github/workflows/release.yml
```

---

## 📌 Roadmap

| Tahap | Status |
|-------|--------|
| v1.0 — Payment Core | 🟢 |
| v1.2 — Dashboard Admin | 🟡 |
| v2.0 — Multi-Merchant + Plugin API | 🔵 |

---

## 🤝 Kontribusi

Kontribusi terbuka.  
Sebelum commit, baca:

📄 `CONTRIBUTING.md`  
📄 `CODE_OF_CONDUCT.md`

---

## 📄 Lisensi

```
MIT License — Open for personal & commercial use.
```

---

### 🔗 Kontak

Issues & request: `/issues`  
Security Report (Private): `security@picommerce.dev`

---

> 🪙 *“Building decentralized commerce infrastructure for the Pi economy.”*

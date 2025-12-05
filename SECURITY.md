# SECURITY.md

## Ringkasan
Dokumen ini menjelaskan prosedur pelaporan kerentanan, kebijakan pengungkapan tanggung jawab, praktik pengamanan yang dianjurkan, dan proses respons insiden untuk proyek **PiCo Developer Platform**. Tujuan: membuat proyek yang audit-ready, kompatibel dengan standar PCT, dan aman untuk integrasi pihak ketiga.

---

## Kontak Pelaporan Kerentanan (Responsible Disclosure)
Jika Anda menemukan kerentanan keamanan, tolong laporkan secepatnya ke:
- Email: `security@pico.dev`
- Opsi aman (PGP): `PGP key fingerprint: <MASUKKAN_FINGERPRINT_PGP_DI_SINI>`
- Jika ingin anonim atau alternatif: buat isu privat di GitHub (template `security/`) atau kirim pesan ke Slack/ Mattermost tim security (lihat README internal).

**Kami berkomitmen menanggapi laporan dalam 72 jam kerja.**  
Jika ada laporan kritikal (RCE, exfiltration, private key leak), tandai sebagai `CRITICAL` pada subjek email.

---

## Policy & Timeline Tanggapan
- Acknowledgement: dalam 72 jam kerja.
- Triage & Klasifikasi: 3 hari kerja setelah acknowledgement.
- Mitigasi sementara / patch: prioritas tinggi dalam 7 hari kerja untuk issue kritikal, 14 hari untuk high, 30 hari untuk medium/low.
- Disclosure publik: hanya setelah patch tersedia atau sesuai kesepakatan dengan pelapor.

---

## Severity Classification (contoh)
- **CRITICAL** — Remote code execution, private key leak, full DB exfiltration, or supply-chain compromise.
- **HIGH** — Authentication bypass, major privilege escalation, funds manipulation.
- **MEDIUM** — Logic bugs that may lead to partial exposure or denial of service for subset user.
- **LOW** — Minor info disclosure, UI issues, best-practice violations.
- **INFO** — Harapan fitur, docs typo, non-security-related bugs.

---

## Cara Melaporkan (Minimal Info yang Dibutuhkan)
1. Deskripsi singkat dan langkah reproduksi (step-by-step) dengan perintah dan payload contoh.  
2. Dampak yang teramati (contoh: jumlah accounts, potensi kerugian).  
3. Versi perangkat lunak / commit hash.  
4. Bukti konsep (PoC) terbatas — jangan menyertakan exploit yang dapat dieksekusi di publik tanpa koreksi.  
5. Kontak reporter untuk follow-up.

---

## Kebijakan Pengungkapan
- Jangan mempublikasikan sebelum perbaikan atau izin tim.
- Jika pelapor ingin publikasi bersama, lakukan koordinasi timeline mitigasi.
- Kami menghargai laporan yang etis dan akan memberikan pengakuan (opsional) jika pelapor setuju.

---

## Praktik Pengembangan Aman (Checklist)
- **Secrets:** Jangan commit API keys/private keys ke repo. Gunakan secret manager (GitHub Secrets, Vault).
- **Dependency management:** Gunakan scanning dependency (Dependabot, Snyk) dan update rutin (minor/patch setiap minggu; major tiap sprint).
- **Static Analysis:** Jalankan ESLint/TSLint, Bandit (Python), CodeQL di pipeline CI. Fail build on high-severity alerts.
- **Dynamic Scanning:** Jalankan DAST pada staging (OWASP ZAP) sebelum produksi.
- **Container security:** Scan images (Trivy), gunakan minimal base images, sign images.
- **Secrets in CI logs:** Mask secrets, rotate keys saat terjadi eksposur.

---

## CI/CD & GitHub Actions
- Semua PR wajib lulus:
  - Unit tests & integration tests,
  - Linting,
  - CodeQL scan (atau SAST setara),
  - Dependency scan,
  - Container image scan (jika ada).
- Proteksi branch `main`: force push disabled, require 1–2 review approvals, CI success required.

---

## Runtime / Infrastruktur
- **Authentication**: OAuth2 / JWT short lived (default access token expiry ≤ 1 jam). Use refresh tokens securely.
- **Rate limiting**: implement rate limiting per IP & per API key.
- **Logging & Monitoring**: centralize logs (ELK/Datadog), enable alerting untuk abnormal activity (spike tx, failed auth).
- **Backup & Recovery**: regular backup database, test restore tiap 90 hari.
- **Network security**: use WAF, limit admin endpoints via IP allowlist / VPN.

---

## API Security Best Practices
- Validate dan sanitize semua input (server-side).
- HMAC / signature verification untuk webhook & PO proofs. Tidak cukup hanya bearer token.
- Reject unsigned requests or expired timestamps. Gunakan `X-Pi-Timestamp` + `X-Pi-Signature`.
- Use strict CORS policy for browser endpoints.
- Enforce TLS 1.2+ with strong ciphers.

---

## Webhook Handling (Keamanan Khusus)
- Validasi signature (HMAC SHA256) pada setiap webhook.
- Gunakan replay-protection: reject requests jika `timestamp` lebih dari ±5 menit.
- Simpan event idempotency keys untuk menghindari duplikat pemrosesan.

---

## Kode Tertentu (Contoh HMAC Validation)
> Contoh pseudocode:
```text
signature = HMAC_SHA256(secret, timestamp + '.' + body)
if signature != header['X-Pi-Signature'] -> reject 401
if abs(now - timestamp) > 5 minutes -> reject 400

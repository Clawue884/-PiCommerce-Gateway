---

🔐 Security Policy

📌 Supported Versions

Proyek ini menggunakan pendekatan keamanan bertingkat. Hanya versi berikut yang menerima pembaruan keamanan resmi:

Version	Status

main (development)	🟡 Under review
Stable Release (v1.x.x)	🟢 Supported
Legacy (v0.x.x)	🔴 No longer supported


Jika Anda menggunakan versi yang tidak lagi didukung, harap upgrade sesegera mungkin.


---

🛡 Reporting a Vulnerability

Kami sangat menghargai kontribusi Anda dalam menjaga keamanan proyek ini.
Jika Anda menemukan celah keamanan, jangan membuat issue publik.

Gunakan salah satu metode aman berikut:

📧 Email: security@[domain-proyek-anda].com
📨 PGPPubKey: (Opsional tambahkan jika ada)
🔏 Severity ratings follow: CVSS v3.1 Standard

Harap sertakan:

Deskripsi singkat kerentanan

Langkah untuk mereplikasi

Lingkungan sistem (OS, versi aplikasi, konfigurasi)

Dampak yang mungkin terjadi

Saran mitigasi (jika ada)


Kami berkomitmen untuk membalas dalam:

Severity	Response Time	Fix ETA

Critical	⏱ 24 hours	1–7 days
High	⏱ 48 hours	3–14 days
Medium	⏱ 5 days	7–30 days
Low	⏱ 7–14 days	30–90 days



---

🔒 Security Best Practices for Contributors

Jika Anda berkontribusi pada proyek ini, Anda WAJIB mengikuti standar keamanan berikut:

✔ Do

Gunakan HTTPS di semua endpoint API

Gunakan environment variables (.env), bukan hard-coded secrets

Pastikan dependency telah di-scan menggunakan:


npm audit
composer audit
yarn audit
cargo audit
pip audit

(Sesuaikan dengan teknologi proyek)

Gunakan commit signed GPG jika memungkinkan:


git commit -S -m "Your signed commit message"

Ikuti OWASP Top 10 + SANS CWE Guidelines


❌ Do Not

Jangan pernah mengunggah:

Password, API Keys, JWT secrets

Private Keys

Database dump atau credentials


Jangan mem-bypass mekanisme keamanan tanpa approval



---

🧪 Security Testing Guidelines

Semua kontribusi harus melalui:

Static Application Security Testing (SAST)

Dependency Security Scanning

Dynamic Application Security Testing (DAST) (optional for release)

Penetration Testing Before Official Release


Tools rekomendasi:

Category	Tools

SAST	SonarQube, CodeQL, Semgrep
Dependency Audit	Snyk, NPM Audit, GitHub Dependabot
DAST	OWASP ZAP, Burp Suite
Secret Scanner	TruffleHog, Gitleaks



---

🔏 Responsible Disclosure

Jika Anda menemukan celah keamanan dan melaporkannya secara benar, Anda akan mendapatkan penghargaan berupa:

🏅 Hall of Fame Contributor (Security)
📜 Certificate of Responsible Disclosure
(optional: ⭐ Bounty Reward jika program bug bounty aktif)

Kami menghormati peneliti keamanan yang beretika.


---

📜 Legal

Dengan berkontribusi atau melakukan penelitian keamanan pada repo ini, Anda setuju bahwa:

Tidak melakukan eksploitasi terhadap data pengguna nyata

Tidak menjual, menjadikan senjata, atau menyalahgunakan kerentanan

Tidak melakukan pelanggaran hukum siber lokal maupun internasional



---

🤝 Commitment

Kami berkomitmen menyediakan sistem yang:

Aman

Terukur

Terdesentralisasi (jika terkait blockchain)

Sesuai standar industri keamanan modern


Keamanan adalah tanggung jawab bersama.


---

🧩 Last Updated: {{tanggal update saat ini}}

(Anda dapat memperbarui tanggal setiap versi rilis)


---

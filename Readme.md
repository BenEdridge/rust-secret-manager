# Rust password/secret manager
- Single encrypted file via a PBKDF2 derived key or key file in bytes (64, 128, 256, 1024, n)
- Simple format based on encrypted DB
- Strongly typed Rust implementation
- Secure defaults (AES256-GCM, PBKDF2)
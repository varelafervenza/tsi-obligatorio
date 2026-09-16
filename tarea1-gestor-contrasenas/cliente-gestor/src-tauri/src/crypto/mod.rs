//! Derivación de clave (Argon2id) y cifrado AEAD (XChaCha20-Poly1305) de la bóveda.
//! Nada de este módulo debe registrar secretos en logs. RF-01, RF-16.

pub mod kdf;
pub mod cipher;

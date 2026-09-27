//! MFA local para desbloquear la bóveda: TOTP y WebAuthn/Windows Hello. RF-11 (parcial;
//! la elección de algoritmo de hash del lado del control central vive en ese servicio).

pub mod totp;
pub mod webauthn;

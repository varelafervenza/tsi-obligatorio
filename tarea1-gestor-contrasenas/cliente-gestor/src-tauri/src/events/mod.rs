//! Construcción y firma (JWS) de eventos de auditoría: alta, modificación, borrado
//! y cambio de contraseña maestra. Solo metadata firmada; nunca un secreto. RF-07, RF-08.

pub mod signer;

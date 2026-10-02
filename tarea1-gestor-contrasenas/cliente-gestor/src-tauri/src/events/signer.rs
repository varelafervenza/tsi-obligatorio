//! Firma RS256 de un evento. Los claims son solo metadata: nunca un secreto.

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::Serialize;

#[derive(Debug)]
pub enum ErrorFirma {
    Clave,
    Firma,
}

impl std::fmt::Display for ErrorFirma {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Clave => write!(f, "La clave privada del agente no se puede usar."),
            Self::Firma => write!(f, "No se pudo firmar el evento."),
        }
    }
}

#[derive(Serialize)]
struct Claims {
    agente_id: String,
    tipo: String,
    sistema: String,
    ts: String,
    /// python-jose rechaza el JWS si falta `exp`, aunque el central no lo compare.
    exp: u64,
}

pub fn firmar(
    pem_privado: &str,
    agente_id: &str,
    tipo: &str,
    sistema: &str,
    ts: &str,
    exp: u64,
) -> Result<String, ErrorFirma> {
    let clave = EncodingKey::from_rsa_pem(pem_privado.as_bytes()).map_err(|_| ErrorFirma::Clave)?;
    encode(
        &Header::new(Algorithm::RS256),
        &Claims {
            agente_id: agente_id.to_string(),
            tipo: tipo.to_string(),
            sistema: sistema.to_string(),
            ts: ts.to_string(),
            exp,
        },
        &clave,
    )
    .map_err(|_| ErrorFirma::Firma)
}

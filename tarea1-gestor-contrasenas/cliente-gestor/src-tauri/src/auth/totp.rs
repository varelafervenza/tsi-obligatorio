//! TOTP local (RFC 6238) para abrir la bóveda. RF-02. El secreto se guarda cifrado en el archivo.

use rand::RngCore;
use totp_rs::{Algorithm, Secret, TOTP};

pub fn generar() -> Result<(String, String), String> {
    let mut bytes = [0u8; 20];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    let totp = armar(&bytes)?;
    let secreto = totp.get_secret_base32();
    let uri = format!(
        "otpauth://totp/Gestor:boveda?secret={secreto}&issuer=Gestor&digits=6&period=30"
    );
    Ok((secreto, uri))
}

pub fn codigo_actual(secreto_b32: &str) -> Result<String, String> {
    armar_b32(secreto_b32)?
        .generate_current()
        .map_err(|e| e.to_string())
}

pub fn verificar(secreto_b32: &str, codigo: &str) -> bool {
    let Ok(totp) = armar_b32(secreto_b32.trim()) else {
        return false;
    };
    totp.check_current(codigo.trim()).unwrap_or(false)
}

fn armar_b32(secreto_b32: &str) -> Result<TOTP, String> {
    let bytes = Secret::Encoded(secreto_b32.to_string())
        .to_bytes()
        .map_err(|e| e.to_string())?;
    armar(&bytes)
}

fn armar(secret: &[u8]) -> Result<TOTP, String> {
    TOTP::new(Algorithm::SHA1, 6, 1, 30, secret.to_vec()).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn el_codigo_actual_verifica_y_otro_no() {
        let (secreto, uri) = generar().unwrap();
        assert!(uri.starts_with("otpauth://totp/"));
        let codigo = codigo_actual(&secreto).unwrap();
        assert!(verificar(&secreto, &codigo));
        let malo = if codigo == "000000" { "111111" } else { "000000" };
        assert!(!verificar(&secreto, malo));
    }
}

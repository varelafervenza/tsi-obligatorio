//! Derivación de la clave de bóveda a partir de la contraseña maestra con Argon2id.
//! Parámetros por defecto: perfil interactivo OWASP (memoria 19 MiB, 2 iteraciones).
//! El salt es único por bóveda y lo guarda quien abre el archivo; acá no se persiste.

use argon2::{Algorithm, Argon2, Params, Version};
use rand::rngs::OsRng;
use rand::RngCore;

/// 19 MiB. Por debajo de esto Argon2id deja de ser memory-hard frente a GPU.
pub const MEMORIA_KIB_BOVEDA: u32 = 19 * 1024;
pub const ITERACIONES_BOVEDA: u32 = 2;
pub const PARALELISMO_BOVEDA: u32 = 1;
const LONGITUD_CLAVE: usize = 32;
const LONGITUD_SALT: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParametrosKdf {
    pub memoria_kib: u32,
    pub iteraciones: u32,
    pub paralelismo: u32,
}

impl ParametrosKdf {
    pub fn boveda() -> Self {
        Self {
            memoria_kib: MEMORIA_KIB_BOVEDA,
            iteraciones: ITERACIONES_BOVEDA,
            paralelismo: PARALELISMO_BOVEDA,
        }
    }
}

#[derive(Debug)]
pub enum ErrorKdf {
    ParametrosInvalidos,
    Derivacion,
}

/// Salt aleatorio de 16 bytes. Se guarda en claro junto al archivo de la bóveda.
pub fn generar_salt() -> [u8; LONGITUD_SALT] {
    let mut salt = [0u8; LONGITUD_SALT];
    OsRng.fill_bytes(&mut salt);
    salt
}

/// Deriva 32 bytes para XChaCha20-Poly1305. La contraseña no se guarda ni se loguea.
pub fn derivar_clave(
    contrasena_maestra: &str,
    salt: &[u8],
    parametros: ParametrosKdf,
) -> Result<[u8; LONGITUD_CLAVE], ErrorKdf> {
    if contrasena_maestra.is_empty() || salt.len() < 8 {
        return Err(ErrorKdf::ParametrosInvalidos);
    }
    let params = Params::new(
        parametros.memoria_kib,
        parametros.iteraciones,
        parametros.paralelismo,
        Some(LONGITUD_CLAVE),
    )
    .map_err(|_| ErrorKdf::ParametrosInvalidos)?;
    let argon = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut clave = [0u8; LONGITUD_CLAVE];
    argon
        .hash_password_into(contrasena_maestra.as_bytes(), salt, &mut clave)
        .map_err(|_| ErrorKdf::Derivacion)?;
    Ok(clave)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params_test() -> ParametrosKdf {
        // Mínimo que acepta Argon2: los tests no usan 19 MiB.
        ParametrosKdf {
            memoria_kib: 8,
            iteraciones: 1,
            paralelismo: 1,
        }
    }

    #[test]
    fn misma_maestra_y_salt_dan_la_misma_clave() {
        let salt = [7u8; 16];
        let a = derivar_clave("correct horse", &salt, params_test()).unwrap();
        let b = derivar_clave("correct horse", &salt, params_test()).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn otra_maestra_u_otro_salt_cambian_la_clave() {
        let salt = [7u8; 16];
        let base = derivar_clave("correct horse", &salt, params_test()).unwrap();
        let otra = derivar_clave("correct horse!", &salt, params_test()).unwrap();
        let otro_salt = derivar_clave("correct horse", &[8u8; 16], params_test()).unwrap();
        assert_ne!(base, otra);
        assert_ne!(base, otro_salt);
    }

    #[test]
    fn rechaza_maestra_vacia_o_salt_corto() {
        let params = params_test();
        assert!(derivar_clave("", &[1u8; 16], params).is_err());
        assert!(derivar_clave("maestra", &[1u8; 4], params).is_err());
    }

    #[test]
    fn salt_generado_no_es_cero() {
        let salt = generar_salt();
        assert_ne!(salt, [0u8; 16]);
    }
}

//! Cifrado AEAD de cada secreto de la bóveda con XChaCha20-Poly1305.
//! El blob es nonce (24 bytes) || ciphertext || tag. Si el tag no cierra, no hay texto plano.

use chacha20poly1305::aead::{Aead, AeadCore, KeyInit, OsRng};
use chacha20poly1305::{XChaCha20Poly1305, XNonce};

const NONCE_LEN: usize = 24;

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorCifrado {
    Manipulado,
}

/// Cifra `plano`. Cada llamada usa un nonce nuevo; no reutilizar la clave con el mismo nonce.
pub fn cifrar(clave: &[u8; 32], plano: &[u8]) -> Result<Vec<u8>, ErrorCifrado> {
    let cipher = XChaCha20Poly1305::new(clave.into());
    let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
    let cifrado = cipher
        .encrypt(&nonce, plano)
        .map_err(|_| ErrorCifrado::Manipulado)?;
    let mut blob = Vec::with_capacity(NONCE_LEN + cifrado.len());
    blob.extend_from_slice(&nonce);
    blob.extend_from_slice(&cifrado);
    Ok(blob)
}

/// Descifra un blob producido por `cifrar`. Falla si cambiaron un solo byte o la clave no es la de origen.
pub fn descifrar(clave: &[u8; 32], blob: &[u8]) -> Result<Vec<u8>, ErrorCifrado> {
    if blob.len() <= NONCE_LEN {
        return Err(ErrorCifrado::Manipulado);
    }
    let (nonce_bytes, cifrado) = blob.split_at(NONCE_LEN);
    let nonce = XNonce::from_slice(nonce_bytes);
    let cipher = XChaCha20Poly1305::new(clave.into());
    cipher
        .decrypt(nonce, cifrado)
        .map_err(|_| ErrorCifrado::Manipulado)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clave() -> [u8; 32] {
        [9u8; 32]
    }

    #[test]
    fn ida_y_vuelta() {
        let blob = cifrar(&clave(), b"secreto-banco").unwrap();
        let plano = descifrar(&clave(), &blob).unwrap();
        assert_eq!(plano, b"secreto-banco");
    }

    #[test]
    fn dos_cifrados_del_mismo_texto_no_coinciden() {
        let a = cifrar(&clave(), b"secreto-banco").unwrap();
        let b = cifrar(&clave(), b"secreto-banco").unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn un_byte_alterado_no_descifra() {
        let mut blob = cifrar(&clave(), b"secreto-banco").unwrap();
        let ultimo = blob.len() - 1;
        blob[ultimo] ^= 0x01;
        assert_eq!(descifrar(&clave(), &blob), Err(ErrorCifrado::Manipulado));
    }

    #[test]
    fn otra_clave_no_descifra() {
        let blob = cifrar(&clave(), b"secreto-banco").unwrap();
        let mut otra = clave();
        otra[0] ^= 0x01;
        assert_eq!(descifrar(&otra, &blob), Err(ErrorCifrado::Manipulado));
    }
}

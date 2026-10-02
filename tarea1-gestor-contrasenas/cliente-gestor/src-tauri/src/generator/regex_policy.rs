//! Comprueba si una contraseña cumple la expresión regular del sistema.
//! La generación usa rechazo: se arma un candidato y se descarta si no entra.

use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorRegex {
    Invalida,
}

pub fn cumple(expresion: &str, secreto: &str) -> Result<bool, ErrorRegex> {
    if expresion.is_empty() {
        return Ok(true);
    }
    let re = Regex::new(expresion).map_err(|_| ErrorRegex::Invalida)?;
    Ok(re.is_match(secreto))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vacia_acepta_cualquier_secreto() {
        assert_eq!(cumple("", "hola").unwrap(), true);
    }

    #[test]
    fn solo_digitos() {
        assert_eq!(cumple(r"^[0-9]{4}$", "1234").unwrap(), true);
        assert_eq!(cumple(r"^[0-9]{4}$", "12ab").unwrap(), false);
    }

    #[test]
    fn expresion_rota() {
        assert_eq!(cumple("(", "1234"), Err(ErrorRegex::Invalida));
    }
}

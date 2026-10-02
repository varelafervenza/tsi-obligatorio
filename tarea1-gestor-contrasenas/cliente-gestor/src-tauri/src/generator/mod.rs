//! Generador de contraseñas y de frases tipo XKCD 936. RF-04, RF-05.
//! Si hay una expresión regular, se generan candidatos hasta cumplirla.

pub mod regex_policy;

use rand::Rng;
use rand::seq::SliceRandom;

const MINUSCULAS: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const MAYUSCULAS: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const DIGITOS: &[u8] = b"23456789";
const SIMBOLOS: &[u8] = b"!@#$%^&*-_=+?";
const INTENTOS_REGEX: usize = 80;

const PALABRAS: &[&str] = &[
    "sol", "rio", "mesa", "luna", "pan", "mar", "bosque", "llave", "nube", "puente",
    "casa", "tren", "faro", "campo", "libro", "piedra", "viento", "plaza", "barco", "monte",
    "agua", "fuego", "tierra", "norte", "sur", "este", "oeste", "verde", "rojo", "azul",
    "gato", "perro", "ave", "pez", "lobo", "oso", "cabra", "flor", "arbol", "semilla",
    "camino", "puerta", "ventana", "reloj", "mapa", "brujula", "ancla", "vela", "remo", "isla",
    "cobre", "plata", "arena", "sal", "miel", "trigo", "maiz", "uva", "limon", "naranja",
];

#[derive(Debug)]
pub enum ErrorGenerador {
    SinCaracteres,
    LongitudCorta,
    RegexInvalida,
    RegexIncumplible,
}

impl std::fmt::Display for ErrorGenerador {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SinCaracteres => write!(f, "Elegí al menos un tipo de carácter."),
            Self::LongitudCorta => {
                write!(f, "La longitud no alcanza para los tipos de carácter elegidos.")
            }
            Self::RegexInvalida => write!(f, "La expresión regular no es válida."),
            Self::RegexIncumplible => write!(
                f,
                "No se pudo armar una contraseña que cumpla la expresión. Ajustá la longitud o los caracteres."
            ),
        }
    }
}

pub struct PedidoGeneracion {
    pub modo: String,
    pub longitud: u32,
    pub minusculas: bool,
    pub mayusculas: bool,
    pub digitos: bool,
    pub simbolos: bool,
    pub regex: String,
}

pub fn generar(pedido: &PedidoGeneracion) -> Result<String, ErrorGenerador> {
    for _ in 0..INTENTOS_REGEX {
        let candidato = if pedido.modo == "passphrase" {
            frase(pedido.longitud)?
        } else {
            aleatoria(pedido)?
        };
        match regex_policy::cumple(&pedido.regex, &candidato) {
            Ok(true) => return Ok(candidato),
            Ok(false) => continue,
            Err(regex_policy::ErrorRegex::Invalida) => return Err(ErrorGenerador::RegexInvalida),
        }
    }
    if pedido.regex.is_empty() {
        Err(ErrorGenerador::SinCaracteres)
    } else {
        Err(ErrorGenerador::RegexIncumplible)
    }
}

fn aleatoria(pedido: &PedidoGeneracion) -> Result<String, ErrorGenerador> {
    let mut grupos: Vec<&[u8]> = Vec::new();
    if pedido.minusculas {
        grupos.push(MINUSCULAS);
    }
    if pedido.mayusculas {
        grupos.push(MAYUSCULAS);
    }
    if pedido.digitos {
        grupos.push(DIGITOS);
    }
    if pedido.simbolos {
        grupos.push(SIMBOLOS);
    }
    if grupos.is_empty() {
        return Err(ErrorGenerador::SinCaracteres);
    }
    let longitud = pedido.longitud.max(1) as usize;
    if longitud < grupos.len() {
        return Err(ErrorGenerador::LongitudCorta);
    }
    let mut rng = rand::rngs::OsRng;
    let mut chars: Vec<u8> = grupos.iter().map(|g| g[rng.gen_range(0..g.len())]).collect();
    let alfabeto: Vec<u8> = grupos.iter().flat_map(|g| g.iter().copied()).collect();
    while chars.len() < longitud {
        chars.push(alfabeto[rng.gen_range(0..alfabeto.len())]);
    }
    chars.shuffle(&mut rng);
    Ok(String::from_utf8(chars).expect("el alfabeto es ASCII"))
}

fn frase(palabras: u32) -> Result<String, ErrorGenerador> {
    let n = palabras.clamp(3, 8) as usize;
    let mut rng = rand::rngs::OsRng;
    let mut elegidas = Vec::with_capacity(n);
    for _ in 0..n {
        elegidas.push(PALABRAS[rng.gen_range(0..PALABRAS.len())]);
    }
    Ok(elegidas.join("-"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pedido_base() -> PedidoGeneracion {
        PedidoGeneracion {
            modo: "aleatoria".into(),
            longitud: 12,
            minusculas: true,
            mayusculas: true,
            digitos: true,
            simbolos: false,
            regex: String::new(),
        }
    }

    #[test]
    fn aleatoria_respeta_longitud_y_tiene_un_digito() {
        let mut pedido = pedido_base();
        pedido.longitud = 10;
        pedido.minusculas = false;
        pedido.mayusculas = false;
        pedido.simbolos = false;
        let clave = generar(&pedido).unwrap();
        assert_eq!(clave.len(), 10);
        assert!(clave.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn frase_tiene_cuatro_palabras() {
        let mut pedido = pedido_base();
        pedido.modo = "passphrase".into();
        pedido.longitud = 4;
        let frase = generar(&pedido).unwrap();
        assert_eq!(frase.split('-').count(), 4);
    }

    #[test]
    fn cumple_regex_de_cuatro_digitos() {
        let mut pedido = pedido_base();
        pedido.longitud = 4;
        pedido.minusculas = false;
        pedido.mayusculas = false;
        pedido.simbolos = false;
        pedido.regex = r"^[0-9]{4}$".into();
        let clave = generar(&pedido).unwrap();
        assert!(regex_policy::cumple(&pedido.regex, &clave).unwrap());
    }

    #[test]
    fn regex_imposible_falla() {
        let mut pedido = pedido_base();
        pedido.longitud = 4;
        pedido.regex = r"^$".into();
        assert!(matches!(generar(&pedido), Err(ErrorGenerador::RegexIncumplible)));
    }
}

//! Identidad del agente, firma del evento y cola local si el control central no responde.
//! RF-07, RF-08, RNF-01. El cuerpo no lleva secretos.

mod signer;

use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding};
use rsa::{RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};

const URL_DEFECTO: &str = "http://localhost:8001/api/events/";

#[derive(Debug)]
pub enum ErrorEvento {
    Disco(String),
    Rechazado(String),
}

impl std::fmt::Display for ErrorEvento {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Disco(m) => write!(f, "{m}"),
            Self::Rechazado(m) => write!(f, "{m}"),
        }
    }
}

#[derive(Clone, Serialize)]
pub struct EstadoAuditoria {
    pub agente_id: String,
    pub url: String,
    pub carpeta_claves: String,
    pub en_cola: usize,
    pub ultimo: String,
}

#[derive(Serialize, Deserialize)]
struct ConfigAuditoria {
    url: String,
    carpeta_claves: String,
}

#[derive(Serialize, Deserialize)]
struct CuerpoEvento {
    tipo: String,
    sistema: String,
    agente_id: String,
    timestamp: String,
    firma_jws: String,
}

pub fn estado(dir: &Path) -> Result<EstadoAuditoria, ErrorEvento> {
    let (agente_id, _) = asegurar_identidad(dir)?;
    let config = leer_config(dir);
    Ok(EstadoAuditoria {
        agente_id,
        url: config.url,
        carpeta_claves: config.carpeta_claves,
        en_cola: contar_cola(dir),
        ultimo: fs::read_to_string(dir.join("ultimo.txt")).unwrap_or_default(),
    })
}

pub fn guardar_config(dir: &Path, url: &str, carpeta_claves: &str) -> Result<String, ErrorEvento> {
    fs::create_dir_all(dir).map_err(|e| ErrorEvento::Disco(e.to_string()))?;
    let config = ConfigAuditoria {
        url: if url.trim().is_empty() {
            URL_DEFECTO.to_string()
        } else {
            url.trim().to_string()
        },
        carpeta_claves: carpeta_claves.trim().to_string(),
    };
    let texto = serde_json::to_string_pretty(&config).map_err(|e| ErrorEvento::Disco(e.to_string()))?;
    fs::write(dir.join("auditoria.json"), texto).map_err(|e| ErrorEvento::Disco(e.to_string()))?;
    if config.carpeta_claves.is_empty() {
        return Ok("Falta la carpeta de claves públicas del control central.".into());
    }
    instalar_clave(dir, Path::new(&config.carpeta_claves))
}

pub fn instalar_clave(dir: &Path, carpeta: &Path) -> Result<String, ErrorEvento> {
    let (agente_id, _) = asegurar_identidad(dir)?;
    let publica = fs::read_to_string(dir.join("agente.pub.pem")).map_err(|e| ErrorEvento::Disco(e.to_string()))?;
    fs::create_dir_all(carpeta).map_err(|e| ErrorEvento::Disco(e.to_string()))?;
    let destino = carpeta.join(format!("{agente_id}.pub.pem"));
    fs::write(&destino, publica).map_err(|e| ErrorEvento::Disco(e.to_string()))?;
    Ok(format!(
        "Clave pública de {agente_id} copiada en {}.",
        destino.to_string_lossy()
    ))
}

/// Firma y envía. Si no hay red, deja el evento en `cola-eventos.jsonl` y no pierde el alta local.
pub fn publicar(dir: &Path, tipo: &str, sistema: &str) -> Result<String, ErrorEvento> {
    let (agente_id, pem) = asegurar_identidad(dir)?;
    let config = leer_config(dir);
    let sistema = if sistema.trim().is_empty() { "boveda" } else { sistema.trim() };
    let ahora = chrono::Utc::now();
    let ts = ahora.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let exp = (ahora.timestamp() + 600).max(0) as u64;
    let firma = signer::firmar(&pem, &agente_id, tipo, sistema, &ts, exp).map_err(|e| ErrorEvento::Disco(e.to_string()))?;
    let cuerpo = CuerpoEvento {
        tipo: tipo.to_string(),
        sistema: sistema.to_string(),
        agente_id,
        timestamp: ts,
        firma_jws: firma,
    };
    let mut pendientes = leer_cola(dir)?;
    let mut siguen = Vec::new();
    for viejo in pendientes.drain(..) {
        match postear(&config.url, &viejo) {
            ResultadoPost::Enviado | ResultadoPost::Rechazado(_) => {}
            ResultadoPost::Reintentar => siguen.push(viejo),
        }
    }
    match postear(&config.url, &cuerpo) {
        ResultadoPost::Enviado => {
            escribir_cola(dir, &siguen)?;
            if siguen.is_empty() {
                Ok("Evento enviado al control central.".into())
            } else {
                Ok(format!("Evento enviado. Quedan {} en cola.", siguen.len()))
            }
        }
        ResultadoPost::Reintentar => {
            siguen.push(cuerpo);
            escribir_cola(dir, &siguen)?;
            Ok(format!(
                "Sin conexión con el control central. El evento quedó en cola ({}).",
                siguen.len()
            ))
        }
        ResultadoPost::Rechazado(codigo) => {
            escribir_cola(dir, &siguen)?;
            Err(ErrorEvento::Rechazado(format!(
                "El control central rechazó el evento ({codigo})."
            )))
        }
    }
}

enum ResultadoPost {
    Enviado,
    Reintentar,
    Rechazado(u16),
}

fn postear(url: &str, cuerpo: &CuerpoEvento) -> ResultadoPost {
    let agent = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(4))
        .build();
    match agent.post(url).send_json(cuerpo) {
        Ok(resp) if (200..300).contains(&resp.status()) => ResultadoPost::Enviado,
        Ok(_) => ResultadoPost::Reintentar,
        Err(ureq::Error::Status(code, _)) if code >= 500 => ResultadoPost::Reintentar,
        Err(ureq::Error::Status(code, _)) => ResultadoPost::Rechazado(code),
        Err(_) => ResultadoPost::Reintentar,
    }
}

fn asegurar_identidad(dir: &Path) -> Result<(String, String), ErrorEvento> {
    fs::create_dir_all(dir).map_err(|e| ErrorEvento::Disco(e.to_string()))?;
    let privada = dir.join("agente.pem");
    let publica = dir.join("agente.pub.pem");
    let id_path = dir.join("agente.id");
    if privada.is_file() && publica.is_file() && id_path.is_file() {
        let id = fs::read_to_string(&id_path).map_err(|e| ErrorEvento::Disco(e.to_string()))?;
        let pem = fs::read_to_string(&privada).map_err(|e| ErrorEvento::Disco(e.to_string()))?;
        return Ok((id.trim().to_string(), pem));
    }
    let mut rng = rand::rngs::OsRng;
    let clave = RsaPrivateKey::new(&mut rng, 2048).map_err(|e| ErrorEvento::Disco(e.to_string()))?;
    let pem = clave
        .to_pkcs8_pem(LineEnding::LF)
        .map_err(|e| ErrorEvento::Disco(e.to_string()))?
        .to_string();
    let publica_pem = RsaPublicKey::from(&clave)
        .to_public_key_pem(LineEnding::LF)
        .map_err(|e| ErrorEvento::Disco(e.to_string()))?;
    let agente_id = format!("agente-{}", hex_corto());
    fs::write(&privada, &pem).map_err(|e| ErrorEvento::Disco(e.to_string()))?;
    fs::write(&publica, publica_pem).map_err(|e| ErrorEvento::Disco(e.to_string()))?;
    fs::write(&id_path, &agente_id).map_err(|e| ErrorEvento::Disco(e.to_string()))?;
    Ok((agente_id, pem))
}

fn hex_corto() -> String {
    let mut bytes = [0u8; 4];
    rand::RngCore::fill_bytes(&mut rand::rngs::OsRng, &mut bytes);
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn leer_config(dir: &Path) -> ConfigAuditoria {
    let path = dir.join("auditoria.json");
    fs::read_to_string(path)
        .ok()
        .and_then(|t| serde_json::from_str(&t).ok())
        .unwrap_or(ConfigAuditoria {
            url: URL_DEFECTO.into(),
            carpeta_claves: String::new(),
        })
}

fn cola_path(dir: &Path) -> PathBuf {
    dir.join("cola-eventos.jsonl")
}

fn leer_cola(dir: &Path) -> Result<Vec<CuerpoEvento>, ErrorEvento> {
    let path = cola_path(dir);
    if !path.is_file() {
        return Ok(Vec::new());
    }
    let texto = fs::read_to_string(path).map_err(|e| ErrorEvento::Disco(e.to_string()))?;
    let mut salida = Vec::new();
    for linea in texto.lines().filter(|l| !l.trim().is_empty()) {
        let cuerpo = serde_json::from_str(linea).map_err(|e| ErrorEvento::Disco(e.to_string()))?;
        salida.push(cuerpo);
    }
    Ok(salida)
}

fn escribir_cola(dir: &Path, eventos: &[CuerpoEvento]) -> Result<(), ErrorEvento> {
    let path = cola_path(dir);
    if eventos.is_empty() {
        let _ = fs::remove_file(path);
        return Ok(());
    }
    let mut texto = String::new();
    for evento in eventos {
        texto.push_str(&serde_json::to_string(evento).map_err(|e| ErrorEvento::Disco(e.to_string()))?);
        texto.push('\n');
    }
    fs::write(path, texto).map_err(|e| ErrorEvento::Disco(e.to_string()))
}

fn contar_cola(dir: &Path) -> usize {
    leer_cola(dir).map(|v| v.len()).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

    #[test]
    fn la_firma_trae_los_claims_que_verifica_el_central() {
        let dir = std::env::temp_dir().join(format!("eventos-firma-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let (id, pem) = asegurar_identidad(&dir).unwrap();
        let publica = fs::read_to_string(dir.join("agente.pub.pem")).unwrap();
        let ts = "2026-10-02T02:00:00Z";
        let firma = signer::firmar(&pem, &id, "alta_credencial", "Banco", ts, 2_000_000_000).unwrap();
        let mut validacion = Validation::new(Algorithm::RS256);
        validacion.validate_exp = false;
        validacion.required_spec_claims.clear();
        let datos = decode::<serde_json::Value>(
            &firma,
            &DecodingKey::from_rsa_pem(publica.as_bytes()).unwrap(),
            &validacion,
        )
        .unwrap();
        assert_eq!(datos.claims["agente_id"], id);
        assert_eq!(datos.claims["tipo"], "alta_credencial");
        assert_eq!(datos.claims["sistema"], "Banco");
        assert_eq!(datos.claims["ts"], ts);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn sin_red_el_evento_queda_en_cola() {
        let dir = std::env::temp_dir().join(format!("eventos-cola-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("auditoria.json"),
            r#"{"url":"http://127.0.0.1:9/api/events/","carpeta_claves":""}"#,
        )
        .unwrap();
        let mensaje = publicar(&dir, "alta_credencial", "Banco").unwrap();
        assert!(mensaje.contains("cola"));
        assert_eq!(contar_cola(&dir), 1);
        let _ = fs::remove_dir_all(&dir);
    }
}

//! Comandos que llama la ventana. La clave de la bóveda vive solo en este proceso.

use std::collections::HashMap;
use std::path::Path;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

use tauri::{AppHandle, Manager, State};

use super::store::{Boveda, CredencialDetalle, CredencialResumen, ErrorBoveda, Politica};

pub struct EstadoBoveda(Mutex<Option<Boveda>>);

impl EstadoBoveda {
    pub fn nuevo() -> Self {
        Self(Mutex::new(None))
    }
}

#[derive(serde::Serialize)]
pub struct EstadoSesion {
    pub abierta: bool,
    pub ruta: Option<String>,
}

#[tauri::command]
pub fn ruta_por_defecto(app: AppHandle) -> Result<String, String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join("boveda-prueba.sqlite").to_string_lossy().into_owned())
}

#[tauri::command]
pub fn crear_boveda(
    estado: State<'_, EstadoBoveda>,
    ruta: String,
    maestra: String,
) -> Result<(), String> {
    let boveda = Boveda::crear(Path::new(&ruta), &maestra).map_err(|e| e.to_string())?;
    let mut guard = bloqueo(&estado);
    *guard = Some(boveda);
    Ok(())
}

#[tauri::command]
pub async fn abrir_boveda(
    app: AppHandle,
    window: tauri::WebviewWindow,
    estado: State<'_, EstadoBoveda>,
    ruta: String,
    maestra: String,
    codigo: Option<String>,
) -> Result<Vec<String>, String> {
    if Boveda::hello_requerido(Path::new(&ruta)).map_err(|e| e.to_string())? {
        let hwnd = window.hwnd().map_err(|e| e.to_string())?.0 as isize;
        let confirmado = tauri::async_runtime::spawn_blocking(move || {
            crate::auth::webauthn::verificar(hwnd)
        })
        .await
        .map_err(|e| e.to_string())??;
        if !confirmado {
            return Err("Windows Hello no fue confirmado.".into());
        }
    }
    let codigo = codigo.as_deref().map(str::trim).filter(|valor| !valor.is_empty());
    match Boveda::abrir_con(Path::new(&ruta), &maestra, codigo) {
        Ok(boveda) => {
            limpiar_fallos(&ruta);
            let sistemas = boveda.sistemas_vencidos().unwrap_or_default();
            for sistema in &sistemas {
                auditar(&app, "vencimiento_credencial", sistema);
            }
            let mut guard = bloqueo(&estado);
            *guard = Some(boveda);
            Ok(sistemas)
        }
        Err(ErrorBoveda::MaestraIncorrecta) => {
            auditar(&app, "intento_fallido_maestra", "boveda");
            let espera = registrar_fallo(&ruta);
            std::thread::sleep(Duration::from_secs(espera));
            Err(ErrorBoveda::MaestraIncorrecta.to_string())
        }
        Err(ErrorBoveda::TotpInvalido) => {
            let espera = registrar_fallo(&ruta);
            std::thread::sleep(Duration::from_secs(espera));
            Err(ErrorBoveda::TotpInvalido.to_string())
        }
        Err(error) => Err(error.to_string()),
    }
}

#[derive(serde::Serialize)]
pub struct EnrollTotp {
    pub secreto: String,
    pub otpauth_uri: String,
}

#[tauri::command]
pub fn enrolar_totp(estado: State<'_, EstadoBoveda>) -> Result<EnrollTotp, String> {
    con_boveda(&estado, |boveda| {
        boveda.enrolar_totp().map(|(secreto, otpauth_uri)| EnrollTotp {
            secreto,
            otpauth_uri,
        })
    })
}

#[tauri::command]
pub fn confirmar_totp(estado: State<'_, EstadoBoveda>, codigo: String) -> Result<(), String> {
    con_boveda(&estado, |boveda| boveda.confirmar_totp(&codigo))
}

#[tauri::command]
pub fn estado_windows_hello(estado: State<'_, EstadoBoveda>) -> Result<bool, String> {
    con_boveda(&estado, |boveda| Boveda::hello_requerido(boveda.ruta()))
}

#[tauri::command]
pub fn configurar_windows_hello(estado: State<'_, EstadoBoveda>, activar: bool) -> Result<(), String> {
    if activar {
        crate::auth::webauthn::registrar()?;
    }
    con_boveda(&estado, |boveda| boveda.configurar_hello(activar))
}

#[tauri::command]
pub fn cerrar_boveda(estado: State<'_, EstadoBoveda>) -> Result<(), String> {
    let mut guard = bloqueo(&estado);
    *guard = None;
    Ok(())
}

#[tauri::command]
pub fn estado_boveda(estado: State<'_, EstadoBoveda>) -> Result<EstadoSesion, String> {
    let guard = bloqueo(&estado);
    Ok(match guard.as_ref() {
        Some(boveda) => EstadoSesion {
            abierta: true,
            ruta: Some(boveda.ruta().to_string_lossy().into_owned()),
        },
        None => EstadoSesion {
            abierta: false,
            ruta: None,
        },
    })
}

#[tauri::command]
pub fn listar_credenciales(estado: State<'_, EstadoBoveda>) -> Result<Vec<CredencialResumen>, String> {
    con_boveda(&estado, |boveda| boveda.listar())
}

#[tauri::command]
pub fn alta_credencial(
    app: AppHandle,
    estado: State<'_, EstadoBoveda>,
    sistema: String,
    usuario: String,
    secreto: String,
    notas: String,
    categoria: String,
) -> Result<CredencialResumen, String> {
    let creada = con_boveda(&estado, |boveda| {
        boveda.alta(&sistema, &usuario, &secreto, &notas, &categoria)
    })?;
    auditar(&app, "alta_credencial", &creada.sistema);
    Ok(creada)
}

#[tauri::command]
pub fn obtener_credencial(
    estado: State<'_, EstadoBoveda>,
    id: i64,
) -> Result<CredencialDetalle, String> {
    con_boveda(&estado, |boveda| boveda.obtener(id))
}

#[tauri::command]
pub fn modificar_credencial(
    app: AppHandle,
    estado: State<'_, EstadoBoveda>,
    id: i64,
    sistema: String,
    usuario: String,
    secreto: String,
    notas: String,
    categoria: String,
) -> Result<(), String> {
    con_boveda(&estado, |boveda| {
        boveda.modificar(id, &sistema, &usuario, &secreto, &notas, &categoria)
    })?;
    auditar(&app, "modificacion_credencial", sistema.trim());
    Ok(())
}

#[tauri::command]
pub fn borrar_credencial(app: AppHandle, estado: State<'_, EstadoBoveda>, id: i64, sistema: String) -> Result<(), String> {
    con_boveda(&estado, |boveda| boveda.borrar(id))?;
    auditar(&app, "borrado_credencial", sistema.trim());
    Ok(())
}

#[tauri::command]
pub fn generar_contrasena(
    modo: String,
    longitud: u32,
    minusculas: bool,
    mayusculas: bool,
    digitos: bool,
    simbolos: bool,
    regex: String,
) -> Result<String, String> {
    crate::generator::generar(&crate::generator::PedidoGeneracion {
        modo,
        longitud,
        minusculas,
        mayusculas,
        digitos,
        simbolos,
        regex,
    })
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn guardar_politica(
    estado: State<'_, EstadoBoveda>,
    sistema: String,
    modo: String,
    longitud: u32,
    minusculas: bool,
    mayusculas: bool,
    digitos: bool,
    simbolos: bool,
    regex: String,
    historial: u32,
    dias_validez: u32,
    categoria: String,
) -> Result<(), String> {
    con_boveda(&estado, |boveda| {
        boveda.guardar_politica(&Politica {
            sistema,
            modo,
            longitud,
            minusculas,
            mayusculas,
            digitos,
            simbolos,
            regex,
            historial,
            dias_validez,
            categoria,
        })
    })
}

#[tauri::command]
pub fn marcar_favorito(estado: State<'_, EstadoBoveda>, id: i64, favorito: bool) -> Result<(), String> {
    con_boveda(&estado, |boveda| boveda.marcar_favorito(id, favorito))
}

#[tauri::command]
pub fn exportar_boveda(estado: State<'_, EstadoBoveda>, ruta: String, transporte: String) -> Result<(), String> {
    con_boveda(&estado, |boveda| boveda.exportar(Path::new(&ruta), &transporte))
}

#[tauri::command]
pub fn importar_boveda(estado: State<'_, EstadoBoveda>, ruta: String, transporte: String) -> Result<usize, String> {
    con_boveda(&estado, |boveda| boveda.importar(Path::new(&ruta), &transporte))
}

#[tauri::command]
pub fn obtener_politica(
    estado: State<'_, EstadoBoveda>,
    sistema: String,
) -> Result<Option<Politica>, String> {
    con_boveda(&estado, |boveda| boveda.obtener_politica(&sistema))
}

#[tauri::command]
pub fn cambiar_maestra(
    app: AppHandle,
    estado: State<'_, EstadoBoveda>,
    actual: String,
    nueva: String,
) -> Result<(), String> {
    con_boveda_mut(&estado, |boveda| boveda.cambiar_maestra(&actual, &nueva))?;
    auditar(&app, "cambio_maestra", "boveda");
    Ok(())
}

#[tauri::command]
pub fn estado_auditoria(app: AppHandle) -> Result<crate::events::EstadoAuditoria, String> {
    let dir = dir_auditoria(&app)?;
    crate::events::estado(&dir).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn guardar_auditoria(app: AppHandle, url: String, carpeta_claves: String) -> Result<String, String> {
    let dir = dir_auditoria(&app)?;
    let mensaje = crate::events::guardar_config(&dir, &url, &carpeta_claves).map_err(|e| e.to_string())?;
    let _ = std::fs::write(dir.join("ultimo.txt"), &mensaje);
    Ok(mensaje)
}

fn dir_auditoria(app: &AppHandle) -> Result<std::path::PathBuf, String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}

fn auditar(app: &AppHandle, tipo: &str, sistema: &str) {
    let Ok(dir) = dir_auditoria(app) else {
        return;
    };
    let mensaje = match crate::events::publicar(&dir, tipo, sistema) {
        Ok(texto) => texto,
        Err(error) => error.to_string(),
    };
    let _ = std::fs::write(dir.join("ultimo.txt"), mensaje);
}

fn fallos() -> &'static Mutex<HashMap<String, u32>> {
    static FALLOS: OnceLock<Mutex<HashMap<String, u32>>> = OnceLock::new();
    FALLOS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// 1 s, 2 s, 4 s y después 8 s. RF-16. Argon2id ya es el costo memory-hard.
pub(crate) fn segundos_de_demora(intentos: u32) -> u64 {
    if intentos == 0 {
        return 0;
    }
    1u64 << (intentos - 1).min(3)
}

fn registrar_fallo(ruta: &str) -> u64 {
    let mut mapa = fallos().lock().unwrap_or_else(|e| e.into_inner());
    let n = mapa.entry(ruta.to_string()).or_insert(0);
    *n = n.saturating_add(1);
    segundos_de_demora(*n)
}

fn limpiar_fallos(ruta: &str) {
    let mut mapa = fallos().lock().unwrap_or_else(|e| e.into_inner());
    mapa.remove(ruta);
}

fn bloqueo(estado: &EstadoBoveda) -> std::sync::MutexGuard<'_, Option<Boveda>> {
    estado.0.lock().unwrap_or_else(|e| e.into_inner())
}

fn con_boveda<T>(
    estado: &EstadoBoveda,
    accion: impl FnOnce(&Boveda) -> Result<T, ErrorBoveda>,
) -> Result<T, String> {
    let guard = bloqueo(estado);
    let boveda = guard.as_ref().ok_or("No hay una bóveda abierta.")?;
    accion(boveda).map_err(|e| e.to_string())
}

fn con_boveda_mut<T>(
    estado: &EstadoBoveda,
    accion: impl FnOnce(&mut Boveda) -> Result<T, ErrorBoveda>,
) -> Result<T, String> {
    let mut guard = bloqueo(estado);
    let boveda = guard.as_mut().ok_or("No hay una bóveda abierta.")?;
    accion(boveda).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::segundos_de_demora;

    #[test]
    fn la_demora_crece_y_se_frena_en_ocho_segundos() {
        assert_eq!(segundos_de_demora(0), 0);
        assert_eq!(segundos_de_demora(1), 1);
        assert_eq!(segundos_de_demora(2), 2);
        assert_eq!(segundos_de_demora(3), 4);
        assert_eq!(segundos_de_demora(4), 8);
        assert_eq!(segundos_de_demora(9), 8);
    }
}

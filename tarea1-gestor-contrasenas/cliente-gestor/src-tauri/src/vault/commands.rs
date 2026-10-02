//! Comandos que llama la ventana. La clave de la bóveda vive solo en este proceso.

use std::path::Path;
use std::sync::Mutex;

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
    Ok(dir.join("boveda.sqlite").to_string_lossy().into_owned())
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
pub fn abrir_boveda(
    app: AppHandle,
    estado: State<'_, EstadoBoveda>,
    ruta: String,
    maestra: String,
) -> Result<(), String> {
    match Boveda::abrir(Path::new(&ruta), &maestra) {
        Ok(boveda) => {
            let mut guard = bloqueo(&estado);
            *guard = Some(boveda);
            Ok(())
        }
        Err(ErrorBoveda::MaestraIncorrecta) => {
            auditar(&app, "intento_fallido_maestra", "boveda");
            Err(ErrorBoveda::MaestraIncorrecta.to_string())
        }
        Err(error) => Err(error.to_string()),
    }
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

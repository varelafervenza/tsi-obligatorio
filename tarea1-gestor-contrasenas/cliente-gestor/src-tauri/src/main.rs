// Punto de entrada de la app de escritorio. No debe contener lógica de negocio:
// solo registra los módulos y expone los comandos Tauri que llama el frontend.

mod auth;
mod crypto;
mod events;
mod generator;
mod vault;

use std::time::Duration;

use tauri::Manager;

/// Cuánto se demora como máximo el cierre de la app reintentando enviar la cola de eventos.
/// Si no alcanza, lo que quede sigue en la cola para el próximo reintento.
const PRESUPUESTO_CIERRE: Duration = Duration::from_secs(2);

fn main() {
    tauri::Builder::default()
        .manage(vault::EstadoBoveda::nuevo())
        .setup(|app| {
            // Al abrir: intenta vaciar la cola en segundo plano, sin demorar el arranque.
            if let Ok(dir) = app.path().app_data_dir() {
                std::thread::spawn(move || {
                    let _ = events::reintentar_cola(&dir);
                });
            }
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                let Ok(dir) = window.path().app_data_dir() else {
                    return;
                };
                if events::eventos_pendientes(&dir) == 0 {
                    return;
                }
                // Hay eventos sin enviar: se frena el cierre un instante para intentar
                // vaciarlos, con un tope de tiempo para no colgar la app si no hay conexión.
                api.prevent_close();
                let ventana = window.clone();
                std::thread::spawn(move || {
                    let _ = events::reintentar_cola_con_presupuesto(&dir, PRESUPUESTO_CIERRE);
                    let _ = ventana.destroy();
                });
            }
        })
        .invoke_handler(tauri::generate_handler![
            vault::commands::ruta_por_defecto,
            vault::commands::crear_boveda,
            vault::commands::abrir_boveda,
            vault::commands::cerrar_boveda,
            vault::commands::estado_boveda,
            vault::commands::listar_credenciales,
            vault::commands::alta_credencial,
            vault::commands::obtener_credencial,
            vault::commands::modificar_credencial,
            vault::commands::borrar_credencial,
            vault::commands::generar_contrasena,
            vault::commands::guardar_politica,
            vault::commands::obtener_politica,
            vault::commands::cambiar_maestra,
            vault::commands::estado_auditoria,
            vault::commands::guardar_auditoria,
            vault::commands::reintentar_eventos,
            vault::commands::marcar_favorito,
            vault::commands::exportar_boveda,
            vault::commands::importar_boveda,
            vault::commands::enrolar_totp,
            vault::commands::confirmar_totp,
        ])
        .run(tauri::generate_context!())
        .expect("error al iniciar la aplicacion");
}

// Punto de entrada de la app de escritorio. No debe contener lógica de negocio:
// solo registra los módulos y expone los comandos Tauri que llama el frontend.

mod auth;
mod crypto;
mod events;
mod generator;
mod vault;

fn main() {
    tauri::Builder::default()
        .manage(vault::EstadoBoveda::nuevo())
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
            vault::commands::marcar_favorito,
            vault::commands::exportar_boveda,
            vault::commands::importar_boveda,
        ])
        .run(tauri::generate_context!())
        .expect("error al iniciar la aplicacion");
}

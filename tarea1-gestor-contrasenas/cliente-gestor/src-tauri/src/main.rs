// Punto de entrada de la app de escritorio. No debe contener lógica de negocio:
// solo registra los módulos y expone los comandos Tauri que llama el frontend.

mod crypto;
mod vault;
mod auth;
mod events;
mod generator;

fn main() {
    tauri::Builder::default()
        // TODO: registrar comandos (vault::abrir_boveda, generator::generar, events::emitir_evento, etc.)
        .run(tauri::generate_context!())
        .expect("error al iniciar la aplicacion");
}

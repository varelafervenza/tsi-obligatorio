//! Windows Hello como segundo factor local para abrir la bóveda (RF-02).
//! Es una verificación de presencia/biometría del usuario en este equipo, no una
//! credencial FIDO: no agrega fuerza criptográfica a la clave de la bóveda.

#[cfg(windows)]
mod imp {
    use windows::core::{factory, HSTRING};
    use windows::Security::Credentials::UI::{
        IUserConsentVerifierInterop, UserConsentVerificationResult, UserConsentVerifier,
        UserConsentVerifierAvailability,
    };
    use windows::Win32::Foundation::HWND;

    pub fn disponible() -> Result<bool, String> {
        let estado = UserConsentVerifier::CheckAvailabilityAsync()
            .and_then(|op| op.get())
            .map_err(|e| e.to_string())?;
        Ok(estado == UserConsentVerifierAvailability::Available)
    }

    pub fn verificar(hwnd: isize, mensaje: &str) -> Result<bool, String> {
        let interop = factory::<UserConsentVerifier, IUserConsentVerifierInterop>()
            .map_err(|e| e.to_string())?;
        let operacion = unsafe {
            interop.RequestVerificationForWindowAsync(HWND(hwnd as *mut _), &HSTRING::from(mensaje))
        }
        .map_err(|e| e.to_string())?;
        let resultado = operacion.get().map_err(|e| e.to_string())?;
        Ok(resultado == UserConsentVerificationResult::Verified)
    }
}

#[cfg(not(windows))]
mod imp {
    pub fn disponible() -> Result<bool, String> {
        Ok(false)
    }

    pub fn verificar(_hwnd: isize, _mensaje: &str) -> Result<bool, String> {
        Err("Windows Hello solo está disponible en Windows".into())
    }
}

#[tauri::command]
pub fn windows_hello_disponible() -> Result<bool, String> {
    imp::disponible()
}

#[tauri::command]
pub fn verificar_windows_hello(window: tauri::WebviewWindow) -> Result<bool, String> {
    let hwnd = window.hwnd().map_err(|e| e.to_string())?;
    imp::verificar(hwnd.0 as isize, "Confirme su identidad para abrir la bóveda")
}

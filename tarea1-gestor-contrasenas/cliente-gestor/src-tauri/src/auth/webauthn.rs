//! Windows Hello como segundo factor local para abrir la bóveda (RF-02).
//! Es una verificación de presencia/biometría del usuario en este equipo, no una
//! credencial FIDO: no agrega fuerza criptográfica a la clave de la bóveda.

#[cfg(windows)]
mod imp {
    use windows::core::{factory, HSTRING};
    use windows::Security::Credentials::UI::{
        UserConsentVerificationResult, UserConsentVerifier, UserConsentVerifierAvailability,
    };
    use windows::Win32::Foundation::HWND;
    use windows::Win32::System::WinRT::IUserConsentVerifierInterop;
    use windows_future::IAsyncOperation;

    pub fn disponible() -> Result<bool, String> {
        let operacion = UserConsentVerifier::CheckAvailabilityAsync().map_err(|e| e.to_string())?;
        let estado = operacion.GetResults().map_err(|e| e.to_string())?;
        Ok(estado == UserConsentVerifierAvailability::Available)
    }

    pub fn verificar(hwnd: isize, mensaje: &str) -> Result<bool, String> {
        let interop = factory::<UserConsentVerifier, IUserConsentVerifierInterop>()
            .map_err(|e| e.to_string())?;
        let operacion: IAsyncOperation<UserConsentVerificationResult> = unsafe {
            interop.RequestVerificationForWindowAsync(HWND(hwnd as *mut _), &HSTRING::from(mensaje))
        }
        .map_err(|e| e.to_string())?;
        let resultado = operacion.GetResults().map_err(|e| e.to_string())?;
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

pub fn disponible() -> Result<bool, String> {
    imp::disponible()
}

pub fn verificar(hwnd: isize) -> Result<bool, String> {
    imp::verificar(hwnd, "Confirme su identidad para abrir la bóveda")
}

#[tauri::command]
pub fn windows_hello_disponible() -> Result<bool, String> {
    disponible()
}

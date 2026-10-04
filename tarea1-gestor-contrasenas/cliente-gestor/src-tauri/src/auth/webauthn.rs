//! Windows Hello como segundo factor local para abrir la bóveda (RF-02).
//! Confirma la presencia del usuario en este equipo. No agrega fuerza criptográfica a la clave
//! de la bóveda, que sigue derivada solo de la maestra con Argon2id.
//!
//! Camino principal: clave de Windows Hello (`KeyCredentialManager`), que acepta PIN y huella.
//! Respaldo: confirmación biométrica (`UserConsentVerifier`), solo si la clave no es compatible.

#[cfg(windows)]
mod imp {
    use windows::core::{factory, HSTRING};
    use windows::Security::Credentials::{
        KeyCredentialCreationOption, KeyCredentialManager, KeyCredentialStatus,
        UserConsentVerificationResult, UserConsentVerifier, UserConsentVerifierAvailability,
    };
    use windows::Security::Cryptography::CryptographicBuffer;
    use windows::Win32::Foundation::HWND;
    use windows::Win32::System::WinRT::IUserConsentVerifierInterop;
    use windows_future::IAsyncOperation;

    const CLAVE: &str = "GestorContrasenas-Boveda";
    const MENSAJE: &str = "Confirme su identidad para abrir la bóveda";

    fn texto<E: std::fmt::Display>(error: E) -> String {
        error.to_string()
    }

    fn clave_soportada() -> Result<bool, String> {
        KeyCredentialManager::IsSupportedAsync()
            .map_err(texto)?
            .GetResults()
            .map_err(texto)
    }

    fn motivo_clave(estado: KeyCredentialStatus) -> String {
        if estado == KeyCredentialStatus::UserPrefersPassword {
            "Windows Hello está en modo contraseña: configurá PIN o huella para usarlo.".into()
        } else if estado == KeyCredentialStatus::NotFound {
            "Esta bóveda no tiene clave de Windows Hello. Activá Windows Hello de nuevo.".into()
        } else if estado == KeyCredentialStatus::UserCanceled {
            "Cancelaste la confirmación de Windows Hello.".into()
        } else {
            format!("Windows Hello devolvió un error al usar la clave ({estado:?}).")
        }
    }

    fn registrar_clave() -> Result<(), String> {
        let resultado = KeyCredentialManager::RequestCreateAsync(
            &HSTRING::from(CLAVE),
            KeyCredentialCreationOption::ReplaceExisting,
        )
        .map_err(texto)?
        .GetResults()
        .map_err(texto)?;
        let estado = resultado.Status().map_err(texto)?;
        if estado == KeyCredentialStatus::Success {
            Ok(())
        } else {
            Err(motivo_clave(estado))
        }
    }

    fn verificar_clave() -> Result<bool, String> {
        let apertura = KeyCredentialManager::OpenAsync(&HSTRING::from(CLAVE))
            .map_err(texto)?
            .GetResults()
            .map_err(texto)?;
        let estado = apertura.Status().map_err(texto)?;
        if estado != KeyCredentialStatus::Success {
            return Err(motivo_clave(estado));
        }
        let credencial = apertura.Credential().map_err(texto)?;
        let desafio = CryptographicBuffer::GenerateRandom(32).map_err(texto)?;
        let firma = credencial
            .RequestSignAsync(&desafio)
            .map_err(texto)?
            .GetResults()
            .map_err(texto)?;
        let estado_firma = firma.Status().map_err(texto)?;
        if estado_firma == KeyCredentialStatus::Success {
            Ok(true)
        } else if estado_firma == KeyCredentialStatus::UserCanceled {
            Ok(false)
        } else {
            Err(motivo_clave(estado_firma))
        }
    }

    fn motivo_disponibilidad(estado: UserConsentVerifierAvailability) -> String {
        if estado == UserConsentVerifierAvailability::NotConfiguredForUser {
            "No hay PIN ni huella configurados para tu usuario de Windows.".into()
        } else if estado == UserConsentVerifierAvailability::DeviceNotPresent {
            "Este equipo no tiene sensor de huella ni de rostro, y la clave de Windows Hello no es compatible.".into()
        } else if estado == UserConsentVerifierAvailability::DisabledByPolicy {
            "Una política de la organización deshabilita Windows Hello en este equipo.".into()
        } else if estado == UserConsentVerifierAvailability::DeviceBusy {
            "El dispositivo de Windows Hello está ocupado. Probá de nuevo en unos segundos.".into()
        } else {
            format!("Windows Hello no está disponible ({estado:?}).")
        }
    }

    fn consentimiento_disponible() -> Result<(), String> {
        let estado = UserConsentVerifier::CheckAvailabilityAsync()
            .map_err(texto)?
            .GetResults()
            .map_err(texto)?;
        if estado == UserConsentVerifierAvailability::Available {
            Ok(())
        } else {
            Err(motivo_disponibilidad(estado))
        }
    }

    fn motivo_verificacion(estado: UserConsentVerificationResult) -> String {
        if estado == UserConsentVerificationResult::DeviceNotPresent {
            "No hay sensor de huella ni de rostro para confirmar la identidad.".into()
        } else if estado == UserConsentVerificationResult::NotConfiguredForUser {
            "No hay PIN ni huella configurados para tu usuario de Windows.".into()
        } else if estado == UserConsentVerificationResult::DisabledByPolicy {
            "Una política de la organización deshabilita Windows Hello.".into()
        } else if estado == UserConsentVerificationResult::DeviceBusy {
            "El dispositivo de Windows Hello está ocupado. Probá de nuevo.".into()
        } else if estado == UserConsentVerificationResult::RetriesExhausted {
            "Se agotaron los intentos de Windows Hello. Esperá un momento y reintentá.".into()
        } else {
            format!("Windows Hello no confirmó la identidad ({estado:?}).")
        }
    }

    fn verificar_consentimiento(hwnd: isize) -> Result<bool, String> {
        let interop = factory::<UserConsentVerifier, IUserConsentVerifierInterop>().map_err(texto)?;
        let operacion: IAsyncOperation<UserConsentVerificationResult> = unsafe {
            interop.RequestVerificationForWindowAsync(HWND(hwnd as *mut _), &HSTRING::from(MENSAJE))
        }
        .map_err(texto)?;
        let resultado = operacion.GetResults().map_err(texto)?;
        if resultado == UserConsentVerificationResult::Verified {
            Ok(true)
        } else if resultado == UserConsentVerificationResult::Canceled {
            Ok(false)
        } else {
            Err(motivo_verificacion(resultado))
        }
    }

    pub fn disponible() -> Result<(), String> {
        if clave_soportada()? {
            return Ok(());
        }
        consentimiento_disponible()
    }

    pub fn registrar() -> Result<(), String> {
        if clave_soportada()? {
            registrar_clave()
        } else {
            consentimiento_disponible()
        }
    }

    pub fn verificar(hwnd: isize) -> Result<bool, String> {
        if clave_soportada()? {
            return verificar_clave();
        }
        verificar_consentimiento(hwnd)
    }
}

#[cfg(not(windows))]
mod imp {
    const SOLO_WINDOWS: &str = "Windows Hello solo está disponible en Windows.";

    pub fn disponible() -> Result<(), String> {
        Err(SOLO_WINDOWS.into())
    }

    pub fn registrar() -> Result<(), String> {
        Err(SOLO_WINDOWS.into())
    }

    pub fn verificar(_hwnd: isize) -> Result<bool, String> {
        Err(SOLO_WINDOWS.into())
    }
}

pub use imp::{disponible, registrar, verificar};

#[tauri::command]
pub fn windows_hello_disponible() -> Result<(), String> {
    disponible()
}

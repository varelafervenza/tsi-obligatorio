# Gestión de Identidades y Accesos (IAM) — Gestor de Contraseñas con Control Centralizado

> Basado en `plantilla/isaca/09-gestion-accesos.md`. Función **Proteger** del MCU 5.0. Este
> documento define **cómo va a funcionar** el acceso (diseño); las capturas de enroll y de
> configuración del hash son **evidencia** que se agrega recién cuando el módulo de auth deje de
> ser un TODO (`cliente-gestor/src-tauri/src/auth/`, `control-central/app/core/mfa.py`) — ver el
> callout al final de la sección 1 con el detalle de qué capturar y cuándo.

---

## Encabezado de mapeo normativo

| Marco | Ítem | Detalle / aporte |
|---|---|---|
| **MCU 5.0 (función)** | **Proteger** | PR-01, PR-02, PR-06, PR-07 (identidad, credenciales, autenticación). |
| **COBIT 2019** | DSS05, APO13 | Gestión de accesos y de usuarios. |
| **ISO/IEC 27001:2022** | A.5.15-A.5.19, A.8.2-A.8.9 | Controles de identidad y acceso. |
| **BCU — GSI** | 2FA obligatorio | Para operaciones de alto valor (ver política de factores, sección 1). |
| **URCDP — Ley 18.331** | Art. 9 y 12 | Seguridad de los datos personales tratados en A02, A05, A07 (`02-Registro-Activos.md`). |

---

## Control del documento

| Campo | Valor |
|---|---|
| Código | SI-IAM-09 |
| Versión | 0.1 (borrador — diseño; sin evidencia real todavía) |
| Responsable | Blue Team (Horacio Duarte, Pablo Morales, Andrés Varela) |
| Fecha | 22/09/2026 |

## 1. Modelo de autenticación

| Factor | Mecanismo en esta solución | Dónde vive |
|---|---|---|
| Algo que sé (S) | Contraseña maestra → Argon2id (KDF) | `cliente-gestor/src-tauri/src/crypto/kdf.rs` |
| Algo que tengo (T) | TOTP (RFC 6238) o llave WebAuthn/FIDO2 | `cliente-gestor/src-tauri/src/auth/totp.rs`, `webauthn.rs` |
| Algo que soy (B) | Windows Hello (biometría/PIN), como authenticator de plataforma WebAuthn | Mismo módulo `auth/webauthn.rs` |

### Política de factores según operación

| Operación | Dónde ocurre | Factor mínimo | Mecanismo |
|---|---|---|---|
| Abrir la bóveda | `cliente-gestor` | 1 factor obligatorio + 2do opcional | Contraseña maestra (+ TOTP/WebAuthn si el usuario lo habilitó) |
| Modificar / agregar / borrar credencial | `cliente-gestor` | 1 factor (bóveda ya abierta) | Ninguno adicional; el evento firmado ya deja trazabilidad (RF-08) |
| **Cambiar la contraseña maestra** | `cliente-gestor` | 2 factores | Maestra actual + MFA local, por ser evento crítico (RF-07) |
| Acceso al panel/dashboard de `control-central` | `control-central` | 2 factores | Password del panel (Argon2id/bcrypt) + TOTP/WebAuthn |
| Acciones administrativas (cambiar rol, elegir algoritmo de hash) | `control-central` | 2 factores + rol admin | RBAC (`app/api/users.py`) + MFA |

> **Nota de diseño**: preferimos WebAuthn/Windows Hello por sobre TOTP cuando el hardware lo
> permite, porque TOTP es phishable (RT-06 del Red Team apunta justo a esto — ver `R06` en
> `03-Analisis-Riesgos.md`) y WebAuthn no.

## 2. Gestión de identidades (provisión / desprovisión)

En `cliente-gestor` no hay "usuarios" en el sentido tradicional: cada bóveda pertenece a una
sola persona y su "alta" es crear la bóveda por primera vez. La gestión de identidades formal
aplica a los **usuarios del panel de `control-central`**:

| Proceso | Procedimiento | Vencimiento |
|---|---|---|
| Alta de usuario del panel | Se crea vía `POST /api/users` (RBAC, rol inicial mínimo) | Al ingreso al equipo/rol |
| Modificación de roles | Requiere aprobación del RSI | Bajo pedido |
| Baja de usuario del panel | Deshabilitar la cuenta (no borrar, por trazabilidad de auditoría) | ≤ 24 h desde la baja efectiva |
| Revisión de accesos | Revisar roles activos contra la matriz RACI (`03-matriz-raci-mcu5.xlsx`) | Antes de cada hito (H2, H3, H4) |

## 3. Registro de accesos (actas)

| Usuario | Rol | Recursos accesibles | Fecha alta | Fecha baja | Revisión |
|---|---|---|---|---|---|
| _(pendiente: recién tiene sentido llenarlo cuando `control-central` esté desplegado con usuarios reales del panel — no antes de H2, 29/09)_ | | | | | |

## 4. Gestión de secretos y credenciales

- Las contraseñas de los **usuarios del panel de `control-central`** se hashean con
  **Argon2id o bcrypt**, elegible desde `app/core/config.py` (`HASH_ALGORITHM`). Argon2id es la
  opción por defecto (memory-hard, recomendado por OWASP).
- Las **contraseñas maestras** de cada bóveda nunca llegan a `control-central`: se derivan y
  verifican íntegramente en `cliente-gestor` (zero-knowledge, ver `01-Politica-Seguridad.md`).
- Las **claves privadas de firma de eventos** (JWS, una por agente) se guardan en el keystore
  local del cliente, nunca en el repositorio ni en `control-central`.
- Prohibido: compartir cuentas del panel, guardar contraseñas maestras en texto plano en
  cualquier lugar (notas, chats, tickets), reutilizar la maestra en otro servicio.

## 5. Política de contraseñas (por sistema)

Esto es exactamente RF-04/RF-05: el usuario define, **por sistema**, la plantilla de contraseña
que va a generar y validar el módulo `generator` de `cliente-gestor`.

| Sistema/tipo | Regla de generación / regex | Longitud | Complejidad | Ciclo de cambio |
|---|---|---|---|---|
| Ejemplo — sistema genérico | `^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[^\w]).+$` (definible por el usuario) | 16+ | Mayúscula+minúscula+número+símbolo | 90 días (aviso, RF-17) |
| Ejemplo — passphrase (XKCD 936) | 4-6 palabras de una wordlist | Equivalente a 16+ | N/A (entropía por longitud) | A criterio del usuario |
| Contraseña maestra | Definida por el usuario, sin regex forzada por el sistema (es su elección) | Recomendado 16+ | Medidor de fortaleza en la UI | Solo ante sospecha de compromiso (evento crítico si cambia) |

## 6. Registro de autenticaciones (audit)

- Intentos de apertura de bóveda (éxito/fallo) quedan localmente en `cliente-gestor`; **solo se
  reenvía a `control-central`/Wazuh cuando se supera el umbral de fuerza bruta** (RF-16), no cada
  intento — para no filtrar información de timing innecesaria.
- Alertas Wazuh: N intentos fallidos consecutivos de maestra en ventana T (regla custom, ver
  `07-Monitoreo-Logs-SIEM.md`, pendiente).
- Enroll/recovery de TOTP o WebAuthn en `cliente-gestor` genera un evento firmado (mismo
  mecanismo que alta/mod/borrado), porque cambiar el segundo factor es tan sensible como cambiar
  la maestra.
- Logins al panel de `control-central` (éxito/fallo) sí quedan siempre registrados ahí, por ser
  un sistema multiusuario.

---

## Evidencia pendiente (qué capturar y cuándo)

Esta es la lista concreta de screenshots/capturas que hay que sacar **una vez que el código deje
de ser un esqueleto**, para completar tanto este documento como el Excel
`01-controles-mcu5-perfil-avanzado.xlsx`:

| Evidencia a capturar | Cuándo se puede tomar | Dónde guardarla |
|---|---|---|
| Enroll de TOTP (QR + primer código validado) | Cuando `auth/totp.rs` esté implementado (meta: H3, 02/10) | `docs/evidencias/09-enroll-totp.png` |
| Registro de WebAuthn/Windows Hello | Cuando `auth/webauthn.rs` esté implementado (meta: H3) | `docs/evidencias/09-enroll-webauthn.png` |
| Config del algoritmo de hash en `control-central` | Cuando `control-central` esté desplegado con su `.env` real (meta: H2, 29/09) | `docs/evidencias/09-config-hash.png` |
| Intento de apertura de bóveda con maestra incorrecta (fricción/delay) | Cuando `crypto/kdf.rs` + delay adaptativo estén implementados | `docs/evidencias/09-delay-fuerza-bruta.png` |
| Login al panel con 2FA | Cuando `control-central` tenga MFA operativo | `docs/evidencias/09-login-panel-2fa.png` |

> No tiene sentido intentar generar esta evidencia antes de esas fechas: sería una captura de
> algo que no funciona de verdad, y en la auditoría del 14/10 piden poder **demostrarlo en vivo**
> (sección 6.6 de `LETRA.md`) — una captura sin la funcionalidad real detrás es peor que no
> tenerla, porque es un control "declarado pero no demostrable" (hallazgo de auditoría).

---

## Guía de llenado (Blue Team)

| Fase | Cómo completar | Con qué evidencia |
|---|---|---|
| Diseño (actual) | Definir el modelo de factores y política de contraseñas | Este documento |
| Implementación (H2-H3) | Implementar TOTP, WebAuthn, Windows Hello y el hash configurable | Capturas de la tabla de arriba |
| Demo (H4/H5) | Probar en vivo las políticas de factor por operación | Video/capturas en `docs/evidencias/` |
| Red Team | Va a atacar la autenticación (phishing TOTP, bypass) — ver R06 en `03-Analisis-Riesgos.md` | Informe Red Team |

## Check de aceptación

- [x] MFA obligatorio definido en al menos 2 operaciones críticas (cambio de maestra, panel).
- [ ] TOTP operativo con RFC 6238 (pendiente de implementación).
- [ ] Argon2id/bcrypt configurado y evidenciado en `control-central` (pendiente de despliegue).
- [ ] Revisión de accesos con acta (pendiente: sección 3 vacía hasta tener usuarios reales del panel).
- [ ] Registro de autenticaciones llegando al SIEM (depende de `07-Monitoreo-Logs-SIEM.md`, aún pendiente).

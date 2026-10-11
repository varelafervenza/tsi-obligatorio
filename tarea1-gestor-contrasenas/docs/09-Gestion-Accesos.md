# Gestión de Identidades y Accesos (IAM) — Gestor de Contraseñas con Control Centralizado

> Basado en `plantilla/isaca/09-gestion-accesos.md`. Función **Proteger** del MCU 5.0. Describe
> el acceso **implementado**: TOTP en la bóveda y en el panel, RBAC y hash elegible. Hello y
> WebAuthn quedan fuera de la entrega (sección «Limitación de alcance»).

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
| Versión | 0.2 (TOTP y RBAC implementados; Hello fuera de alcance) |
| Responsable | Blue Team (Horacio Duarte, Pablo Morales, Andrés Varela) |
| Fecha | 10/10/2026 |

## 1. Modelo de autenticación

| Factor | Mecanismo en esta solución | Dónde vive |
|---|---|---|
| Algo que sé (S) | Contraseña maestra → Argon2id (KDF) | `cliente-gestor/src-tauri/src/crypto/kdf.rs` |
| Algo que tengo (T) | TOTP (RFC 6238) | `cliente-gestor/src-tauri/src/auth/totp.rs` y TOTP del panel |
| Algo que soy (B) | Windows Hello / WebAuthn | **Fuera de la entrega** (ver limitación más abajo) |

### Política de factores según operación

| Operación | Dónde ocurre | Factor mínimo | Mecanismo |
|---|---|---|---|
| Abrir la bóveda | `cliente-gestor` | 1 factor obligatorio + 2do opcional | Contraseña maestra (+ TOTP si el usuario lo habilitó) |
| Modificar / agregar / borrar credencial | `cliente-gestor` | 1 factor (bóveda ya abierta) | Ninguno adicional; el evento firmado ya deja trazabilidad (RF-08) |
| **Cambiar la contraseña maestra** | `cliente-gestor` | 2 factores | Maestra actual + TOTP si está enrolado (RF-07) |
| Acceso al panel/dashboard de `control-central` | `control-central` | 2 factores | Password del panel (Argon2id/bcrypt) + TOTP |
| Acciones administrativas (cambiar rol, elegir algoritmo de hash) | `control-central` | 2 factores + rol admin | RBAC (`app/api/users.py`) + MFA |

> **Nota de diseño**: preferimos WebAuthn/Windows Hello por sobre TOTP cuando el hardware lo
> permite, porque TOTP es phishable (RT-06 del Red Team apunta justo a esto — ver `R06` en
> `03-Analisis-Riesgos.md`) y WebAuthn no.

### Limitación de alcance: Windows Hello y WebAuthn

**Decisión:** acordada con el docente, Windows Hello (y WebAuthn/U2F) queda **fuera de la
entrega** y se documenta como limitación. El segundo factor implementado es TOTP, junto con la
maestra.

**Razones:**
- Smart App Control de Windows 11 bloquea los ejecutables sin firma digital. Firmar el
  instalador requiere un certificado de pago, que el equipo decidió no adquirir, y el equipo
  no desactiva esa protección.
- Las APIs de Windows Hello (`KeyCredentialManager` y `UserConsentVerifier`) dependen del
  hardware y de la configuración del equipo. En el equipo de desarrollo, con PIN configurado
  pero sin sensor de huella o rostro, la verificación no pudo validarse de forma confiable, y
  el mensaje de error no alcanzó para diagnosticar la causa.
- WebAuthn en el navegador exige HTTPS con un dominio fijo (RP ID), que no existe en el
  laboratorio (ver la limitación de TLS en `00-arquitectura-c4.md`).

**Qué queda en el código:** el código de Windows Hello se eliminó de la versión entregada. El
segundo factor de la bóveda es sólo TOTP, y no se presenta Hello como control demostrable.

**Riesgo residual:** el segundo factor depende de TOTP, que es susceptible a phishing
(RT-06). Se acepta con la justificación anterior y se refleja en `03-Analisis-Riesgos.md`.

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
| rsi@correo.local | admin | Panel `/api/users`, incidentes, KPIs | 06/10/2026 | — | Prueba B-14 de `guia-de-pruebas.md` |

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

## Evidencia (qué mostrar en la auditoría)

| Evidencia | Dónde está | Estado |
|---|---|---|
| TOTP de la bóveda (QR y confirmación) | App + prueba C-09 de `guia-de-pruebas.md` | Implementado |
| Delay al fallar la maestra | `vault/commands.rs`; prueba en la guía | Implementado |
| TOTP del panel | `POST /api/users/.../totp`; prueba B-14 | Implementado |
| Hash Argon2id/bcrypt | `app/core/config.py`, alta de usuario | Implementado |
| Windows Hello / WebAuthn | Limitación en este documento | Fuera de alcance |

---

## Guía de llenado (Blue Team)

| Fase | Cómo completar | Con qué evidencia |
|---|---|---|
| Diseño | Modelo de factores y política de contraseñas | Este documento |
| Implementación | TOTP y hash configurable (hecho). Hello/WebAuthn fuera de alcance | Guía C-09 y B-14 |
| Demo (14/10) | Probar en vivo maestra + TOTP y el panel | App y `http://localhost:8001/docs` |
| Red Team | Va a atacar la autenticación (phishing TOTP, bypass) — ver R06 en `03-Analisis-Riesgos.md` | Informe Red Team |

## Check de aceptación

- [x] MFA obligatorio definido en al menos 2 operaciones críticas (cambio de maestra, panel).
- [x] TOTP operativo con RFC 6238 (bóveda y panel).
- [x] Argon2id/bcrypt elegible en `control-central`.
- [x] Usuario de prueba `rsi@correo.local` en la sección 3.
- [ ] Revisión periódica de accesos con acta (no hay calendario formal).

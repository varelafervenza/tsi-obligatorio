# Plantilla ISACA/COBIT 2019 — Gestión de Identidades y Accesos (IAM / TOTP / SSO)

---

## Encabezado de mapeo normativo

| Marco | Ítem | Detalle / aporte |
|---|---|---|
| **MCU 5.0 (función)** | **Proteger** | Protección del acceso y de las identidades. |
| **MCU 5.0 (categorías)** | PR-01, PR-02, PR-06, PR-07 | Identidad, credenciales, autenticación/… de acceso. |
| **COBIT 2019** | DSS05, APO13 | Gestión de accesos y de usuarios. |
| **ISO/IEC 27001:2022** | A.5.15 Accesos, A.5.16, A.5.17, A.5.18, A.8.2 a A.8.5, A.8.6 a A.8.9 | Controles de identidad y acceso (privilegios, revisión). |
| **ISO/IEC 27002** | 5.15–5.19, 8.1–8.9 | Guía de implementación de accesos. |
| **BCU — Guía de Seguridad de la Información** | Requisito de autenticación y accesos | **Autenticación de dos factores obligatoria** para transferencias/operaciones; control de accesos según jerarquía. |
| **URCDP — Ley 18.331** | Art. 9 y 12 | Seguridad de los datos personales. |

> Requisitos del usuario de la letra: la solución debe poder configurarse con **TOTP**, **Windows Hello**, **WebAuthn**, **u2f/argon2** y elegir algoritmos.

---

## Control del documento

| Campo | Valor |
|---|---|
| Código | SI-IAM-09 |
| Versión | 1.0 |
| Responsable | `[Nombre]` |
| Fecha | `[DD/MM/AAAA]` |

---

## 1. Modelo de autenticación

| Factor | Mecanismo | Solicitud / intervalo |
|---|---|---|
| Algo que sé (S) | Password + hash (Argon2id / bcrypt) | al inicio de sesión |
| Algo que tengo (T) | TOTP (RFC 6238) / llave U2F/WebAuthn | al inicio de sesión / alta sensibilidad |
| Algo que soy (B) | Windows Hello (biometría/PIN) | cuando el usuario lo habilita |

### Política de factores según operación

| Operación | Factor mínimo | Mecanismo |
|---|---|---|
| Ver contraseñas en el gestor | 2 factores | Password + TOTP |
| Modificar / agregar / borrar contraseña | 2 factores | Password + TOTP |
| Cambiar password maestra | 2 factores y reconfirmación | Password + TOTP / Hello |
| Acceso al dashboard de control | 2 factores | Password + TOTP |
| Acceso administrativo | 2 factores + MFA administrativo | + U2F recomendado |

## 2. Gestión de identidades (provisión / desprovisión)

| Proceso | Procedimiento | Vencimiento |
|---|---|---|
| Alta de usuario | `[creación en Keycloak/Gestor]` | al ingreso |
| Modificación de roles | `[revisión gerencial]` | bajo pedido |
| Baja de usuario | `[deshabilitar al día siguiente]` | ≤ 24 h |
| Revisión de accesos | `[auditoría semestral]` | 2 veces/año |

## 3. Registro de accesos (actas)

| Usuario | Rol | Recursos accesibles | Fecha alta | Fecha baja | Revisión |
|---|---|---|---|---|---|
| | | | | | |

## 4. Gestión de secretos y credenciales

- Las credenciales de la solución se almacenan protegidas con `[Argon2id / bcrypt]`.
- Llaves maestras: fragmentadas/rotadas cada `[n]` días, guardadas fuera del gestor.
- Prohibido: compartir cuentas, usar credenciales en texto plano, reutilizar password maestra en otros servicios.

## 5. Política de contraseñas (por sistema)

| Sistema/tipo | Regla de generación / regex | Longitud | Complejidad | Ciclo de cambio |
|---|---|---|---|---|
| `[Servidor]` | `[expresión regular]` | `[16+]` | `[may/mín/núm/símbol]` | `[90 d]` |
| `[App]` | `[regex]` | `[20+]` | `[alta]` | `[180 d]` |
| Password maestra | `[regex]` | `[mín 16]` | `[diseñada por el usuario]` | `[180 d / solo cambio]` |

## 6. Registro de autenticaciones (audit)

- Logs de autenticación exitosas y fallidas → SIEM.
- Alertas de `[n]` intentos fallidos consecutivos.
- Registro de cambios (TOTP enroll/recover).

---

## Guía de llenado (Blue Team)

| Fase | Cómo completar | Con qué evidencia |
|---|---|---|
| Blue Team - Implementación | Implementar TOTP, WebAuthn, Windows Hello (según corresponda) | Captura de enroll + login |
| Blue Team - Demo | Probar políticas de factor por operación | Video / capturas |
| Red Team | Atacará la autenticación (phishing, token, bypass) | Informe Red Team |
| Cierre | Corregir hallazgos de acceso | Plan de mejora |

## Check de aceptación

- [ ] MFA obligatorio en al menos 2 operaciones críticas.
- [ ] TOTP operativo con RFC 6238.
- [ ] Argon2/bcrypt usado para hashing de credenciales.
- [ ] Revisión de accesos con acta.
- [ ] Registro de autenticaciones en el SIEM.
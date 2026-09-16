# Modelo de arquitectura C4 — Gestor de Contraseñas con Control Centralizado

> Basado en `plantilla/plantilla-arquitectura-C4.md`. Borrador para hito **H1 (21/09/2026)**.
> Para T1 corresponde: Niveles 1 y 2 completos + Nivel 3 para autenticación/cifrado (según la
> guía "Uso por tarea" de la plantilla).

---

## Control del documento

| Campo | Valor |
|---|---|
| Nombre | Modelo de arquitectura C4 — Gestor de Contraseñas con Control Centralizado |
| Código | ARQ-C4-01 |
| Versión | 0.1 (borrador) |
| Autor/equipo | Blue Team |
| Fecha | 15/09/2026 |
| Aprobación | Pendiente (docente / RSI) |

---

## Nivel 1 — Contexto

El sistema como una sola caja: **"Gestor de Contraseñas con Control Centralizado"**.

| Actor/Sistema externo | Relación con el sistema | Datos intercambiados |
|---|---|---|
| Usuario final | Usa el gestor local para crear/consultar/gestionar credenciales | Ninguno sale de su equipo salvo eventos de auditoría |
| RSI / Auditor (Grafana + correo) | Recibe notificaciones y monitorea el dashboard | Eventos de auditoría (metadata), alertas, KPIs — nunca secretos |
| SIEM (Wazuh) | Recibe eventos normalizados y agentes reportan integridad de archivos | Eventos, logs, resultado de reglas |
| Servidor de correo (Mailu) | Recibe solicitudes de envío desde el control central | Correos de notificación (alta/mod/borrado/cambio de maestra) |
| Red Team (Parte 2, fuera del límite de confianza) | Ataca el sistema una vez congelado | N/A — es un actor adversarial, no un integrador |

**Límite de confianza clave**: todo lo que cruza de `cliente-gestor` hacia el resto del sistema
es metadata firmada (JWS). Ningún actor externo al equipo del usuario puede ver el contenido
de una bóveda (zero-knowledge, sección 1.1 de `LETRA.md`).

## Nivel 2 — Contenedores

| Contenedor | Tecnología | Responsabilidad | Despliegue | Protocolo |
|---|---|---|---|---|
| Cliente de escritorio | Tauri (Rust) + React/TS | Bóveda local, MFA, generador, firma de eventos | Binario nativo en la VM/equipo del usuario | HTTPS saliente (eventos) |
| API de control central | Python / FastAPI | Verificación de eventos, RBAC, dashboard API | Contenedor Docker | REST/HTTPS |
| Base de datos | PostgreSQL 16 | Persistencia de eventos, usuarios, incidentes | Contenedor Docker | SQL/TLS |
| SIEM | Wazuh (manager + indexer + dashboard) | Correlación, reglas de detección, FIM | VM dedicada (stack oficial `wazuh-docker`) | Syslog / API Wazuh |
| Servidor de correo | Mailu | Envío de notificaciones | VM dedicada (stack oficial Mailu) | SMTP/TLS |
| Dashboard de KPIs | Grafana | Visualización de alertas/incidentes/KPIs | Contenedor Docker | HTTP, lee de Postgres/Wazuh |
| Gestión de casos (decisión abierta) | TheHive, o tabla `Incident` interna | Ciclo de vida de incidentes | Contenedor Docker (si se usa TheHive) | REST |

## Nivel 3 — Componentes

### Contenedor: Cliente de escritorio (`cliente-gestor`)

| Componente | Responsabilidad | Depende de |
|---|---|---|
| `crypto` (kdf, cipher) | Argon2id + XChaCha20-Poly1305 | — |
| `vault` (store) | CRUD de credenciales cifradas, historial, políticas por sistema | `crypto` |
| `auth` (totp, webauthn) | MFA local para desbloquear la bóveda | `crypto` (deriva material de sesión) |
| `events` (signer) | Firma JWS y envío/cola de eventos | `vault` (dispara al mutar), clave privada del agente |
| `generator` (regex_policy) | Generación de contraseñas/passphrase cumpliendo regex por sistema | `vault` (lee la política definida) |

### Contenedor: API de control central (`control-central`)

| Componente | Responsabilidad | Depende de |
|---|---|---|
| `api.events` | Recibe y verifica eventos JWS | `core.security`, `models.event` |
| `api.users` | RBAC, MFA de usuarios del panel, elección Argon2id/bcrypt | `core.mfa`, `core.security`, `models.user` |
| `api.dashboard` | KPIs, alertas, incidentes, estado de agentes | `models.event`, `models.incident` |
| `core.security` | Verificación de firma JWS con clave pública por agente; hashing de contraseñas de usuarios del panel | Claves públicas de agentes (`keys/agentes/`) |
| `core.mfa` | TOTP/WebAuthn/Windows Hello para usuarios del control central | `api.users` |
| `siem.wazuh_forwarder` | Normaliza y reenvía eventos a Wazuh | `api.events` |

## Nivel 4 — Código

Solo para los módulos de seguridad, según la guía de la plantilla.

| Componente | Patrones | Archivos/Clases relevantes | Dónde se documenta |
|---|---|---|---|
| `crypto` (cliente) | Strategy (KDF intercambiable Argon2id/bcrypt si se habilita) | `cliente-gestor/src-tauri/src/crypto/kdf.rs`, `cipher.rs` | `docs/09-Gestion-Accesos.md` |
| `events.signer` (cliente) | — | `cliente-gestor/src-tauri/src/events/signer.rs` | `docs/09-Gestion-Accesos.md`, `docs/07-Monitoreo-Logs-SIEM.md` |
| `core.security` (control central) | — | `control-central/app/core/security.py` | `docs/09-Gestion-Accesos.md` |

---

## Guía de auditoría (orden de presentación)

Seguir `plantilla/plantilla-arquitectura-C4.md`: Contexto → Contenedores → Componentes → demo en
vivo → vínculo de cada caja con controles MCU/BCU/ISO.

## Check de aceptación

- [ ] 4 niveles completos (código solo para módulos de seguridad).
- [ ] Tecnologías reales (las efectivamente desplegadas, no solo las planeadas).
- [ ] Relaciones con actores y con SIEM/correo correctas.
- [ ] Acompañado de capturas en `docs/evidencias/`.

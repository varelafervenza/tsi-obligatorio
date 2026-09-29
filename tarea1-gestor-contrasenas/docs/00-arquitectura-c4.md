# Modelo de arquitectura C4 — Gestor de Contraseñas con Control Centralizado

> Basado en `plantilla/plantilla-arquitectura-C4.md`. Borrador para hito **H1 (21/09/2026)**.
> Para T1 corresponde: Niveles 1 y 2 completos + Nivel 3 para autenticación/cifrado (según la
> guía "Uso por tarea" de la plantilla).

---

## Encabezado de mapeo normativo

| Marco | Ítem de referencia |
|---|---|
| **MCU 5.0** | Evidencia para **Identificar** (ID-01), **Gobernar** (GV-01) y **Proteger** (PR-01). |
| **BCU (GSI)** | Inventario y arquitectura de acceso/red. |
| **ISO/IEC 27001:2022** | A.5.9 (Inventario), A.5.29 (continuidad), A.8.16 (monitoreo). |
| **COBIT 2019** | APO03, BAI02, BAI06. |
| **Plantillas asociadas** | `02-registro-activos`, `07-monitoreo-logs`, `06-plan-continuidad`. |

---

## Control del documento

| Campo | Valor |
|---|---|
| Nombre | Modelo de arquitectura C4 — Gestor de Contraseñas con Control Centralizado |
| Código | ARQ-C4-01 |
| Versión | 0.1 (borrador) |
| Autor/equipo | Blue Team (Horacio Duarte, Pablo Morales, Andrés Varela) |
| Fecha | 15/09/2026 |
| Aprobación | Pendiente (docente / RSI) |

---

## Nivel 1 — Contexto

El sistema como una sola caja: **"Gestor de Contraseñas con Control Centralizado"**.

| Actor/Sistema externo | Relación con el sistema | Datos intercambiados |
|---|---|---|
| Usuario final | Usa el gestor local para crear/consultar/gestionar credenciales | Ninguno sale de su equipo salvo eventos de auditoría |
| RSI / Auditor | Consulta el dashboard y recibe notificaciones para supervisar la seguridad y gestionar incidentes | Alertas, incidentes, KPIs y metadatos de auditoría — nunca secretos |
| Dashboard (Grafana) | Visualiza KPIs, alertas, incidentes y funcionamiento para el RSI/auditor | Datos agregados de eventos, alertas, incidentes y agentes |
| SIEM (Wazuh) | Recibe eventos normalizados, recibe reportes de agentes, correlaciona reglas y genera alertas | Eventos, logs, integridad de archivos y resultados de reglas |
| Servidor de correo (Mailu) | Entrega notificaciones enviadas por el control central | Correos de alta, modificación, borrado y cambio de contraseña maestra |
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
| Gestión de incidentes (`Incident` interno) | FastAPI + SQLAlchemy + PostgreSQL | Registra incidentes derivados de alertas Wazuh y gestiona su ciclo de vida | Componente de `control-central` y PostgreSQL | REST/HTTPS |

La gestión de incidentes se implementa inicialmente dentro de `control-central`, sin desplegar
TheHive. Una alerta de Wazuh crea un `Incident` en estado `abierto`; el RSI lo asigna y pasa a
`en análisis`, adjunta evidencias y acciones, y finalmente lo pasa a `resuelto` con responsable
y fecha de cierre. TheHive queda como integración futura, no como alternativa de la arquitectura
actual.

### Flujos de comunicación y protección en tránsito

| Flujo | Protocolo | Protección y condiciones |
|---|---|---|
| Cliente → API de control central | HTTPS | TLS 1.2 o superior, certificado validado por el cliente; el endpoint público termina TLS en un reverse proxy y reenvía a FastAPI por red interna restringida |
| API → PostgreSQL | PostgreSQL sobre TLS cuando atraviesa una red no confiable; en el despliegue local, red interna Docker restringida | Credenciales mediante configuración externa; nunca en el repositorio ni en imágenes |
| API → Mailu | SMTP submission/TLS | Puerto 587, autenticación SMTP y validación del certificado del servidor |
| Agente → Wazuh manager | TLS | Puertos 1514/1515 según el modo de comunicación configurado; certificados y claves gestionados fuera del código |
| Grafana → API/BD | HTTPS o red interna restringida | Acceso mediante RBAC; no se exponen credenciales de PostgreSQL al navegador |

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
| `api.dashboard` | KPIs, alertas, incidentes, estado de agentes y última sincronización | `models.event`, `models.incident` |
| `api.incidents` | Crea un incidente desde una alerta Wazuh, permite asignarlo, adjuntar evidencias y cambiarlo entre `abierto`, `en análisis` y `resuelto` | `models.incident`, `api.dashboard`, PostgreSQL |
| `models.incident` | Persistencia del incidente, alerta de origen, responsable, estado, evidencias, fecha de creación y fecha de resolución | SQLAlchemy, PostgreSQL |
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

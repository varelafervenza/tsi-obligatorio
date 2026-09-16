# Modelo de arquitectura 4+1 — Gestor de Contraseñas con Control Centralizado

> Basado en `plantilla/plantilla-arquitectura-4más1.md`. Borrador para hito **H1 (21/09/2026)**.

---

## Control del documento

| Campo | Valor |
|---|---|
| Nombre | Modelo de arquitectura 4+1 — Gestor de Contraseñas con Control Centralizado |
| Código | ARQ-4+1-01 |
| Versión | 0.1 (borrador) |
| Autor/equipo | Blue Team |
| Fecha | 15/09/2026 |
| Aprobación | Pendiente (docente / RSI) |

---

## 1. Vista Lógica

| Módulo | Responsabilidad | Interfaces que expone/consume | Dependencias |
|---|---|---|---|
| `cliente-gestor` (bóveda) | Almacenar y operar credenciales cifradas localmente: alta/consulta/mod/borrado, generador, políticas por sistema (regex, historial, vencimiento) | Comandos Tauri (IPC interno frontend↔Rust); HTTPS saliente hacia `control-central` (solo eventos) | Ninguna externa para operar (RNF-01) |
| `cliente-gestor` (auth local) | MFA para desbloquear la bóveda: TOTP, WebAuthn/Windows Hello | API WebAuthn de plataforma; librería TOTP | `cliente-gestor` (bóveda) |
| `cliente-gestor` (eventos) | Construir y firmar (JWS) eventos de auditoría; encolar si no hay red | POST `/api/events` hacia `control-central` | `control-central` (API eventos) |
| `control-central` (API eventos) | Verificar firma JWS, persistir evento, reenviar a SIEM, disparar correo | REST (recibe de clientes); interno hacia SIEM y Mail | PostgreSQL, Wazuh, Mailu |
| `control-central` (API usuarios) | Gestión de usuarios/roles del panel, MFA y elección de algoritmo de hash (Argon2id/bcrypt) | REST | PostgreSQL |
| `control-central` (dashboard API) | KPIs, alertas, incidentes, estado de agentes | REST consumido por Grafana | PostgreSQL, Wazuh |
| SIEM (Wazuh) | Correlación de eventos, reglas de detección (fuerza bruta de maestra, borrado masivo, cambio de maestra), FIM | Syslog/API desde `control-central`; agentes en clientes | — |
| Mail (Mailu) | Envío de notificaciones ante alta/mod/borrado/cambio de maestra | SMTP desde `control-central` | — |
| Dashboard (Grafana) | Visualización de KPIs/alertas/incidentes/funcionamiento | Lee de PostgreSQL / Wazuh | `control-central`, Wazuh |
| SOAR/casos (TheHive o tabla `Incident` propia — **decisión abierta**, ver bitácora 15/09) | Registro y ciclo de vida de incidentes | REST / interno en `control-central` | Wazuh (alertas) |

**Patrón**: event-driven entre cliente y control central (el cliente nunca es consultado por el
servidor; solo empuja eventos firmados cuando hay red). Dentro de `control-central`, arquitectura
en capas simple (API → core → modelos → DB), sin microservicios (no se justifica la complejidad
para la escala de RNF-05).

## 2. Vista de Procesos

| Secuencia | Participantes | Flujo (pasos) | Tiempo esperado |
|---|---|---|---|
| Apertura de bóveda | Usuario, `cliente-gestor` (auth, bóveda) | 1. Usuario ingresa contraseña maestra → 2. Argon2id deriva clave → 3. (opcional) MFA local (TOTP/WebAuthn) → 4. Se descifra la bóveda (XChaCha20-Poly1305) → 5. UI muestra credenciales | < 2 s (RNF-02) |
| Alta de credencial → evento → notificación | `cliente-gestor`, `control-central`, SIEM, Mail | 1. Usuario crea credencial → 2. Se cifra y persiste localmente → 3. Se firma evento JWS → 4. POST a `control-central` (si hay red; si no, se encola) → 5. `control-central` verifica firma y persiste → 6. Reenvía a Wazuh → 7. Dispara correo a RSI | Notificación < 1 min desde el evento (a definir como KPI de cobertura) |
| **Cambio de contraseña maestra** (evento crítico) | `cliente-gestor`, `control-central`, SIEM, Mail | 1. Usuario cambia maestra → 2. Se re-deriva y re-cifra toda la bóveda → 3. Evento firmado de severidad alta → 4. `control-central` lo marca crítico → 5. Notificación **inmediata** por correo → 6. Wazuh dispara alerta de alta severidad | Notificación inmediata (RF-07) |
| Detección de fuerza bruta de maestra | `cliente-gestor` (delay adaptativo), Wazuh, Mail | 1. Intentos fallidos repetidos localmente → 2. Delay adaptativo local (RF-16) → 3. Si se supera umbral, evento de intento fallido al control central → 4. Regla Wazuh correlaciona N intentos en ventana T → 5. Alerta + notificación | MTTD como KPI (sección 6.4 LETRA.md) |
| Import/export de bóveda | Usuario, `cliente-gestor` | 1. Exportar bóveda con contraseña de transporte propia (RF-12) → 2. Archivo cifrado portable → 3. Import en otro equipo revalida MFA antes de descifrar | — |

## 3. Vista de Desarrollo

| Capa/Paquete | Tecnología | Responsabilidad | Dependencias |
|---|---|---|---|
| `cliente-gestor/src-tauri` | Rust (Tauri 2) | Lógica de bóveda, cripto, MFA, eventos, generador | `argon2`, `chacha20poly1305`, `rusqlite`, `totp-rs`, `jsonwebtoken` |
| `cliente-gestor/src` | React + TypeScript (Vite) | UI de escritorio | `@tauri-apps/api` |
| `control-central/app` | Python 3.12 + FastAPI | API REST, verificación de eventos, RBAC, dashboard | `sqlalchemy`, `pydantic`, `python-jose`, `passlib` |
| `infra/` | Docker Compose | Orquestación de control-central, Postgres, Grafana, (Wazuh/Mailu vía stacks oficiales) | Docker Engine |
| Repositorio | Git, un solo repo con ambos módulos + `infra/` + `docs/` | Trazabilidad para auditoría y para el Red Team (tag `v1.0`) | — |

## 4. Vista Física / Despliegue

| Nodo | Rol | Componentes que ejecuta | IP/VLAN | Recursos | Puertos |
|---|---|---|---|---|---|
| VM-Cliente | Estación del usuario final | `cliente-gestor` (binario Tauri) + agente Wazuh | `[a definir en el lab]` | 4 GB RAM (RNF-02) | Saliente 443/tcp hacia VM-ControlCentral |
| VM-ControlCentral | Servidor de la organización | `control-central` (Docker), PostgreSQL, Grafana | `[a definir]` | `[a definir]` | 8000 (API), 3000 (Grafana), 443 (si hay reverse proxy TLS) |
| VM-SIEM | SIEM/HIDS | Wazuh manager + indexer + dashboard | `[a definir]` | `[a definir]` | 1514/1515 (agentes), 55000 (API Wazuh) |
| VM-Mail | Servidor de correo | Mailu (Postfix/Dovecot/Rspamd) | `[a definir]` | `[a definir]` | 25/587 (SMTP), 993 (IMAP si aplica) |

> Pendiente completar IPs/VLANs reales cuando se levanten las VMs del laboratorio del curso
> (Anexo A de `LETRA.md`) y volcarlas también en `02-registro-activos-mcu5.xlsx`.

## 5. Escenarios (el "+1")

| ID escenario | Descripción | Vistas que toca | Observación/validación |
|---|---|---|---|
| ESC-01 | Usuario abre la bóveda offline (sin red) y opera con normalidad | Lógica, Procesos, Física | Validar RNF-01 desconectando la VM-Cliente de red y repitiendo el flujo |
| ESC-02 | Alta de credencial genera evento, llega a Wazuh y dispara correo | Lógica, Procesos, Física | Validar cobertura 100% de eventos (KPI 6.4) |
| ESC-03 | Cambio de contraseña maestra dispara notificación inmediata y alerta de severidad alta | Lógica, Procesos | Validar MTTD/MTTR |
| ESC-04 | Ataque de fuerza bruta contra la maestra local activa el delay adaptativo y luego la regla Wazuh | Procesos, Física | Es también RT-01/RT-08 del Red Team; debe quedar cubierto antes de la entrega |
| ESC-05 | Exportación de bóveda con contraseña de transporte y reimportación en otro cliente | Lógica, Desarrollo | Validar RF-12 y que el archivo exportado no sea descifrable sin la contraseña de transporte |
| ESC-06 | Falsificación de un evento con clave de otro agente es rechazada por `control-central` | Lógica, Procesos | Mitigación directa de RT-02 |

---

## Guía para completar en auditoría

Seguir el orden de `plantilla/plantilla-arquitectura-4más1.md` (físico → lógico → procesos → demo
en vivo → vínculo con controles MCU/BCU/ISO).

## Check de aceptación

- [ ] Las 4 vistas + escenarios están completas.
- [ ] El diagrama físico coincide con lo realmente desplegado (pendiente: reemplazar IPs `[a definir]`).
- [ ] Cada control de auditoría puede anclarse a una vista.
- [ ] Acompañado en repositorio con capturas reales (`docs/evidencias/`).
- [ ] Diagrama gráfico (`docs/diagrama-arquitectura.png`) generado a partir de esta tabla — **pendiente**.

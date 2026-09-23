# Bitácora de Trabajo — Tarea 1 (Blue Team)

> Formato según `plantilla/isaca/99-bitacora-trabajo.md`. Reglas: registro **diario** (no la noche
> anterior a la entrega), cada miembro firma sus entradas, **no se omiten fallos**, horas en **UTC**,
> cada entrada referencia evidencia real en `docs/evidencias/` cuando exista.

## Control del documento

| Campo | Valor |
|---|---|
| Código | SI-BIT-99 |
| Dueño | Equipo Blue Team |
| Período | 15/09/2026 → 07/10/2026 (pre-entrega) |
| Versión | 1.0 |

---

## Entradas

---
Fecha: 15/09/2026
Equipo: Blue
Responsable: [completar nombre de quien firma]
---

## Actividad: Análisis de la letra y definición de arquitectura/stack

- **Fase**: Diseño
- **Duración**: (completar)
- **Tarea realizada**: Se leyó `LETRA.md` completo (RF-01 a RF-17, RNF-01 a RNF-11, calendario,
  criterios de evaluación y proceso de auditoría MCU 5.0 perfil Avanzado). A partir de eso se
  definió el enfoque de arquitectura y el stack tecnológico para el Blue Team:
  - **Cliente offline** (`cliente-gestor/`): Tauri (Rust + React/TS). Argon2id como KDF,
    XChaCha20-Poly1305 como cifrado AEAD de la bóveda, SQLite embebido como almacenamiento,
    TOTP (RFC 6238) + WebAuthn/Windows Hello como MFA local, eventos de auditoría firmados
    con JWS (clave privada **por agente**, no una clave compartida global — así un cliente
    comprometido no puede forjar eventos de otro).
  - **Control central** (`control-central/`): API FastAPI + PostgreSQL, RBAC de usuarios,
    verificación de firma JWS, reenvío a SIEM, disparo de correo.
  - **SIEM/HIDS**: Wazuh (agente en clientes + manager), con reglas custom para fuerza bruta
    de maestra, borrado masivo y cambio de contraseña maestra (RF-10).
  - **Correo**: Mailu (Postfix+Dovecot+Rspamd empaquetado), con foco explícito en configurar
    bien SPF/DKIM/DMARC porque es exactamente lo que el Red Team va a atacar (RT-04).
  - **Dashboard/SOAR**: Grafana sobre Postgres/Wazuh para KPIs; TheHive queda como decisión
    abierta (requiere Cassandra+Elasticsearch propios — evaluar si conviene resolver RF-13
    con una tabla `Incident` propia en `control-central` en su lugar).
  - **Wazo**: se descarta por ahora (es explícitamente opcional en la letra); se reconsidera
    solo si sobra tiempo después de H4.
  - Se armó además un plan de fases alineado al calendario obligatorio (H1 21/09, H2 29/09,
    H3 02/10, H4 07/10 pre-entrega, H5 14/10 defensa) y una lista de puntos no negociables
    (perfil MCU 5.0 Avanzado con Excel completos, arquitectura 4+1 **y** C4 antes de la demo,
    orden fijo de auditoría, gate duro de Proteger/Detectar demostrables en vivo, cobertura
    100% de eventos, zero-knowledge real, bitácora diaria, tag `v1.0` + hash SHA-256 el 07/10).
- **Herramienta / comando**: Lectura y análisis de `tarea1-gestor-contrasenas/LETRA.md`.
- **Resultado**: Éxito. Stack y enfoque acordados; ver detalle también en `README.md` (raíz de
  la tarea) y en la estructura de carpetas creada el mismo día (siguiente entrada).
- **Evidencia anexa**: (pendiente — agregar captura/export de la conversación si se decide
  guardarla en `docs/evidencias/`).
- **Incidencia / hallazgo**: Ninguna.
- **Observaciones**: Esta decisión de stack es la base para todo lo que sigue; cualquier cambio
  (p. ej. cambiar Tauri por Electron, o Wazuh por Security Onion) debe registrarse acá con el
  motivo, porque la arquitectura 4+1/C4 y el registro de activos dependen de esto.

---
Fecha: 15/09/2026
Equipo: Blue
Responsable: [completar nombre de quien firma]
---

## Actividad: Esqueleto del repositorio

- **Fase**: Diseño
- **Duración**: (completar)
- **Tarea realizada**: Se creó la estructura de carpetas del proyecto siguiendo el stack
  definido en la entrada anterior:
  - `cliente-gestor/` (Tauri): módulos `crypto`, `vault`, `auth`, `events`, `generator` del
    lado Rust, y `pages`/`components`/`lib` del lado frontend. Todo con stubs y TODOs, sin
    lógica implementada todavía.
  - `control-central/` (FastAPI): routers `events`, `users`, `dashboard`; `core/config.py`,
    `core/security.py`, `core/mfa.py`; modelos `Event`, `User`, `Incident`; `db/`; `siem/
    wazuh_forwarder.py`. También stubs con TODOs.
  - `infra/`: `docker-compose.yml` con `control-central` + `postgres` + `grafana` + `thehive`
    (este último marcado como pendiente de decisión); `README.md` en `infra/wazuh/` e
    `infra/mailu/` explicando que esos stacks se generan con las herramientas oficiales
    (wazuh-docker, asistente de Mailu) en vez de inventar un compose propio desactualizable.
  - `scripts/`: `generar_evento_prueba.sh` (para probar el pipeline evento→SIEM→correo antes
    de tener el cliente terminado) y `verificar_integridad_tag.sh` (hash SHA-256 del tag de
    entrega, pedido en la sección 8.2 de `LETRA.md`).
  - `.gitignore` en la raíz de la tarea (node_modules, target, venv, `.env`, `*.vault`, `keys/`).
- **Herramienta / comando**: creación manual de archivos (aún no se corrió `npm install` ni
  `cargo build`; el esqueleto no compila todavía, es solo estructura + contratos).
- **Resultado**: Parcial (a propósito). El esqueleto define dónde va cada cosa y qué falta
  (cada `README.md` de módulo tiene una lista de pendientes), pero no hay funcionalidad real.
- **Evidencia anexa**: árbol de directorios en el commit inicial del repositorio (pendiente de
  `git init` + primer commit).
- **Incidencia / hallazgo**: Ninguna. Nota para el registro de activos: todavía no hay
  repositorio Git inicializado en `tsi-obligatorio/` — hay que hacerlo antes de repartir tareas
  entre compañeros para evitar trabajar sobre carpetas sueltas sin control de versiones.
- **Observaciones**: Próximo paso: completar `docs/00-arquitectura-4mas1.md` y
  `docs/00-arquitectura-c4.md` con este mismo stack (hito H1, vence 21/09/2026).

---

---
Fecha: 22/09/2026
Equipo: Blue
Responsable: [completar nombre de quien firma]
---

## Actividad: Registro de activos (Markdown + Excel MCU 5.0)

- **Fase**: Diseño
- **Duración**: (completar)
- **Tarea realizada**: Se completó el inventario de activos a partir de la arquitectura ya
  definida (`docs/00-arquitectura-c4.md`, `docs/00-arquitectura-4mas1.md`) y del esqueleto real
  del repositorio. Se cargaron 14 activos (A01-A14) cubriendo cliente-gestor, bóveda, clave de
  firma de eventos, control-central, PostgreSQL, claves públicas de agentes, RBAC del panel,
  Wazuh (manager y agentes), Mailu, Grafana, gestión de incidentes, el propio repositorio Git y
  la segmentación de red del laboratorio. Se completó tanto `docs/02-Registro-Activos.md` como
  la hoja `Activos` de `plantilla/mcu5/excel/02-registro-activos-mcu5.xlsx` (mismo contenido,
  formato exigido para MCU 5.0 ID-01), preservando el estilo/formato original del Excel.
- **Herramienta / comando**: Python + `openpyxl` (`pip install openpyxl`) para editar el `.xlsx`
  manteniendo encabezado, bordes y anchos de columna del template.
- **Resultado**: Éxito. Ambos documentos quedan alineados; los datos de infraestructura (IPs,
  VLANs) siguen como pendiente explícito hasta levantar las VMs del laboratorio (Anexo A).
- **Evidencia anexa**: (pendiente — agregar captura del Excel abierto en `docs/evidencias/`).
- **Incidencia / hallazgo**: Ninguna. Nota: el hito H1 (aprobación de arquitectura y RF/RNF)
  vencía el 21/09/2026; esta entrada es del 22/09, un día después — a mencionar en la próxima
  revisión de cronograma con el equipo.
- **Observaciones**: Próximo paso natural: `03-analisis-riesgos` (usa este inventario como
  insumo directo) y el Excel `03-matriz-raci-mcu5.xlsx`.

---
Fecha: 22/09/2026
Equipo: Blue
Responsable: [completar nombre de quien firma]
---

## Actividad: Análisis de riesgos (12 riesgos derivados de RT-01..RT-12)

- **Fase**: Diseño
- **Duración**: (completar)
- **Tarea realizada**: Se completó `docs/03-Analisis-Riesgos.md` usando como catálogo de amenazas
  los propios objetivos del Red Team (RT-01 a RT-12 de la sección 7.2 de `LETRA.md`), en vez de
  inventar amenazas genéricas — así el registro de riesgos queda alineado 1 a 1 con lo que el
  Red Team va a intentar entre el 28/10 y el 09/11. Se evaluaron los 12 riesgos (probabilidad,
  impacto, nivel según la matriz 5x5), se armó el plan de tratamiento y la sección de riesgo
  residual. Quedaron 4 riesgos en nivel **Alto** (R01 fuerza bruta de maestra, R03 IDOR/XSS/SQLi
  en control-central, R06 bypass de MFA, R10 exfiltración por errores de la app, R11 OSINT del
  repositorio) marcados como tratamiento obligatorio antes del 07/10/2026.
- **Herramienta / comando**: Redacción manual sobre la plantilla `plantilla/isaca/03-analisis-riesgos.md`.
- **Resultado**: Éxito (borrador v0.1). Falta la aceptación formal del riesgo residual por el RSI
  (hoy son fechas objetivo, no firmas reales) y la v1 formal que el cronograma de la letra
  marcaba para el 18/09/2026 — ya pasada.
- **Evidencia anexa**: (pendiente).
- **Incidencia / hallazgo**: Igual que la entrada anterior, quedamos con el cronograma de la letra
  atrasado (H1 vencía 21/09, análisis de riesgos v1 vencía 18/09). Registrado para que el equipo
  decida cómo recuperar el atraso antes del 07/10.
- **Observaciones**: Próximo paso natural en el orden de la matriz de documentación (sección 8.1
  de `LETRA.md`): `01-Politica-Seguridad.md` (aún pendiente, vence 30/09) y `09-Gestion-Accesos.md`.

## Check de aceptación (repetir por período de entrega)

- [ ] Registro diario sin lagunas superiores a 2 días.
- [ ] Cada miembro firma sus entradas (reemplazar los `[completar nombre]` de arriba).
- [ ] Cada hallazgo/incidente tiene su entrada de bitácora asociada.
- [ ] Cada control auditado del Excel MCU 5.0 puede relacionarse con una o más entradas de acá.

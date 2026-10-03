# Bitácora de Trabajo — Tarea 1 (Blue Team)

> Formato según `plantilla/isaca/99-bitacora-trabajo.md`. Reglas: registro **diario** (no la noche
> anterior a la entrega), cada miembro firma sus entradas, **no se omiten fallos**, horas en **UTC**,
> cada entrada referencia evidencia real en `docs/evidencias/` cuando exista.

## Control del documento

| Campo | Valor |
|---|---|
| Código | SI-BIT-99 |
| Dueño | Equipo Blue Team (Horacio Duarte, Pablo Morales, Andrés Varela) |
| Período | 15/09/2026 → 07/10/2026 (pre-entrega) |
| Versión | 1.0 |

---

## Entradas

---
Fecha: 15/09/2026
Equipo: Blue
Responsable: Andrés Varela
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
Responsable: Andrés Varela
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
Responsable: Andrés Varela
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
  la hoja `Activos` de `docs/mcu5/excel/02-registro-activos-mcu5.xlsx` (mismo contenido,
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
Responsable: Andrés Varela
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

---
Fecha: 22/09/2026
Equipo: Blue
Responsable: Andrés Varela
---

## Actividad: Política de Seguridad de la Información

- **Fase**: Diseño
- **Duración**: (completar)
- **Tarea realizada**: Se completó `docs/01-Politica-Seguridad.md` (función **Gobernar** de MCU
  5.0), después de tener ya la arquitectura, el inventario de activos y el análisis de riesgos —
  a propósito, para que la política refleje lo que realmente se diseñó y no una declaración
  genérica. Define alcance (cliente-gestor, control-central, infra, repositorio, personas),
  principios de seguridad ligados a decisiones concretas (zero-knowledge, AEAD + JWS, RBAC, Zero
  Trust), la jerarquía hacia las políticas específicas todavía pendientes (accesos, monitoreo,
  incidentes, continuidad) y roles/responsabilidades.
- **Herramienta / comando**: Redacción manual sobre `plantilla/isaca/01-politica-seguridad.md`.
- **Resultado**: Éxito (borrador v0.1). Sin aprobación formal todavía — queda pendiente que el
  RSI del equipo la apruebe antes de la auditoría del 14/10/2026.
- **Evidencia anexa**: (pendiente).
- **Incidencia / hallazgo**: Ninguna.
- **Observaciones**: Con esto quedan cubiertos los primeros 4 documentos de la matriz de la
  sección 8.1 de `LETRA.md` (arquitectura 4+1/C4, registro de activos, análisis de riesgos,
  política de seguridad). Siguiente en la matriz: `09-Gestion-Accesos.md` (23-30/09/2026).

---
Fecha: 22/09/2026
Equipo: Blue
Responsable: Andrés Varela
---

## Actividad: Gestión de accesos (diseño; evidencia pendiente)

- **Fase**: Diseño
- **Duración**: (completar)
- **Tarea realizada**: Se completó `docs/09-Gestion-Accesos.md` (función Proteger): modelo de
  autenticación (maestra + Argon2id, TOTP, WebAuthn/Windows Hello), política de factor mínimo por
  operación (2FA obligatorio para cambio de maestra y panel de `control-central`), gestión de
  identidades del panel (RBAC), reglas de secretos y política de contraseñas por sistema (RF-04).
  Se dejó una tabla explícita de **qué capturas de evidencia faltan y cuándo van a poder tomarse**
  (enroll TOTP/WebAuthn, config del hash, delay de fuerza bruta, login con 2FA), porque hoy el
  módulo de auth es solo un esqueleto (`cliente-gestor/src-tauri/src/auth/`,
  `control-central/app/core/mfa.py`) y no tiene sentido fabricar evidencia de algo que no corre.
- **Herramienta / comando**: Redacción manual sobre `plantilla/isaca/09-gestion-accesos.md`.
- **Resultado**: Éxito (diseño). Ningún ítem del check de aceptación que requiere evidencia real
  está marcado todavía — a propósito, para no declarar un control no demostrable en la auditoría.
- **Evidencia anexa**: Ninguna todavía (ver tabla "Evidencia pendiente" del propio documento).
- **Incidencia / hallazgo**: Ninguna.
- **Observaciones**: Este documento queda "abierto" hasta H3 (02/10): a medida que se implemente
  cada mecanismo, hay que volver acá a tildar el check de aceptación y agregar la captura real en
  `docs/evidencias/`.

---
Fecha: 23/09/2026
Equipo: Blue
Responsable: Andrés Varela
---

## Actividad: Matriz RACI (reparto de roles por componente)

- **Fase**: Diseño
- **Duración**: (completar)
- **Tarea realizada**: Se completó `docs/mcu5/excel/03-matriz-raci-mcu5.xlsx` (vencía hoy,
  23/09/2026, según la sección 8.1 de `LETRA.md`) reemplazando los roles genéricos del template
  ("Analista/SOC", "Admin IAM", etc.) por los tres integrantes reales del equipo, repartidos por
  componente: **Horacio Duarte** = `cliente-gestor` (bóveda, cripto, MFA local); **Pablo
  Morales** = `control-central` + infraestructura (API, Wazuh, Mailu, backups); **Andrés
  Varela** = RSI/coordinación, monitoreo y documentación. Se completaron los 7 procesos del
  template (gestión de incidentes, accesos, riesgos, vulnerabilidades, capacitación,
  continuidad, notificación a autoridades) con R/A/C/I, documento de referencia y evidencia
  (marcando como pendiente la de los documentos aún no escritos).
- **Herramienta / comando**: Python + `openpyxl`, mismo enfoque que para el registro de activos.
- **Resultado**: Éxito. Queda como base para repartir el trabajo real de las próximas semanas
  (07-monitoreo-logs, 10-vulnerabilidades, 06-continuidad, 04-incidentes, 12-notificación).
- **Evidencia anexa**: (pendiente — captura del Excel en `docs/evidencias/`).
- **Incidencia / hallazgo**: Ninguna.
- **Observaciones**: El reparto por componente fue una decisión del equipo (confirmada por
  Andrés en esta sesión); si en la práctica el trabajo se reparte distinto, hay que volver a
  este Excel y actualizarlo, porque el auditor puede pedir consistencia entre la RACI y quién
  demuestra cada control en la auditoría del 14/10.

---
Fecha: 23/09/2026
Equipo: Blue
Responsable: Andrés Varela
---

## Actividad: Excel de controles MCU 5.0 perfil Avanzado (47 controles)

- **Fase**: Diseño
- **Duración**: (completar)
- **Tarea realizada**: Se completó `docs/mcu5/excel/01-controles-mcu5-perfil-avanzado.xlsx`
  (Gobernar, Identificar, Proteger, Detectar, Responder, Recuperar — 47 controles en total),
  marcando **Aplica = Sí/N.A.** en cada uno y completando **evidencia necesaria** y **cómo se
  demuestra** con referencias reales a lo ya construido (`docs/`, `docs/mcu5/excel/`,
  `infra/`, código del esqueleto) en vez de dejar el texto genérico del template. Resultado: 45
  controles en Sí (la mayoría de Proteger/Detectar/Responder/Recuperar quedan honestamente
  "pendiente de implementación", con fecha objetivo) y 2 en **N.A. justificado**:
  - *Presupuesto y recursos de seguridad* (Gobernar): no aplica un presupuesto monetario en un
    proyecto académico; el recurso real son las horas del equipo, trazadas en esta bitácora.
  - *Monitoreo continuo de la red / NIDS* (Detectar): decisión de alcance del equipo de no
    implementar Security Onion/Suricata/Zeek (explícitamente opcional en la letra) y concentrar
    la detección en Wazuh (HIDS/SIEM).
- **Herramienta / comando**: Python + `openpyxl`.
- **Resultado**: Éxito. Esta planilla queda como el tracker maestro que el auditor va a recorrer
  control por control el 14/10; conviene revisarla cada vez que se implemente algo nuevo (semanal,
  según la letra) para ir pasando filas de "pendiente" a evidencia real.
- **Evidencia anexa**: (pendiente — captura del Excel en `docs/evidencias/`).
- **Incidencia / hallazgo**: Al completarla quedó explícito que casi todo Proteger/Detectar
  (MFA, cifrado, Wazuh, Mailu, Grafana) sigue sin implementación real — es información valiosa
  para priorizar las próximas dos semanas antes del 07/10, no una sorpresa a evitar sino algo para
  registrar (no se omiten fallos ni huecos).
- **Observaciones**: Con este Excel completo, ya cubrimos las tres planillas de apoyo del MCU 5.0
  pendientes de esta semana (activos, RACI, controles). Falta la de bitácora
  (`04-bitacora-planilla.xlsx`), que se llena en paralelo a este mismo archivo `.md`.

---
Fecha: 23/09/2026
Equipo: Blue
Responsable: Andrés Varela
---

## Actividad: Excel de bitácora + reorganización de los Excel MCU 5.0

- **Fase**: Diseño
- **Duración**: (completar)
- **Tarea realizada**: Se completó `04-bitacora-planilla.xlsx` como espejo de las 8 entradas ya
  cargadas en este mismo archivo `.md` (mismas fechas, responsable y tareas, con la columna
  "Hora (UTC)" marcada como `N/D` porque no veníamos registrando la hora exacta — a corregir de
  acá en adelante). Además, siguiendo indicación explícita, se movieron los 4 Excel completados
  (activos, RACI, controles, bitácora) desde `plantilla/mcu5/excel/` hacia
  `docs/mcu5/excel/` (mismo nombre de subcarpeta que en `plantilla/`), y se **restauraron los
  originales en blanco** en `plantilla/mcu5/excel/` (vía `git checkout` del commit inicial), para
  que esa carpeta siga siendo material de referencia del curso sin material propio de la Tarea 1
  encima. Se actualizaron todas las referencias cruzadas en `01-Politica-Seguridad.md`,
  `02-Registro-Activos.md` y este mismo archivo para apuntar a la nueva ruta.
- **Herramienta / comando**: Python + `openpyxl`; `git checkout <commit-inicial> -- <archivos>`
  para restaurar los templates.
- **Resultado**: Éxito. `plantilla/` queda limpia; `docs/mcu5/excel/` es ahora la fuente de verdad
  de los 4 Excel de esta tarea.
- **Evidencia anexa**: (pendiente).
- **Incidencia / hallazgo**: Ninguna, más allá de la falta de registro de hora exacta ya anotada.
- **Observaciones**: De acá en más, si se generan más Excel de apoyo, copiarlos directamente a
  `docs/mcu5/excel/` y dejar `plantilla/` intacta.

---
Fecha: 25/09/2026
Equipo: Blue
Responsable: Pablo Morales
Hora (UTC): 23:06
---

## Actividad: Contraste de la letra con el repositorio y ubicación de los Excel MCU

- **Fase**: Documentación
- **Duración**: 0,8 h
- **Tarea realizada**: Se contrastó `LETRA.md` (RF-01..17, hitos H1–H5, matriz §8.1) con el
  árbol de `tarea1-gestor-contrasenas/`. Se confirmó que los 4 Excel MCU de la Tarea 1 están
  en `docs/mcu5/excel/` y que `plantilla/mcu5/excel/` queda como template del curso.
- **Herramienta / comando**: lectura de `LETRA.md`, `docs/README.md` y `docs/mcu5/excel/*.xlsx`.
- **Resultado**: Éxito. Fuente de verdad de los Excel confirmada.
- **Evidencia anexa**: (ninguna captura; los Excel ya están versionados en `docs/mcu5/excel/`).
- **Incidencia / hallazgo**: Hueco de bitácora 23/09→25/09 (límite de 2 días).
- **Observaciones**: No se fabricaron evidencias de MFA/Wazuh/correo.

---
Fecha: 25/09/2026
Equipo: Blue
Responsable: Pablo Morales
Hora (UTC): 23:54
---

## Actividad: Recorrido del esqueleto de control-central e infra

- **Fase**: Documentación
- **Duración**: 0,8 h
- **Tarea realizada**: Se recorrió `control-central/` e `infra/`. Solo `GET /healthz` responde;
  eventos, usuarios y dashboard lanzan `NotImplementedError`. El compose define
  `blue-team-net` (API, Postgres, Grafana; TheHive incompleto).
- **Herramienta / comando**: lectura de `control-central/app/main.py` e
  `infra/docker-compose.yml`.
- **Resultado**: Parcial. Ningún RF demostrable; el código sigue en stub (entrada 15/09).
- **Evidencia anexa**: (ninguna captura — revisión de código).
- **Incidencia / hallazgo**: TheHive vs tabla `Incident` sigue abierta (RF-13).
- **Observaciones**: Próximo paso de este rol: persistencia + `POST /api/events` sobre
  `blue-team-net`.

---
Fecha: 26/09/2026
Equipo: Blue
Responsable: Pablo Morales
Hora (UTC): 00:48
---

## Actividad: Decisión de despliegue H2 (Docker, no VMs)

- **Fase**: Documentación
- **Duración**: 0,9 h
- **Tarea realizada**: Se resolvió no bloquear H2 (29/09) por VMs. El prototipo se levanta
  con Docker en un host, red `blue-team-net`, enganchando Wazuh y Mailu (stacks oficiales).
  El cliente Tauri queda nativo. Las filas VM-* de la 4+1 son nodos lógicos. El Anexo A no
  está en el repo; se consulta a la cátedra sin frenar el compose.
- **Herramienta / comando**: lectura de `LETRA.md` §6.1/§10, `docs/00-arquitectura-4mas1.md`
  e `infra/docker-compose.yml`.
- **Resultado**: Éxito. Criterio de despliegue para H2 registrado.
- **Evidencia anexa**: (ninguna captura — decisión documentada acá).
- **Incidencia / hallazgo**: Anexo A ausente del repositorio.
- **Observaciones**: Docs `07` y `06` se diseñan en paralelo (28/09–01/10) sin tildar KPIs
  ni restores hasta que corran.

---
Fecha: 26/09/2026
Equipo: Blue
Responsable: Pablo Morales
Hora (UTC): 23:00
---

## Actividad: Parte 1 H2 — sesión Postgres y healthz en el compose

- **Fase**: Implementación
- **Duración**: 1,0 h
- **Tarea realizada**: Se conectó `control-central` a PostgreSQL (`app/db/session.py`) y
  `GET /healthz` ahora hace `SELECT 1` (503 si la BD no responde). En
  `infra/docker-compose.yml`: healthcheck de Postgres, `depends_on` con condición,
  puerto 5432 publicado, TheHive detrás del profile `thehive` para que `docker compose up`
  no lo levante. `POST /api/events` sigue sin implementar (siguiente commit).
- **Herramienta / comando**: edición de `session.py`, `main.py`, `docker-compose.yml`.
  Verificación: `cd infra; docker compose up --build -d` y
  `curl http://localhost:8000/healthz` (pendiente en esta máquina: el entorno del agente
  no pudo spawn Docker).
- **Resultado**: Parcial. Código listo; falta confirmar `{"status":"ok","database":"up"}`
  en el host.
- **Evidencia anexa**: (captura de healthz cuando corra el compose en `docs/evidencias/`).
- **Incidencia / hallazgo**: Shell del agente con EPERM al invocar Docker; verificar en
  terminal local.
- **Observaciones**: Docs `07`/`06` no se tocan en este commit. Siguiente parte: persistir
  eventos en Postgres.

---
Fecha: 27/09/2026
Equipo: Blue
Responsable: Pablo Morales
Hora (UTC): 01:15
---

## Actividad: Parte 2 H2 — persistir POST /api/events en Postgres

- **Fase**: Implementación
- **Duración**: 0,8 h
- **Tarea realizada**: Se agregó el modelo `audit_events` y `POST /api/events/` (201)
  guarda tipo, sistema, agente, timestamp, firma JWS e IP. `GET /api/events/` lista
  los últimos 20. La firma se almacena con `firma_valida=null` (verificación JWS y
  SIEM/correo quedan para el siguiente commit). `create_all` al arrancar crea la tabla.
- **Herramienta / comando**: `app/models/event.py`, `app/api/events.py`, `app/main.py`;
  prueba: `bash scripts/generar_evento_prueba.sh` y `GET /api/events/`.
- **Resultado**: Código listo. Verificar en el host tras `docker compose up --build`.
- **Evidencia anexa**: (captura del 201 + listado cuando corra, en `docs/evidencias/`).
- **Incidencia / hallazgo**: Ninguna nueva. JWS sigue sin verificar a propósito.
- **Observaciones**: Docs `07`/`06` siguen pendientes (diseño). No mezclar Mailu/Wazuh acá.

---
Fecha: 27/09/2026
Equipo: Blue
Responsable: Pablo Morales
Hora (UTC): 02:02
---

## Actividad: Verificación de POST /api/events en el host

- **Fase**: Prueba
- **Duración**: 0,2 h
- **Tarea realizada**: Tras `docker compose up --build`, se envió un evento
  `alta_credencial` / `sistema-prueba` / `agente-dev-01`. La API respondió 201 con
  `id=1`, `firma_valida` vacío y `occurred_at`/`received_at` coherentes.
- **Herramienta / comando**: `Invoke-RestMethod` POST `http://localhost:8000/api/events/`.
- **Resultado**: Éxito. Persistencia demostrable en vivo.
- **Evidencia anexa**: (guardar captura del 201 en `docs/evidencias/26-post-evento-alta.png`).
- **Incidencia / hallazgo**: Ninguna.
- **Observaciones**: Espejar esta fila en `04-bitacora-planilla.xlsx`. Siguiente parte H2:
  verificar JWS o forwarder/correo (no ambos en el mismo commit).

---
Fecha: 27/09/2026
Equipo: Blue
Responsable: Pablo Morales
Hora (UTC): 02:15
---

## Actividad: Parte 3 H2 — forwarder JSONL para el SIEM

- **Fase**: Implementación
- **Duración**: 0,5 h
- **Tarea realizada**: Cada `POST /api/events/` escribe una línea JSON (sin secretos ni
  `firma_jws`) en `infra/logs/audit-events.jsonl`, montado en el contenedor como
  `/var/log/control-central/`. Si el disco falla, el 201 no se revierte: Postgres
  sigue siendo la fuente de verdad. Wazuh y las 3 reglas RF-10 no se levantan en
  este commit; el formato queda documentado en `infra/wazuh/README.md`.
- **Herramienta / comando**: `app/siem/wazuh_forwarder.py`, `SIEM_LOG_PATH`, volumen
  `./logs` en `docker-compose.yml`.
- **Resultado**: Código listo. Verificar con un POST y `Get-Content infra/logs/audit-events.jsonl`.
- **Evidencia anexa**: (captura de la línea JSONL en `docs/evidencias/`).
- **Incidencia / hallazgo**: Ninguna.
- **Observaciones**: Docs `07` se pueden empezar a diseñar sobre este formato. Siguiente:
  correo o JWS (un commit cada uno).

---
Fecha: 27/09/2026
Equipo: Blue
Responsable: Pablo Morales
Hora (UTC): 02:26
---

## Actividad: Verificación del forwarder JSONL en el host

- **Fase**: Prueba
- **Duración**: 0,2 h
- **Tarea realizada**: Tras rebuild del compose, un POST `alta_credencial` escribió
  `event_id: 2` en `infra/logs/audit-events.jsonl` (programa, tipo, sistema, agente,
  timestamps, `firma_valida: null`). Sin `firma_jws` ni secretos.
- **Herramienta / comando**: `Get-Content .\logs\audit-events.jsonl`
- **Resultado**: Éxito. Pipeline evento → Postgres → log demostrable.
- **Evidencia anexa**: (captura de la línea JSONL en `docs/evidencias/`).
- **Incidencia / hallazgo**: Ninguna.
- **Observaciones**: Espejar en `04-bitacora-planilla.xlsx`. Siguiente commit: correo
  (RF-07) o verificación JWS.

---
Fecha: 27/09/2026
Equipo: Blue
Responsable: Pablo Morales
Hora (UTC): 02:45
---

## Actividad: Parte 4 H2 — notificación SMTP (Mailpit)

- **Fase**: Implementación
- **Duración**: 0,5 h
- **Tarea realizada**: Cada evento RF-07 (alta/mod/borrado/cambio de maestra) dispara
  un correo SMTP con metadata solamente. `intento_fallido_maestra` no manda mail.
  H2 usa Mailpit (`localhost:8025`); Mailu se enchufa después con las mismas
  `SMTP_*`. Si SMTP falla, el 201 no se revierte.
- **Herramienta / comando**: `app/notify/mailer.py`; servicio `mailpit` en
  `infra/docker-compose.yml`.
- **Resultado**: Código listo. Verificar: rebuild, POST, abrir http://localhost:8025.
- **Evidencia anexa**: (captura de Mailpit en `docs/evidencias/`).
- **Incidencia / hallazgo**: Ninguna. Mailu (SPF/DKIM) queda para un commit de infra.
- **Observaciones**: Docs `07`/`12` se pueden mencionar este canal. Siguiente: JWS.

---
Fecha: 27/09/2026
Equipo: Blue
Responsable: Pablo Morales
Hora (UTC): 02:52
---

## Actividad: Verificación de correo SMTP (Mailpit)

- **Fase**: Prueba
- **Duración**: 0,2 h
- **Tarea realizada**: Tras rebuild, un POST `alta_credencial` llegó a Mailpit
  (`http://localhost:8025`) con asunto y cuerpo de metadata (id, tipo, sistema,
  agente, timestamps). Sin secretos. Formato correcto.
- **Herramienta / comando**: UI Mailpit :8025
- **Resultado**: Éxito. RF-07 demostrable en el prototipo H2 (canal Mailpit).
- **Evidencia anexa**: (captura de Mailpit en `docs/evidencias/`).
- **Incidencia / hallazgo**: Ninguna. Mailu (SPF/DKIM) sigue pendiente.
- **Observaciones**: Espejar en `04-bitacora-planilla.xlsx`. Siguiente: verificación JWS.

---
Fecha: 27/09/2026
Equipo: Blue
Responsable: Pablo Morales
Hora (UTC): 18:46
---

## Actividad: Parte 5 H2 — verificación JWS por agente

- **Fase**: Implementación
- **Duración**: 0,7 h
- **Tarea realizada**: El central verifica RS256 con `keys/agentes/{agente_id}.pub.pem`
  (una clave por agente, no HMAC global). Los claims del JWS deben coincidir con
  tipo/sistema/agente/timestamp. Se persiste igual si falla (`firma_valida=false`)
  para que el SIEM vea el intento. Scripts: `generar_par_agente.py` y
  `generar_evento_prueba.py`. La privada no se versiona (`keys/` en .gitignore).
- **Herramienta / comando**: `app/core/security.py`; volumen `../keys/agentes` en compose.
- **Resultado**: Código listo. Verificar: par de claves, POST firmado (`true`) y
  `--sin-firma` (`false`).
- **Evidencia anexa**: (captura de ambos 201 en `docs/evidencias/`).
- **Incidencia / hallazgo**: Ninguna.
- **Observaciones**: Grafana (paso 2) es el siguiente commit; la UI ya está en :3000.

---
Fecha: 29/09/2026
Equipo: Blue
Responsable: Pablo Morales
Hora (UTC): 20:29
---

## Actividad: Compose — cwd y archivos .env de laboratorio

- **Fase**: Implementación
- **Duración**: 0,3 h
- **Tarea realizada**: El error `no configuration file provided` era el directorio
  de trabajo, no un compose borrado (`infra/docker-compose.yml` sigue ahí). Se
  agregó `compose.yaml` en la raíz de la tarea (include). Las plantillas
  `infra/.env.example` y `infra/control-central.env.example` se versionan (solo
  placeholders). Las copias `.env` / `control-central.env` quedan gitignored.
- **Herramienta / comando**: `docker compose up --build -d` desde
  `tarea1-gestor-contrasenas/` o `infra/`.
- **Resultado**: Pendiente verificar en el host.
- **Evidencia anexa**: (ninguna).
- **Incidencia / hallazgo**: Compose no busca el YAML en carpetas padre.
- **Observaciones**: Postgres/Grafana de ejemplo usan `changeme`. Credenciales
  reales fuera del repo. Copiar `*.example` y editar la copia local.
Responsable: Horacio Duarte
Hora (UTC): 10:34
---

## Actividad: Revisión y actualización de arquitecturas 4+1 y C4

- **Fase**: Diseño / Documentación
- **Duración**: (completar)
- **Tarea realizada**: Se contrastaron `docs/00-arquitectura-4mas1.md` y
  `docs/00-arquitectura-c4.md` contra `LETRA.md` y se corrigieron omisiones de RF-13 y RNF-06.
  Se decidió documentar la gestión de incidentes mediante un modelo `Incident` interno en
  `control-central`, implementado conceptualmente con FastAPI + SQLAlchemy + PostgreSQL, con
  estados `abierto`, `en análisis` y `resuelto`, y flujo alerta Wazuh → incidente → asignación →
  evidencias → cierre. TheHive queda registrado como integración futura, no como alternativa
  del despliegue actual.
  También se documentaron los flujos de protección en tránsito: TLS 1.2+, reverse proxy para
  el acceso a FastAPI, validación de certificados, conexión API/PostgreSQL, SMTP submission/TLS
  hacia Mailu, comunicación TLS del agente con Wazuh y acceso protegido de Grafana. En el Nivel 1
  del C4 se separaron los actores humanos (RSI/auditor) de los sistemas Grafana, Wazuh y Mailu.
- **Resultado**: Éxito documental. La arquitectura queda alineada con RF-13 y RNF-06, pero la
  implementación de `Incident`, los endpoints de gestión, TLS productivo y la integración real
  con Wazuh siguen pendientes de implementación y evidencia.
- **Evidencia anexa**: Documentos actualizados `docs/00-arquitectura-4mas1.md` y
  `docs/00-arquitectura-c4.md`. Capturas de despliegue y pruebas: pendientes.
- **Incidencia / hallazgo**: Se detectó que el C4 mantenía TheHive como decisión abierta y que
  no describía con suficiente precisión TLS ni separaba personas de sistemas externos. Se corrigió
  en la documentación; queda pendiente reflejar la decisión en cualquier documento antiguo que
  todavía mencione TheHive como alternativa actual.
- **Observaciones**: Espejar esta entrada en `docs/mcu5/excel/04-bitacora-planilla.xlsx` cuando
  se actualice la planilla.

---
Fecha: 27/09/2026
Equipo: Blue
Responsable: Horacio Duarte
Hora (UTC): 15:45
---

## Actividad: Elaboración del diagrama de arquitectura

- **Fase**: Diseño / Documentación
- **Duración**: (completar)
- **Tarea realizada**: Se elaboró el diagrama general de arquitectura del Gestor de
  Contraseñas con Control Centralizado, tomando como base las vistas 4+1 y C4. Se representaron
  el cliente offline, `control-central`, PostgreSQL, Wazuh, Mailu, Grafana y la gestión de
  incidentes mediante `Incident` interno, junto con los principales flujos de eventos, alertas y
  notificaciones.
- **Resultado**: Éxito documental. El diseño del diagrama queda alineado con la arquitectura
  4+1/C4 y preparado para la presentación inicial de la demo. La exportación o incorporación de
  `docs/diagrama-arquitectura.png` 
- **Evidencia anexa**: `docs/00-arquitectura-4mas1.md` y `docs/00-arquitectura-c4.md`; imagen
  `docs/diagrama-arquitectura.png`

---
Fecha: 02/10/2026
Equipo: Blue
Responsable: Horacio Duarte
Hora (UTC): 02:10
---

## Actividad: Bóveda local cifrada y primera ventana del gestor

- **Fase**: Implementación / Prueba
- **Duración**: 2 h
- **Tarea realizada**: Se implementó la cripto de la bóveda (Argon2id, 19 MiB / 2 iteraciones, y XChaCha20-Poly1305) y el archivo SQLite: crear, abrir, cerrar, alta, consulta, modificación y borrado. La contraseña y las notas se guardan cifradas. La pantalla mínima quedó en `cliente-gestor/src/App.tsx`. `cargo test` en `src-tauri`: 11 pruebas ok (cripto + bóveda, incluido que el secreto no queda en texto plano en el archivo).
- **Herramienta / comando**: `cargo test` en `cliente-gestor/src-tauri`. Luego, en `cliente-gestor`: `npm install` y `npm run tauri dev`.
- **Resultado**: Éxito. La ventana del gestor abre en el host. La prueba manual (crear bóveda, alta, ver, editar, cerrar, abrir con la misma maestra, rechazar otra maestra, y comprobar que el secreto no está en claro en `boveda.sqlite`) quedó escrita en `cliente-gestor/README.md`.
- **Evidencia anexa**: (pendiente — captura de la ventana abierta y del rechazo de maestra incorrecta en `docs/evidencias/`).
- **Incidencia / hallazgo**: Tres cortes antes de abrir la ventana, todos corregidos. (1) `npm install` falló con `UNABLE_TO_VERIFY_LEAF_SIGNATURE`; se reintentó con `NODE_OPTIONS=--use-system-ca`. (2) `vite` 8 no convive con `@vitejs/plugin-react` 4; se fijó Vite 5.4. (3) `cargo` no podía crear `src-tauri/target` (`Acceso denegado`, os error 5). Se creó la carpeta y `cargo build` terminó. Queda un aviso de caché incremental en rutas con `ñ` (`contraseñas`); no impide generar el ejecutable.
- **Observaciones**: El cliente todavía no emite eventos al control central. Siguiente paso: generador y política por sistema (RF-04, RF-05). Espejar esta fila en `docs/mcu5/excel/04-bitacora-planilla.xlsx`. Rust quedó instalado en el host (`rustc` 1.99); hace falta una terminal nueva para que `cargo` esté en el PATH.

---
Fecha: 02/10/2026
Equipo: Blue
Responsable: (firmar: integrante que ejecutó la prueba)
Hora (UTC): 02:40
---

## Actividad: Eventos firmados al control central y cambio de maestra

- **Fase**: Implementación / Prueba
- **Duración**: 2 h
- **Tarea realizada**: El cliente firma cada alta, modificación, borrado, cambio de maestra e intento fallido de apertura con JWS RS256 (una clave por agente, guardada en los datos de la app) y hace POST a `http://localhost:8000/api/events/`. Si no hay red, el evento queda en `cola-eventos.jsonl` y el próximo envío reintenta la cola. Un 4xx no se encola. El JWS incluye `exp` porque python-jose lo exige; el central sigue comparando agente, tipo, sistema y timestamp. El cambio de maestra reencripta secretos, notas e historial con un salt nuevo. También quedó el generador (contraseña y frase) y la política por sistema (longitud, caracteres, regex, historial y vencimiento).
- **Herramienta / comando**: `cargo test` en `cliente-gestor/src-tauri` (23 pruebas ok, contando la copia cifrada del paso siguiente). `npx tsc --noEmit` ok. `docker compose up --build -d` desde `infra/`. `GET http://localhost:8000/healthz` respondió `{"status":"ok","database":"up"}`.
- **Resultado**: Éxito de código y de stack. La ventana del gestor levantó con `npm run tauri dev`. La prueba manual del correo en Mailpit y de `firma_valida: true` está escrita en `cliente-gestor/README.md`; la captura sigue pendiente.
- **Evidencia anexa**: (pendiente — Mailpit con el alta y la respuesta de `/api/events/` en `docs/evidencias/`).
- **Incidencia / hallazgo**: (1) Docker Desktop estaba instalado y el motor apagado: `open //./pipe/dockerDesktopLinuxEngine` no existe hasta abrir la app. El puerto 8080 lo usa el propio `com.docker.backend`; la API va al 8000. (2) Pegar en PowerShell el prompt `PS C:\...>` o el texto del error anterior hace que `PS` se ejecute como `Get-Process`. (3) El build de la imagen falló con `CERTIFICATE_VERIFY_FAILED` al bajar paquetes de PyPI (la misma red que cortó npm). Se agregó `--trusted-host` en `control-central/Dockerfile` y el compose terminó. (4) Aviso de caché incremental por la `ñ` en la ruta; el ejecutable igual arranca.
- **Observaciones**: Espejar esta fila en `docs/mcu5/excel/04-bitacora-planilla.xlsx`. La clave pública hay que copiarla a `keys/agentes` desde la ventana antes del primer alta, si se quiere `firma_valida: true`.

---
Fecha: 02/10/2026
Equipo: Blue
Responsable: (firmar: integrante que ejecutó la prueba)
Hora (UTC): 02:55
---

## Actividad: Buscador, favoritos y copia cifrada de la bóveda

- **Fase**: Implementación
- **Duración**: 0,5 h
- **Tarea realizada**: La lista filtra por texto (sistema, usuario, categoría), por favoritos y por vencidas (RF-15). Cada credencial se puede marcar favorita; la columna se agrega al abrir una bóveda vieja. Exportar e importar escriben un archivo `.gex` cifrado con contraseña de transporte distinta de la maestra (RF-12). El secreto no queda en claro en ese archivo.
- **Herramienta / comando**: `cargo test vault::store::tests::favorito_y_copia_cifrada_viajan_a_otra_boveda` ok. Suite completa: 23 pruebas ok.
- **Resultado**: Éxito automático. La ventana en `tauri dev` recompiló y quedó con el buscador y **Copia cifrada**. Falta la prueba manual en la ventana y la captura.
- **Evidencia anexa**: (pendiente).
- **Incidencia / hallazgo**: Ninguna en el test. El aviso de vencimiento al control central (RF-17) no se envía: la API solo acepta alta, modificación, borrado, cambio de maestra e intento fallido. Hace falta un tipo nuevo en el central si se quiere ese correo.
- **Observaciones**: Cerrar y volver a abrir la bóveda para que aparezca la columna `favorito`. Espejar en la planilla Excel.

---
Fecha: 03/10/2026
Equipo: Blue
Responsable: Horacio Duarte
Hora (UTC): 16:50
---

## Actividad: Aviso de vencimiento al control central e incidentes (RF-17, RF-13)

- **Fase**: Implementación
- **Duración**: 1,0 h
- **Tarea realizada**: Se cruzó la letra (sección 3) con el código. El aviso local de vencimiento ya existía; faltaba el evento. El central acepta `vencimiento_credencial`, lo persiste, lo escribe en el JSONL y manda el correo. Al abrir la bóveda, si hay credenciales vencidas, la ventana lo dice y el cliente firma un evento por sistema, con la misma cola offline que el resto. Quedó el registro de incidentes: tabla `incidents` y `POST/GET /api/incidents/` más `PATCH /api/incidents/{id}`. El alta exige un evento ya guardado, arranca en `abierto` y al pasar a `resuelto` guarda la fecha. El estado `en_analisis` es el «en análisis» de RF-13. Severidad `S0`–`S3`, la de `docs/04-gestion-incidentes.md`.
- **Herramienta / comando**: `python -m py_compile` de la API. `cargo test sistemas_vencidos_avisa_solo_los_que_ya_pasaron` en `cliente-gestor/src-tauri`: 1 prueba ok (solo entra el sistema ya vencido).
- **Resultado**: Éxito de código. No se reconstruyó el compose en esta sesión: la tabla `incidents` aparece al arrancar de nuevo el control central.
- **Evidencia anexa**: (pendiente — captura del aviso en la ventana, del 201 del evento y de un incidente en `docs/evidencias/`).
- **Incidencia / hallazgo**: `cargo test` no pudo crear `src-tauri/target` (`Acceso denegado`, os error 5), el mismo corte del 02/10. Se creó la carpeta y la prueba terminó. Queda el aviso de caché incremental; no impide el ejecutable. `npx tsc --noEmit` intentó bajar `tsc` del registry y falló con `UNABLE_TO_VERIFY_LEAF_SIGNATURE`; en esta máquina no está el TypeScript local. La prueba de Rust sí corrió.
- **Observaciones**: Espejar esta fila en `docs/mcu5/excel/04-bitacora-planilla.xlsx`. TheHive no se usa.

---
Fecha: 03/10/2026
Equipo: Blue
Responsable: Horacio Duarte
Hora (UTC): 17:15
---

## Actividad: Panel de KPIs y Grafana (RF-09, RF-14, sección 6.4)

- **Fase**: Implementación
- **Duración**: 0,4 h
- **Tarea realizada**: `GET /api/dashboard/kpis` deja de lanzar `NotImplementedError`. Calcula agentes con evento en las últimas 24 h, última recepción, prueba SMTP sin mandar correo, volumen por tipo, firmas inválidas, intentos fallidos de maestra e incidentes por estado. MTTD es el promedio desde `occurred_at` del evento hasta `creado_en` del incidente. MTTR es el promedio hasta `resuelto_en`. La cobertura cuenta cuántos de los cuatro tipos de RF-07 ya llegaron al menos una vez. El uptime es el del proceso. Grafana provisiona el datasource Postgres y el tablero **Control central** (eventos, incidentes abiertos, agentes, última sincronización, volumen y listado). El compose pasa al contenedor de Grafana el usuario y la clave de Postgres.
- **Herramienta / comando**: `python -m py_compile app/api/dashboard.py app/core/runtime.py app/main.py`.
- **Resultado**: Éxito de código. Hay que `docker compose up --build` para ver el endpoint y el tablero en el puerto 3000.
- **Evidencia anexa**: (pendiente — respuesta de `/api/dashboard/kpis` y captura del tablero en `docs/evidencias/`).
- **Incidencia / hallazgo**: La tasa de falsos positivos queda en null. No hay alertas del manager de Wazuh todavía; poner 0 hubiera dicho que la detección no falla.
- **Observaciones**: Espejar en la planilla Excel.

---
Fecha: 03/10/2026
Equipo: Blue
Responsable: Horacio Duarte
Hora (UTC): 17:30
---

## Actividad: Usuarios del panel, delay de la maestra y reglas Wazuh (RF-11, RF-16, RF-10)

- **Fase**: Implementación
- **Duración**: 0,5 h
- **Tarea realizada**: Usuarios del panel en `panel_users`. El primer `POST /api/users/` no pide token y queda como admin; los siguientes los crea un admin. El hash es `argon2id` o `bcrypt`, a elección. El login devuelve un token. El listado no muestra la contraseña ni el secreto TOTP. El enroll es `POST /api/users/{id}/totp/enroll` y se activa con `POST /api/users/{id}/totp/confirmar`; después el login exige el código de 6 dígitos. Los eventos del agente siguen entrando por JWS, sin esta sesión. Una maestra incorrecta espera 1 s, luego 2 s, 4 s y se queda en 8 s; una apertura correcta pone el contador en cero. Argon2id sigue siendo el costo memory-hard. Las reglas de RF-10 quedaron en `infra/wazuh/local_rules.xml`: 5 intentos fallidos del mismo agente en 2 minutos, 5 borrados en 2 minutos, y cambio de maestra en el primer evento. `localfile-audit.xml` apunta al JSONL.
- **Herramienta / comando**: vector TOTP de RFC 6238 (tiempo 59, secreto de la RFC, código `287082`). `cargo test la_demora_crece_y_se_frena_en_ocho_segundos`: ok. `xml.etree` leyó los cinco `rule id` del archivo de Wazuh.
- **Resultado**: Éxito de código. WebAuthn y Windows Hello siguen sin endpoint. Las reglas no se dispararon contra un manager levantado.
- **Evidencia anexa**: (pendiente — login con TOTP, la espera al fallar la maestra, y una alerta de Wazuh en `docs/evidencias/`).
- **Incidencia / hallazgo**: El mismo aviso de caché incremental de `cargo` (os error 5 al cerrar la sesión incremental). La prueba igual terminó en 5 s porque `target/` ya existía.
- **Observaciones**: Falta despliegue, no lógica nueva de estos tres RF: manager Wazuh, retención de 90 días, Mailu (SPF/DKIM) y TLS. Espejar las tres entradas del 03/10 en `docs/mcu5/excel/04-bitacora-planilla.xlsx`.

---
Fecha: 03/10/2026
Equipo: Blue
Responsable: Horacio Duarte
Hora (UTC): 17:45
---

## Actividad: Alertas de las reglas, retención de 90 días y TOTP local (RF-10, RNF-07, RF-02)

- **Fase**: Implementación
- **Duración**: 0,6 h
- **Tarea realizada**: Cada evento que cumple las reglas de `local_rules.xml` abre una fila en `alerts`: cambio de maestra en el acto, y 5 intentos fallidos o 5 borrados del mismo agente en 2 minutos. `GET /api/alerts/` las lista y `PATCH /api/alerts/{id}` marca si es falso positivo; el panel calcula la tasa solo sobre las ya clasificadas. Al arrancar, el central borra eventos con más de 90 días que no sean origen de un incidente o de una alerta. En la bóveda, **TOTP de esta bóveda** genera el secreto, lo guarda cifrado con la maestra y, una vez confirmado, la próxima apertura lo exige. Un código TOTP inválido usa la misma espera que una maestra incorrecta, pero no se registra como `intento_fallido_maestra`.
- **Herramienta / comando**: `cargo test totp` en `cliente-gestor/src-tauri`: 2 pruebas ok (el código actual verifica, y sin código la bóveda no abre).
- **Resultado**: Éxito de código. El manager de Wazuh no se levantó; las alertas de esta sesión son las de la API.
- **Evidencia anexa**: (pendiente — una alerta en `/api/alerts/` y la ventana pidiendo TOTP, en `docs/evidencias/`).
- **Incidencia / hallazgo**: `totp-rs` 5.7 no trae `generate_secret` ni `get_url` sin features extra. El secreto se genera con `OsRng` (20 bytes) y la URL otpauth se arma en el módulo. El aviso de caché incremental de `cargo` se repitió; las pruebas terminaron igual.
- **Observaciones**: WebAuthn/Windows Hello, Mailu y TLS siguen fuera. Espejar esta fila en el Excel de bitácora.

---
Fecha: 03/10/2026
Equipo: Blue
Responsable: Horacio Duarte
Hora (UTC): 18:40
---

## Actividad: QR de enrolamiento TOTP y ruta inicial de la bóveda (RF-02)

- **Fase**: Implementación
- **Duración**: 0,5 h
- **Tarea realizada**: El alta de TOTP de la bóveda muestra un QR de la URI `otpauth`, para escanearlo con una app de autenticación. El código de 6 dígitos de esa app confirma el alta y es el que pide la próxima apertura. Se sacó de la pantalla de inicio el cálculo del código: la misma ventana no puede ser el segundo factor. Si la cámara no lee el QR, **No puedo escanear** deja el secreto para carga manual. El archivo que propone el login pasó de `boveda.sqlite` a `boveda-prueba.sqlite`, en la carpeta de datos de la app (`uy.tsi.gestor-contrasenas`).
- **Herramienta / comando**: `npm install qrcode.react` con `NODE_OPTIONS=--use-system-ca`. `npx tsc --noEmit` en `cliente-gestor`: sin errores.
- **Resultado**: Éxito de código. Hay que volver a correr `npm run tauri dev` para ver el QR, porque entró una dependencia nueva.
- **Evidencia anexa**: (pendiente — captura del QR y de la apertura con el código de la app, en `docs/evidencias/`).
- **Incidencia / hallazgo**: Una primera versión mostraba el código de 6 dígitos también en el login. Se quitó: con el secreto a la vista, ese número no agrega un factor.
- **Observaciones**: WebAuthn/Windows Hello, Mailu y TLS siguen fuera. Espejar esta fila en el Excel de bitácora.

## Check de aceptación (repetir por período de entrega)

- [ ] Registro diario sin lagunas superiores a 2 días.
- [x] Cada miembro firma sus entradas (Andrés Varela hasta el 23/09; Pablo Morales el
  25/09–26/09 UTC; Horacio Duarte el 27/09, el 02/10 y el 03/10).
- [ ] Cada hallazgo/incidente tiene su entrada de bitácora asociada.
- [ ] Cada control auditado del Excel MCU 5.0 puede relacionarse con una o más entradas de acá.

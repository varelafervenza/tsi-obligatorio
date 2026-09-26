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

## Check de aceptación (repetir por período de entrega)

- [ ] Registro diario sin lagunas superiores a 2 días.
- [x] Cada miembro firma sus entradas (Andrés Varela hasta el 23/09; Pablo Morales el
  25/09–26/09 UTC; Horacio Duarte firma las suyas cuando corresponda).
- [ ] Cada hallazgo/incidente tiene su entrada de bitácora asociada.
- [ ] Cada control auditado del Excel MCU 5.0 puede relacionarse con una o más entradas de acá.

# Guía de pruebas — Gestor de Contraseñas con Control Centralizado

> Paso a paso de todas las pruebas de la entrega. Para cada una se indica **quién la puede hacer**:
>
> - **Persona:** se hace sólo con esta guía, sin instalar nada más que lo indicado.
> - **Sesión de Claude:** conviene pedir ayuda a una sesión de Claude si algo falla, para leer el error,
>   calcular un código, interpretar un reporte o cambiar un archivo.
>
> Todos los comandos se corren desde la raíz `tarea1-gestor-contrasenas/` salvo que se indique otra carpeta.

---

## Requisitos

| Requisito | Para qué | Dónde se consigue |
|---|---|---|
| Docker Desktop abierto | Backend, base de datos, Grafana, Mailpit | docker.com |
| Python 3.12 y `pip` | Scripts de claves, eventos y simulación | python.org |
| `pip install "python-jose[cryptography]"` | Firma de eventos | `pip` |
| Navegador web | Mailpit, Grafana, `/docs` de la API | — |
| Windows 10, o Windows 11 sin Smart App Control | Ejecutar la app de escritorio | Ver el README de evaluación |
| Git Bash o WSL | Scripts `.sh` de backup y restauración | Viene con Git para Windows |

---

## Parte 1 — Backend (`control-central`)

### B-01. Levantar el stack

| Campo | Detalle |
|---|---|
| **Quién** | Persona |
| **Pasos** | 1. `cd infra` · 2. `copy .env.example .env` · 3. `copy control-central.env.example control-central.env` · 4. `docker compose up --build -d` · 5. `docker compose ps` |
| **Resultado esperado** | Los cuatro servicios en `running`, y Postgres en `healthy`. |
| **Si falla** | Pedir ayuda a una sesión de Claude con el mensaje de error. |

### B-02. Salud de la API

| Campo | Detalle |
|---|---|
| **Quién** | Persona |
| **Pasos** | Abrir `http://localhost:8001/healthz` en el navegador. |
| **Resultado esperado** | `{"status":"ok","database":"up"}` |

### B-03. Documentación de la API

| Campo | Detalle |
|---|---|
| **Quién** | Persona |
| **Pasos** | Abrir `http://localhost:8001/docs`. |
| **Resultado esperado** | Página con las rutas de events, alerts, incidents, users y dashboard. |

### B-04. Par de claves del agente

| Campo | Detalle |
|---|---|
| **Quién** | Persona (si el comando funciona); sesión de Claude si falla. |
| **Pasos** | `python scripts/generar_par_agente.py` |
| **Resultado esperado** | Se crean `keys/agentes/agente-dev-01.pem` (privada, no se commitea) y `agente-dev-01.pub.pem`. |

### B-05. Evento firmado

| Campo | Detalle |
|---|---|
| **Quién** | Persona |
| **Pasos** | `python scripts/generar_evento_prueba.py` |
| **Resultado esperado** | `201` con `"firma_valida":true`. |

### B-06. Evento sin firma

| Campo | Detalle |
|---|---|
| **Quién** | Persona |
| **Pasos** | `python scripts/generar_evento_prueba.py --sin-firma` |
| **Resultado esperado** | `201` con `"firma_valida":false`. |

### B-07. Listado de eventos

| Campo | Detalle |
|---|---|
| **Quién** | Persona |
| **Pasos** | Abrir `http://localhost:8001/api/events/?limit=10`. Activar "Dar formato al texto" para verlo ordenado. |
| **Resultado esperado** | Los últimos eventos con su `firma_valida`. |

### B-08. Log para el SIEM

| Campo | Detalle |
|---|---|
| **Quién** | Persona |
| **Pasos** | En PowerShell, desde `infra/`: `Get-Content .\logs\audit-events.jsonl -Tail 3` |
| **Resultado esperado** | Una línea JSON por evento, con `programa`, `tipo`, `agente_id` y `firma_valida`. **No** aparece `firma_jws` ni ningún secreto. |

### B-09. Correo de notificación

| Campo | Detalle |
|---|---|
| **Quién** | Persona |
| **Pasos** | Abrir `http://localhost:8025`. |
| **Resultado esperado** | Un correo "Alta de credencial" por cada evento de alta, cambio, borrado o cambio de maestra. Los intentos fallidos de maestra **no** mandan correo. |

### B-10. KPIs del panel

| Campo | Detalle |
|---|---|
| **Quién** | Persona |
| **Pasos** | Abrir `http://localhost:8001/api/dashboard/kpis`. |
| **Resultado esperado** | Campos `funcionamiento`, `alertas`, `incidentes` y `kpis`. La cobertura sube con cada tipo de evento que llega. |

### B-11. Tablero de Grafana

| Campo | Detalle |
|---|---|
| **Quién** | Persona |
| **Pasos** | Abrir `http://localhost:3000`. Entrar con el usuario y la contraseña de `infra/.env` (`GRAFANA_ADMIN_USER` y `GRAFANA_ADMIN_PASSWORD`). Ir a **Dashboards → Control central**. |
| **Resultado esperado** | Paneles con datos: eventos, incidentes abiertos, agentes, última sincronización (un tiempo relativo, como "hace 5 minutos"), volumen por tipo e incidentes. |

### B-12. Simulación de casos de uso (alertas)

| Campo | Detalle |
|---|---|
| **Quién** | Persona |
| **Pasos** | `python scripts/simular_casos_uso.py` · Luego `GET /api/alerts/` en el navegador. |
| **Resultado esperado** | 11 eventos con `201`. Aparecen alertas `fuerza_bruta_maestra`, `borrado_masivo` y `cambio_maestra`. |
| **Nota** | Si se corre dos veces, aparecen alertas duplicadas de `cambio_maestra`. Es esperado; anotarlo. |

### B-13. Incidente desde una alerta

| Campo | Detalle |
|---|---|
| **Quién** | Persona. Las rutas de incidentes no piden token, así que se pueden probar desde `http://localhost:8001/docs` con **Try it out**. |
| **Pasos** | 1. Mirar `GET /api/alerts/` y anotar el `evento_id` de una alerta. · 2. En `POST /api/incidents/` mandar `{"evento_origen_id": <ese id>, "severidad": "S1", "descripcion": "...", "asignado_a": "..."}`. · 3. Anotar el `id` del incidente. · 4. En `PATCH /api/incidents/{incidente_id}` mandar `{"estado":"en_analisis"}`, y después `{"estado":"resuelto"}`. |
| **Resultado esperado** | Estado `abierto`, luego `en_analisis`, luego `resuelto` con `resuelto_en` con fecha. |
| **Nota** | Las rutas de usuarios (B-14) sí piden token. `/docs` no tiene un botón para cargarlo: se manda en PowerShell con el encabezado `Authorization` (ver B-14). |

### B-14. Usuario del panel con TOTP

| Campo | Detalle |
|---|---|
| **Quién** | Persona si tiene una app de autenticación en el celular. Sesión de Claude si no tiene celular (ver la nota). |
| **Pasos** | En PowerShell, desde la raíz de la tarea: 1. Crear el usuario (el primero no necesita token): `Invoke-RestMethod -Method Post -Uri http://localhost:8001/api/users/ -ContentType "application/json" -Body '{"email":"rsi@correo.local","password":"clave-de-prueba","rol":"admin","algoritmo_hash":"argon2id"}'` 2. Hacer login y guardar el token: `$T = (Invoke-RestMethod -Method Post -Uri http://localhost:8001/api/users/login -ContentType "application/json" -Body '{"email":"rsi@correo.local","password":"clave-de-prueba"}').access_token` 3. Enrolar TOTP con el token (devuelve un `secreto`): `Invoke-RestMethod -Method Post -Uri http://localhost:8001/api/users/1/totp/enroll -Headers @{Authorization="Bearer $T"}` 4. Agregar el secreto en la app de autenticación (escanear el QR o cargarlo a mano). 5. Confirmar con el código de 6 dígitos de la app: `Invoke-RestMethod -Method Post -Uri http://localhost:8001/api/users/1/totp/confirmar -Headers @{Authorization="Bearer $T"} -ContentType "application/json" -Body '{"codigo":"123456"}'` 6. Probar el login **sin** código (debe dar 401) y **con** código (debe dar token). |
| **Resultado esperado** | `mfa_habilitado: true`, login sin código rechazado, login con código aceptado. |
| **Nota** | Sin celular, el código se calcula con `app/core/mfa.py` (función `codigo_totp`). Eso lo hace mejor una sesión de Claude. |

### B-15. Restauración de la base

| Campo | Detalle |
|---|---|
| **Quién** | Persona. Sesión de Claude si falla. |
| **Pasos** | 1. `bash scripts/backup_bd.sh` · 2. `bash scripts/restaurar_bd_prueba.sh` |
| **Resultado esperado** | "verificando sha256: OK", tiempo de restauración y `OK` en las cuatro tablas. |

### B-16. Backup de logs

| Campo | Detalle |
|---|---|
| **Quién** | Persona |
| **Pasos** | Copiar `infra/logs/audit-events.jsonl` a `infra/backups/` y comparar el número de líneas. |
| **Resultado esperado** | Misma cantidad de líneas en ambas. |

---

## Parte 2 — Cliente de escritorio (Windows)

Antes de empezar: el instalador está en el artefacto `cliente-gestor-instalador` del workflow de GitHub
Actions. En Windows 11, Smart App Control tiene que estar desactivado (ver el README de evaluación).

### C-01. Instalar y abrir la app

| Campo | Detalle |
|---|---|
| **Quién** | Persona |
| **Pasos** | Ejecutar el `.exe`, seguir el instalador, abrir **Gestor de Contraseñas**. |
| **Resultado esperado** | La ventana abre en la pantalla de apertura de la bóveda. |

### C-02. Crear la bóveda

| Campo | Detalle |
|---|---|
| **Quién** | Persona |
| **Pasos** | Dejar la ruta por defecto en **Archivo de la bóveda**, escribir una **Contraseña maestra** y tocar **Crear bóveda**. |
| **Resultado esperado** | Se abre la pantalla "Bóveda abierta". |

### C-03. Alta de credencial

| Campo | Detalle |
|---|---|
| **Quién** | Persona |
| **Pasos** | En **Alta**: **Sistema** (por ejemplo `sistema-app`), **Usuario**, **Contraseña**. Tocar **Agregar**. |
| **Resultado esperado** | La credencial aparece en la lista **Credenciales**. |

### C-04. Enviar el evento y verificar la firma

| Campo | Detalle |
|---|---|
| **Quién** | Persona. La clave de la app se instala con un botón; no hace falta copiarla a mano. |
| **Pasos** | 1. En **Control central**, poner la **Carpeta de claves públicas del central** (la carpeta `keys/agentes` del repo, con su ruta completa) y tocar **Guardar y copiar clave pública**. · 2. Dar de alta otra credencial. · 3. Abrir `http://localhost:8001/api/events/?limit=5`. |
| **Resultado esperado** | El primer alta (antes de guardar la clave) tiene `firma_valida: false`. El alta después de guardar tiene `firma_valida: true`. La app muestra "Evento enviado al control central". |

### C-05. Buscador, filtros y favoritos

| Campo | Detalle |
|---|---|
| **Quién** | Persona |
| **Pasos** | Escribir en **Buscar**, cambiar **Filtro** (favoritos, vencidas) y **Categoría**. Marcar una credencial con la estrella. |
| **Resultado esperado** | La lista se filtra y el favorito queda marcado. |

### C-06. Generador de contraseñas

| Campo | Detalle |
|---|---|
| **Quién** | Persona |
| **Pasos** | En **Alta**, expandir **Política de este sistema**, elegir el modo (contraseña o frase) y tocar el generador. |
| **Resultado esperado** | Se genera una contraseña que cumple la longitud y la expresión regular de la política. |

### C-07. Política por sistema

| Campo | Detalle |
|---|---|
| **Quién** | Persona |
| **Pasos** | Definir longitud mínima, expresión regular, historial (cuántas anteriores no se pueden repetir) y días de validez. Tocar **Guardar política**. Intentar una contraseña que no cumple. |
| **Resultado esperado** | La app rechaza la contraseña que no cumple y muestra el motivo. |

### C-08. Cambio de maestra

| Campo | Detalle |
|---|---|
| **Quién** | Persona |
| **Pasos** | Expandir **Cambiar contraseña maestra**, escribir la actual y la nueva, tocar el botón. Cerrar la bóveda y abrirla con la nueva. |
| **Resultado esperado** | Se abre con la nueva. La maestra anterior no sirve. Llega una alerta `cambio_maestra` y su correo (ver B-09 y B-12). |

### C-09. TOTP de la bóveda

| Campo | Detalle |
|---|---|
| **Quién** | Persona, con una app de autenticación en el celular. |
| **Pasos** | 1. Expandir **TOTP de esta bóveda** y tocar **Generar TOTP**. · 2. Escanear el QR con la app. · 3. Escribir el código de 6 dígitos y tocar **Confirmar TOTP**. · 4. Cerrar y abrir la bóveda. Sin el código, no abre. |
| **Resultado esperado** | La apertura pide el código de la app. |
| **Nota** | Si la cámara no lee el QR, expandir **No puedo escanear** y cargar el secreto a mano. |

### C-10. Espera por maestra incorrecta

| Campo | Detalle |
|---|---|
| **Quién** | Persona, con un cronómetro. |
| **Pasos** | Intentar abrir con una maestra incorrecta cuatro veces seguidas, midiendo cuánto tarda cada intento. |
| **Resultado esperado** | Las esperas son de 1, 2, 4 y después 8 segundos. Al abrir bien, el contador se reinicia. |

### C-11. Copia cifrada (exportar e importar)

| Campo | Detalle |
|---|---|
| **Quién** | Persona |
| **Pasos** | Expandir **Copia cifrada**, elegir **Archivo de la copia** y una **Contraseña de transporte** distinta de la maestra. Exportar. Importar el archivo `.gex` en otra bóveda con la misma contraseña de transporte. |
| **Resultado esperado** | Las credenciales aparecen en la otra bóveda. El archivo `.gex` no muestra las contraseñas en texto plano. |
| **Nota** | Si se pierde la contraseña de transporte, el `.gex` no se puede abrir. |

### C-12. Modo sin conexión

| Campo | Detalle |
|---|---|
| **Quién** | Persona |
| **Pasos** | 1. Detener el control central: `docker compose stop control-central` (desde `infra/`). · 2. En la app, dar de alta una credencial. · 3. Verificar que la app sigue funcionando y que aparece el evento en la cola: `cola-eventos.jsonl` en la carpeta de datos de la app (`%APPDATA%\uy.tsi.gestor-contrasenas`). · 4. Volver a levantar el control central: `docker compose start control-central`. · 5. Dar de alta otra credencial, o esperar al siguiente envío. |
| **Resultado esperado** | La app no se traba sin red. Los eventos quedan en la cola y se envían cuando el central vuelve. |
| **Nota** | Si la cola no se vacía, pedir ayuda a una sesión de Claude. |

### C-13. Aviso de vencimiento

| Campo | Detalle |
|---|---|
| **Quién** | Persona |
| **Pasos** | Crear un sistema con **Días de validez** de 1, esperar al día siguiente (o cambiar la fecha de la PC) y abrir la bóveda. |
| **Resultado esperado** | La app avisa de los sistemas vencidos y firma un evento `vencimiento_credencial` por cada uno. |
| **Nota** | Cambiar la fecha de la PC puede afectar a otras cosas; mejor esperar. |

### C-14. Windows Hello (fuera de la entrega)

No se prueba. Está documentado como limitación en `docs/09-Gestion-Accesos.md`.

### C-15. Conjunto mínimo de 22 sistemas

| Campo | Detalle |
|---|---|
| **Quién** | Persona |
| **Pasos** | En la app, abrir el archivo `cliente-gestor/datos-prueba/boveda-anexo-b.sqlite` con la maestra `maestra-de-prueba`. |
| **Resultado esperado** | La lista tiene 22 sistemas (correo, banca, VPN, Grafana, Wazuh, etc.). La contraseña de cada uno es `Lab-` + el nombre del sistema + `-2026`. Son datos ficticios. |

---

## Parte 3 — Pruebas automáticas y de calidad

### A-01. Tests del cliente en GitHub Actions

| Campo | Detalle |
|---|---|
| **Quién** | Persona |
| **Pasos** | En GitHub, abrir la pestaña **Actions**, elegir el workflow `cliente-gestor` y el último run. |
| **Resultado esperado** | Los jobs `test` e `instalador` en verde. `test` corre los tests de Rust de la bóveda, incluido el que arma la bóveda de 22 sistemas. |

### A-02. Tests del cliente en la máquina

| Campo | Detalle |
|---|---|
| **Quién** | Sesión de Claude (requiere Rust y las herramientas de Visual Studio). |
| **Pasos** | `cd cliente-gestor/src-tauri` y `cargo test`. |

### A-03. Análisis de dependencias y de código

| Campo | Detalle |
|---|---|
| **Quién** | Persona para correr los comandos. Sesión de Claude para interpretar el resultado. |
| **Pasos** | 1. `pip-audit -r control-central/requirements.txt` · 2. `bandit -r control-central/app -f txt` · 3. En `cliente-gestor/`: `npm audit --omit=dev` |
| **Resultado esperado** | Lo que ya está documentado en `docs/10-Gestion-Vulnerabilidades.md`. El aviso que puede seguir es el de `ecdsa` (V04, riesgo aceptado). |

---

## Limitaciones (no se prueban en esta entrega)

- **WebAuthn y Windows Hello:** `docs/09-Gestion-Accesos.md`.
- **TLS en el canal cliente → API:** `docs/00-arquitectura-c4.md`.
- **Mailu (correo con SPF/DKIM):** se usa Mailpit. La decisión está en `docs/00-arquitectura-c4.md`.
- **Indexer y dashboard de Wazuh:** no se levantan. El manager y el agente sí, con `docker compose up` desde `infra/`.
- **Escaneo de red y web (OpenVAS, nmap, nuclei):** pendiente.

---

## Resumen

| Parte | Pruebas | Persona | Con ayuda de Claude |
|---|---|---|---|
| Backend | B-01 a B-16 | B-01 a B-12, B-15, B-16 | B-04 si falla, B-14 si no hay celular, B-15 si falla |
| Cliente | C-01 a C-13 | Todas | C-12 si la cola no se vacía |
| Automáticas | A-01 a A-03 | A-01 y A-03 (comandos) | A-02 y la interpretación de A-03 |

# Pendientes para la entrega — Tarea 1

> Fuente única de lo que falta. Se actualiza en el **mismo commit** que cierra cada punto.
> Estado al **06/10/2026**. Pre-entrega: **07/10/2026**. Defensa: **14/10/2026**.
>
> Columna **Exigido por**: `Letra` = lo pide `LETRA.md`; `Propio` = compromiso que el equipo declaró en sus documentos.
> Responsable: `a confirmar` cuando no hay asignación en la RACI.

## Antes de la pre-entrega (07/10)

### Exigido por la letra

| # | Pendiente | Exigido por | Responsable | Estado |
|---|---|---|---|---|
| 1 | Tag `v1.0` y hash SHA-256 con `scripts/verificar_integridad_tag.sh` | Letra (congelamiento) | a confirmar | Pendiente |
| 2 | Video de la demo, de 5 minutos o menos | Letra | a confirmar | Pendiente |
| 3 | Cobertura de eventos desde la app: alta, modificación y borrado reales. Hoy el 100 % salió de un script | Letra (6.4) | a confirmar | Pendiente |

### Compromisos del análisis de riesgos (`docs/03-Analisis-Riesgos.md`, v2)

| # | Riesgo | Pendiente | Exigido por | Estado |
|---|---|---|---|---|
| 4 | R01 | Medidor de fortaleza de la maestra (no está en el código) | Propio | Pendiente |
| 5 | R03 | Pruebas de IDOR sobre la API y DAST básico | Propio | Pendiente |
| 6 | R10 | Barrido de patrones de secretos en logs y código | Propio | Pendiente |
| 7 | R13 | Quitar la publicación del puerto 5432 en `infra/docker-compose.yml`. Antes, confirmar que ningún script lo necesita | Propio | Pendiente |
| 8 | R05 | Publicar la API solo en `127.0.0.1`. Antes, confirmar que los evaluadores acceden desde su equipo | Propio | Pendiente |
| 9 | R02 | Regla de alerta por firma inválida repetida | Propio | Pendiente |
| 10 | R07 | CSP en Grafana y escape de campos de evento | Propio | Pendiente |
| 11 | R09 | Validar los parámetros del KDF al importar un `.gex` | Propio | Pendiente |

Ya cerrados: R06 (aviso al activar MFA) y R11 (hook de `gitleaks`), en `aaecc19`.

### Documentación

| # | Pendiente | Exigido por | Responsable | Estado |
|---|---|---|---|---|
| 12 | Firma del RSI en 01, 03, 06, 11, 12 y 28 | Letra | RSI | Pendiente |
| 13 | Renombrar `04-gestion-incidentes.md` a `04-Gestion-Incidentes.md` y actualizar `docs/README.md` | Propio | a confirmar | Pendiente |
| 14 | Actualizar las 24 filas "pendiente" de `docs/mcu5/excel/01-controles-mcu5-perfil-avanzado.xlsx` con evidencia real | Letra (MCU 5.0) | a confirmar | Pendiente |
| 15 | Espejar en `docs/mcu5/excel/04-bitacora-planilla.xlsx` las entradas desde el 06/10 | Letra (bitácora) | a confirmar | Pendiente |
| 16 | Revisar entradas de la bitácora sin firma o con hora/duración N/D | Propio | a confirmar | Pendiente |

### Equipo

| # | Pendiente | Exigido por | Responsable | Estado |
|---|---|---|---|---|
| 17 | Instalar el hook de pre-commit en los clones de Pablo y Horacio: `python -m pip install pre-commit` y `python -m pre_commit install` | Propio | Pablo Morales, Horacio Duarte | Pendiente |
| 18 | Probar la guía de pruebas con otra persona y en otra máquina | Propio | a confirmar | Pendiente |

## Antes de la defensa (14/10)

### Exigido por la letra

| # | Pendiente | Responsable | Estado |
|---|---|---|---|
| 19 | Wazuh manager levantado con al menos una alerta real, o limitación documentada | a confirmar | Pendiente |
| 20 | Mailu con SPF y DKIM, o justificar Mailpit ante la cátedra | a confirmar | Pendiente |
| 21 | Consulta escrita a la cátedra: Anexo A (no está en el repo) y postura sobre Mailpit y Wazuh | a confirmar | Pendiente |
| 22 | Medir uptime, RNF-02 (apertura < 2 s), RNF-05 (carga básica) y RNF-10 (rollback) | a confirmar | Pendiente |
| 23 | Escaneo de red y web (nmap, OpenVAS o nuclei), más Trivy y `cargo audit` | a confirmar | Pendiente |
| 24 | CVSS de V03 y V04 en `docs/10-Gestion-Vulnerabilidades.md` | a confirmar | Pendiente |
| 25 | Backup diario automático (tarea programada de Windows) y copia fuera del equipo | a confirmar | Pendiente |
| 26 | Custodia de `keys/agentes/`: decidir dónde se guarda la clave privada | RSI | Pendiente |
| 27 | Contactos oficiales de reporte (teléfono y correo) en `docs/12-Notificacion-Incidentes.md` | RSI | Pendiente |
| 28 | Subir FastAPI y starlette para cerrar V03. Se deja para el final por riesgo de regresión | a confirmar | Pendiente |

## Decisiones abiertas (sin resolver)

### Cola de eventos offline y vencimiento de la firma (hallazgo del 07/10)

Ver el detalle en `docs/00-arquitectura-c4.md`, "Limitación de alcance: cola de eventos offline y vencimiento
de la firma", y la fila R02 de `docs/03-Analisis-Riesgos.md`. Ideas en discusión, ninguna decidida todavía:

| Idea | Qué resuelve | Qué no resuelve |
|---|---|---|
| Extender el plazo de la firma a 8 horas | Que una demora normal (el equipo apagado durante la noche) no genere `firma_valida: false` | No evita que una demora más larga lo siga generando. Alarga la ventana de reúso de una firma vieja |
| Reintentar siempre al abrir y al cerrar la app | Vacía la cola en más momentos, sin esperar a la próxima operación | Si el usuario no abre la app seguido, la cola sigue esperando igual |
| Botón que se activa cuando hay eventos sin enviar | Visibilidad: el usuario sabe que hay algo pendiente y puede reintentar cuando quiera | No evita el vencimiento si el usuario no lo usa a tiempo |
| Avisar al cerrar que hay eventos sin enviar, con riesgo de firma vencida | Transparencia: el usuario sabe a qué se expone antes de cerrar | Requiere agregar manejo del evento de cierre de ventana (hoy no existe, verificado en el código) |
| Formulario al cerrar para sincronizar o cerrar avisando | Le da al usuario la decisión en el momento más probable de encontrar la cola llena | Es la opción con más superficie nueva de UI y de pruebas |

## Opcional (no exigido)

- Tests de retención de 90 días e incidentes (`control-central/tests/`).
- Dockerfile para correr los tests del cliente.
- Job de CI en Ubuntu.
- Página propia para gestionar incidentes y usuarios.
- Repetir la restauración de la base, o registrar otro incidente.
- Agregar el hash de cada commit y las horas reales a las entradas de la bitácora.

## Cerrado

| Fecha | Commit | Qué se cerró |
|---|---|---|
| 06/10 | `54f2ffb`, `ec8e6f3` | Backup, restauración con datos, guía de pruebas |
| 06/10 | `6cc98f4` | Declaración de aplicabilidad (93 controles) |
| 06/10 | `dd5dab2` | Notificación simulada del incidente 1 (12) |
| 06/10 | `3768fc1` | Informe final del Blue Team (28) |
| 06/10 | `49a99be` | Cobertura al 100 % con `modificacion_credencial` (script) |
| 06/10 | `6ba28d2` | MTTD desde la primera alerta del evento de origen |
| 06/10 | `c52b7c0` | MTTR desde la alerta; severidad S2 del incidente 1 |
| 06/10 | `cc52884` | Pruebas automáticas de MTTD, MTTR y cobertura |
| 06/10 | `fa4b30b` | Análisis de riesgos versión 2 (03) |
| 06/10 | `aaecc19` | Hook de `gitleaks`, aviso MFA (R06) y pruebas de firma JWS |

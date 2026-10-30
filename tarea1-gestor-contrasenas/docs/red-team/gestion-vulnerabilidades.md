# Gestión de vulnerabilidades — Red Team

**Sistema:** Tarea 1 — Gestor de contraseñas. **Estado:** preliminar. Documento basado en los campos de [la plantilla 10](../../../plantilla/isaca/10-gestion-vulnerabilidades.md). La tabla incluye hallazgos incorporados hasta el 27/10/2026; las fichas H01/H02 conservan su valoración del 23/10.

| Campo | Valor |
|---|---|
| Código / versión | SI-VUL-10-RT / 0.1 |
| Responsable del registro | Pendiente de identificar. |
| Informe asociado | [Informe Red Team](informe-red-team.md) |
| Referencias de la plantilla | MCU: Identificar/Proteger; ISO 27001 A.8.8; COBIT APO12/DSS05. No acredita cumplimiento general. |

## 1. Herramientas utilizadas

Lectura de código y scripts Python/HTTP con consultas GET para H01; Postman para el replay de H02. Corroboración con PostgreSQL, registros JSONL y Mailpit. Revisar la adecuación de Postman al requisito open source.

## 2. Registro de vulnerabilidades

| ID | Activo / debilidad | CVSS v3.x: puntaje y vector | Estado | Fecha de detección local | Fecha de remediación | Responsable de remediación |
|---|---|---|---|---|---|---|
| RT-H01 | API de auditoría/KPI sin autenticación | **Media — 5.3**, `CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:L/I:N/A:N` | Abierta; confirmación inicial, repetición posterior inconclusa. | 21/10/2026 | Sin corrección documentada. | Pendiente de asignar. |
| RT-H02 | Recepción de eventos sin deduplicación | **Baja — 3.7**, `CVSS:3.1/AV:N/AC:H/PR:N/UI:N/S:U/C:N/I:L/A:N` | Abierta; duplicación confirmada. | 21/10/2026 | Sin corrección documentada. | Pendiente de asignar. |
| RT-H03 | Reconfiguración incompleta desactiva TOTP de bóveda | **Pendiente de valorar** | Abierta; reproducción manual, evidencia final parcial. | 24/10/2026 | Sin corrección documentada. | Pendiente de asignar. |
| RT-H04 | Token del panel sin vencimiento temporal | **Pendiente de valorar** | Análisis estático; prueba temporal pendiente. | 25/10/2026 | Sin corrección documentada. | Pendiente de asignar. |

Los IDs se conservan para mantener trazabilidad con las pruebas y evidencias. P01 y P02 muestran controles efectivos en lo probado y no se incluyen como vulnerabilidades. Herramientas, reproducción e impacto por hallazgo: fichas siguientes. No hay retests documentados.

Seguimiento documental: RT-H01 → INC-2026-002; RT-H02 → INC-2026-003; RT-H03 → INC-2026-005; RT-H04 → INC-2026-006 en el [Registro de incidentes](registro-incidentes.md). El último es solo revisión de código, no un ataque ejecutado.

**RT-P07:** se observó la bóveda vacía abierta a los 11 minutos. [Ficha y capturas](RT-P07-inactividad-boveda.md). La ausencia de bloqueo se mantiene como observación sin ID de hallazgo ni CVSS hasta fijar el plazo esperado y valorar el escenario de acceso físico.

### Fichas técnicas

### RT-H01 — Lectura de auditoría y KPI sin autenticación

- **Debilidad:** falta de autenticación en rutas de lectura operacional; causa y procedimiento en [P04](RT-P04-acceso-sin-autenticacion.md).
- **Reproducción:** sin Authorization ni cookies, consultar `GET /api/events/?limit=1` y `GET /api/dashboard/kpis`; comparar con `GET /api/users/`.
- **Resultado e impacto:** inicialmente auditoría y KPI devolvieron 200 con datos; usuarios, 401. Se leyeron metadatos sin login/MFA. No se observaron contraseñas ni accesibilidad desde Internet.
- **Evidencia y límite:** [resultado instrumentado](evidencias/revision-inicial/resultado-reproduccion-aislada.json), sección `consultas_sin_authorization`, y script de la misma carpeta. No se conservó captura HTTP íntegra inicial. La nueva consulta terminó por timeout; evidencia manual de Postman pendiente.
- **Recomendación:** exigir autenticación y permisos para auditoría/KPI; limitar la salud pública a información mínima.
- **Clasificación:** CVSS v3.1 **5.3 — Media**. Vector: `CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:L/I:N/A:N`. CWE pendiente; MITRE T1213, relación contextual.
- **Justificación:** acceso HTTP por red (AV:N), solicitud simple (AC:L), sin cuenta (PR:N) ni interacción de otra persona (UI:N). Se compromete la confidencialidad limitada de metadatos (C:L); no se demostró modificación ni indisponibilidad (I:N/A:N), ni afectación de otra autoridad de seguridad (S:U). AV:N no significa exposición a Internet: el acceso observado fue al laboratorio.

### RT-H02 — Replay duplica auditoría y notificaciones

- **Debilidad:** ausencia de deduplicación del evento lógico y sus efectos.
- **Reproducción:** generar un evento ficticio nuevo y reenviar dos veces exactamente su cuerpo firmado antes del vencimiento. Procedimiento en [P03](RT-P03-replay.md).
- **Resultado e impacto:** original 51 y reenvíos 52/53 con los mismos campos firmados y firma válida; tres registros y tres correos para un evento lógico.
- **Evidencia y límite:** [P03 y su índice](RT-P03-replay.md), con PostgreSQL, JSONL y Mailpit en `evidencias/RT-P03/`. Se requiere acceso al endpoint y un mensaje firmado vigente; no se demostró cómo lo obtendría un tercero. El cuerpo archivado se reconstruyó desde PostgreSQL y no es una captura HAR. Para repetir se necesita un evento nuevo.
- **Recomendación:** ID de evento cubierto por firma, unicidad persistente y procesamiento idempotente que evite repetir efectos ante reintentos legítimos.
- **Clasificación:** CVSS v3.1 **3.7 — Baja**, valoración condicionada a los supuestos siguientes. Vector: `CVSS:3.1/AV:N/AC:H/PR:N/UI:N/S:U/C:N/I:L/A:N`. CWE pendiente; MITRE T1565.
- **Justificación:** reenvío HTTP por red (AV:N). Se asume ausencia de una sesión o rol adicional exigido por el receptor (PR:N) y que no se necesita interacción de otra persona durante el replay (UI:N). AC:H representa la condición de conseguir un mensaje legítimo aún vigente; su obtención por un tercero no se demostró. Se afecta de forma limitada la integridad de la auditoría (I:L), sin lectura adicional de secretos ni caída demostrada (C:N/A:N). La repetición de correo no acredita cambio de autoridad de seguridad (S:U).
- **Sensibilidad del puntaje:** si cualquier emisor puede obtener de forma ordinaria mensajes válidos para este abuso, AC:L daría **5.3 — Media** con `CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:N/I:L/A:N`. Si se exige un rol, revisar también PR. La valoración 3.7 no acredita que se haya demostrado la dificultad de capturar el mensaje.

## 3. Priorización de remediación

### Incorporación del 25/10/2026 — RT-H04

**Token del panel sin vencimiento temporal.** Identificado en código, validación HTTP pendiente. `_emitir_token` guarda el hash; `usuario_actual` comprueba coincidencia sin antigüedad y `User` no guarda expiración. Un nuevo login del mismo usuario sí reemplaza el token previo. Si se obtiene un token válido, la ausencia de caducidad amplía su ventana de reutilización; no se demostró robo ni privilegios adicionales.

[Ficha técnica y límites](RT-H04-sesion-sin-vencimiento.md). No se ejecutó una prueba temporal ni se iniciaron servicios. Severidad/CVSS, clasificación, responsable, plazo, incidente y riesgo pendientes. No se suma a los hallazgos reproducidos en ejecución ni a los conteos por severidad existentes.

### Incorporación del 24/10/2026 — RT-H03

**Reconfiguración incompleta desactiva TOTP de la bóveda.** Abierta y reproducida manualmente con una bóveda ficticia. CVSS/vector, CWE, MITRE, responsable, incidente y riesgo pendientes. Sin remediación ni retest.

Generar otro TOTP reemplaza el secreto y persiste `totp_activo=0` antes de confirmarlo. Abandonar permite abrir después sin segundo factor. Requiere una bóveda previamente abierta y sigue requiriendo la contraseña para abrirla después. Conservar el factor anterior hasta confirmar el candidato y exigir reautenticación para modificar factores.

Procedimiento y límites: [RT-P05](RT-P05-reconfiguracion-totp.md). Capturas y hashes: `evidencias/RT-P05/`. La ausencia de código en la apertura final consta en el reporte del usuario; la imagen final acredita apertura. Las prioridades CVSS siguientes de H01/H02 no incluyen aún H03.

Valoración base realizada el 23/10/2026 con la [especificación CVSS v3.1 de FIRST](https://www.first.org/cvss/v3.1/specification-document). CVSS es propiedad de FIRST y se utiliza conforme a sus condiciones. Los puntajes expresan severidad técnica; no sustituyen el análisis de riesgo del entorno.

Prioridad propuesta: **H01 primero (Media, 5.3)** y **H02 después (Baja, 3.7 condicionada)**. Revisar H02 cuando se aclare cómo un tercero obtiene el mensaje firmado. La plantilla propone SLA de 30 días para medias y 90 días para bajas; confirmar su adopción con el Blue Team antes de asignar vencimientos. No hay fechas de corrección acordadas.

## 4. Corrección y verificación

Ventana de cambios, responsables y fechas: pendientes de acordar con el Blue Team. Aplicar correcciones primero en el laboratorio, conservar la versión evaluada y registrar versión corregida, prueba repetida, evidencia y resultado. Cerrar solo con verificación; si se acepta un riesgo, registrar quién lo acepta y su justificación.

## 5. Tratamiento de falsos positivos

No hay descartes aprobados documentados. Cualquier descarte debe registrar hallazgo, motivo, evidencia, responsable que lo aprueba y fecha.

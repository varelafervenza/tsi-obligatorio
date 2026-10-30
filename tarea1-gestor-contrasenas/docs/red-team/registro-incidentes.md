# Registro de incidentes de prueba — Red Team

**Sistema:** Tarea 1 — Gestor de contraseñas. **Estado:** preliminar. Basado en [la plantilla 04](../../../plantilla/isaca/04-gestion-incidentes.md). Casos iniciales del 21/10/2026 y seguimiento incorporado hasta el 27/10/2026.

| Campo | Valor |
|---|---|
| Código / versión | SI-INC-04-RT / 0.1 |
| Responsable | Pendiente de identificar. |
| Informe | [Informe Red Team](informe-red-team.md) |
| Referencias de la plantilla | MCU Detectar/Responder/Recuperar; ISO 27001 A.5.24–A.5.28; COBIT DSS02/DSS04/APO12. |

## 1. Criterio de registro

Son ejercicios autorizados con datos ficticios, no incidentes de producción. Se consolida cada hallazgo vigente como registro de prueba según la plantilla, conservando sus límites. P01 se incluye como intento simulado detectado. P02 queda documentado como evento de alteración detectada en su [prueba](RT-P02-integridad-eventos.md).

Clasificación inicial propuesta para estos ejercicios: S0, impacto catastrófico; S1, compromiso grave de componente central; S2, intento de intrusión detectado o afectación limitada de confidencialidad/integridad; S3, sospecha menor sin afectación confirmada. Se adapta S2 de la plantilla para incluir los efectos limitados observados. Estas categorías no equivalen al nivel de alerta Wazuh ni al puntaje CVSS; no constituyen una clasificación aprobada por el Blue Team.

## 2. Registro

| ID | Caso | Fecha del evento | Vector / activo | Severidad inicial | Estado del registro | Responsable |
|---|---|---|---|---|---|---|
| INC-2026-002 | P04 / H01: lectura sin autenticación | 21/10/2026 local; hora UTC no registrada. | HTTP GET / API de auditoría y KPI. | S2 — propuesta | Analizando; sin cierre verificado. | Pendiente |
| INC-2026-003 | P03 / H02: replay | 22/10/2026 UTC; reenvíos 01:17:51.453214 y 01:19:22.531800. | HTTP POST / recepción de eventos, auditoría y notificaciones. | S2 — propuesta | Analizando; sin cierre verificado. | Pendiente |
| INC-2026-004 | P01: intentos de maestra | 21/10/2026 11:52:12.481 UTC, alerta de correlación. | Interfaz local / acceso a bóveda y detección Wazuh. | S2 — propuesta | Analizando; intento simulado detectado. | Pendiente |
| INC-2026-005 | P05 / H03: TOTP desactivado durante reconfiguración incompleta | 24/10/2026 local; hora no registrada. | App de escritorio / bóveda ficticia vacía. | Pendiente | Reproducción manual documentada; sin respuesta ni cierre verificados. | Pendiente |
| INC-2026-006 | H04: token del panel sin vencimiento temporal | 25/10/2026 local; hora no registrada. | Revisión de código de autenticación; sin explotación. | Pendiente | Hallazgo estático; prueba temporal pendiente. | Pendiente |

Las fechas de P03 equivalen a la noche del 21/10 en Montevideo. Las horas provienen de los registros conservados; no son horas de notificación o de respuesta humana. “Analizando” identifica el seguimiento documental pendiente, no una actuación acreditada del Blue Team.

## 3. Hechos, impacto y respuesta observada

### INC-2026-002 — Lectura de auditoría y KPI sin sesión

- **Hecho e impacto:** los GET iniciales de auditoría y KPI devolvieron 200 con metadatos sin autenticación; usuarios devolvió 401. No se observaron contraseñas ni exposición a Internet.
- **Detección:** comprobación HTTP instrumentada del evaluador. No hay evidencia de detección automática o notificación al Blue Team.
- **Respuesta:** contención, erradicación y recuperación no documentadas. La repetición posterior terminó por timeout y no acredita corrección.
- **Evidencia y seguimiento:** [P04](RT-P04-acceso-sin-autenticacion.md), ficha RT-H01 y [resultado inicial](evidencias/revision-inicial/resultado-reproduccion-aislada.json), sección GET. Verificar autenticación y permisos tras corregir; conservar respuestas de controles sin sesión y con sesión autorizada.

### INC-2026-003 — Duplicación de auditoría y notificaciones

- **Hecho e impacto:** dos reenvíos del mismo evento firmado produjeron IDs 52 y 53 además del original 51, tres líneas de auditoría y tres correos para una operación lógica.
- **Detección:** comparación posterior del evaluador en PostgreSQL, JSONL y Mailpit. Los correos repetidos no acreditan detección del replay por la defensa.
- **Respuesta:** contención, erradicación y recuperación no documentadas. No se acreditó indisponibilidad del servicio ni limpieza posterior de duplicados.
- **Evidencia y seguimiento:** [P03](RT-P03-replay.md) y ficha RT-H02 de [Gestión de vulnerabilidades](gestion-vulnerabilidades.md). Verificar deduplicación e idempotencia con un evento nuevo firmado y conservar el resultado del retest.

### INC-2026-004 — Intentos simulados de contraseña maestra

- **Hecho e impacto:** cinco intentos incorrectos dentro de dos minutos generaron eventos firmados y alerta de correlación `100101`, nivel 10. No se demostró acceso indebido.
- **Detección:** Wazuh correlacionó los intentos; evidencia de alerta real, distinta de simulaciones con `wazuh-logtest`.
- **Respuesta:** no se acreditó bloqueo automático ni actuación humana. Apertura final y demora de interfaz pendientes; no hay recuperación o cierre formal documentados.
- **Evidencia y seguimiento:** [P01](RT-P01-intentos-maestra.md) y `evidencias/RT-P01/`. Confirmar recepción de la alerta y respuesta del Blue Team antes de cerrar el ejercicio.

### INC-2026-005 — Reconfiguración incompleta de TOTP

- **Hecho e impacto:** al generar un segundo TOTP y cerrar sin confirmarlo, el usuario reportó que la bóveda ficticia volvió a abrir con la contraseña sin pedir el código anterior. Requiere haber abierto la bóveda antes de la reconfiguración.
- **Detección y respuesta:** se observó manualmente; no hay evidencia de alerta, actuación del Blue Team, corrección ni retest.
- **Evidencia y seguimiento:** [P05](RT-P05-reconfiguracion-totp.md) y `evidencias/RT-P05/`. La imagen final muestra apertura, pero no el campo TOTP anterior al clic; ese paso se apoya en el relato del usuario. Confirmar severidad y recuperar el factor de la bóveda de prueba si corresponde.

### INC-2026-006 — Token sin vencimiento temporal

- **Hecho e impacto:** la revisión de código encontró que se compara el hash del token sin verificar edad o inactividad. La reutilización presupone obtener un token válido; no se demostró robo ni acceso indebido.
- **Detección y respuesta:** hallazgo de análisis estático; no hay un incidente de explotación observado, alerta ni respuesta del Blue Team acreditada.
- **Evidencia y seguimiento:** [ficha H04](RT-H04-sesion-sin-vencimiento.md). Acordar duración permitida de sesión y ejecutar una comprobación temporal acotada antes de valorar el impacto.

## 4. Notificación y escalamiento

Contacto del Blue Team/docente, canal, fecha de comunicación, acuse de recibo y plazo de respuesta: pendientes por registro. Ante un efecto fuera del alcance, detener la prueba y comunicarlo al contacto acordado.

No hay notificaciones externas documentadas. Son casos de laboratorio con datos ficticios; cualquier ejercicio de notificación a BCU/URCDP debe identificarse como simulación y acordarse con la cátedra. Los requisitos y plazos aplicables a incidentes reales deben validarse antes de usarlos.

## 5. Lecciones y cierre

La detección funcionó en P01 y la integridad de la firma en P02. La firma no evita reenvíos ni protege las consultas. Acciones técnicas: fichas de [Gestión de vulnerabilidades](gestion-vulnerabilidades.md).

Para cerrar cada registro: completar responsable y validar la severidad propuesta, comunicación y respuesta del Blue Team, acciones realizadas con fecha y evidencia, verificación final y lección aprendida. Si no hubo compromiso que requiera restauración, registrar esa justificación; no inventar acciones de recuperación ni tiempos de respuesta.

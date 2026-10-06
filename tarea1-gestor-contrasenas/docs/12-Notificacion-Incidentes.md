# Notificación de Incidentes (BCU / URCDP) — Incidente 1 (simulado)

---

## Encabezado de mapeo normativo

| Marco | Ítem | Detalle / aporte |
|---|---|---|
| **MCU 5.0 (función)** | **Responder** | Notificación y comunicación de incidentes (RS-05). |
| **MCU 5.0 (categorías)** | RS-04, RS-05 | Análisis y reporte de incidentes. |
| **COBIT 2019** | DSS02, MEA01 | Escalamiento y aseguramiento. |
| **ISO/IEC 27001:2022** | A.5.26, A.5.27 | Respuesta a incidentes, aprendizaje, evidencia. |
| **BCU — Guía de Seguridad de la Información** | Requisito de notificación de incidentes | Evaluación de si el caso es relevante para el BCU. |
| **URCDP — Ley 18.331 + Dec. 414/009** | Art. 20 | Evaluación de si hubo violación de datos personales. |

---

## Control del documento

| Campo | Valor |
|---|---|
| Código | SI-NOT-12 |
| Versión | 0.1 (borrador) |
| Responsable | Andrés Varela (redacción), con revisión de Pablo Morales y Horacio Duarte |
| Fecha | 06/10/2026 |
| Tipo de caso | **Simulación** (incidente de laboratorio con datos de prueba). No se envió a ningún organismo. |

---

## 1. Instrucciones de llenado

- Un formulario por incidente.
- Fechas y horas en **UTC**, con la hora local de Uruguay (UTC-3) entre paréntesis.
- Evidencia técnica: `docs/evidencias/06-restauracion-bd-con-datos.txt` y `docs/evidencias/07-simulacion-casos-uso.txt`, además del log `infra/logs/audit-events.jsonl` (eventos 6 a 10).

---

## 2. Formulario de notificación al BCU (incidentes relevantes)

| Campo | Valor |
|---|---|
| Fecha y hora del incidente (UTC) | 05/10/2026 20:29:48 (05/10/2026 17:29:48 hora Uruguay). Primer intento fallido de la serie. |
| Fecha y hora de detección (UTC) | 05/10/2026 20:29:51.248 (05/10/2026 17:29:51 hora Uruguay). Creación de la alerta 1 `fuerza_bruta_maestra` por el quinto intento. |
| Institución/financiera | No aplica. Simulación en el laboratorio de la Tarea 1; no hay entidad real afectada. |
| Tipo de incidente | Otro: intento de fuerza bruta contra la contraseña maestra (simulado). |
| Sistemas afectados | A02 bóveda cifrada (objetivo simulado de los intentos), A04 servidor control-central (donde se detectó), A05 PostgreSQL (donde se guardó el evento). |
| Datos afectados | No. Ningún intento tuvo éxito y no hubo acceso a la bóveda ni a la base de usuarios. |
| Impacto estimado | Ninguno en la simulación. Cinco eventos fallidos registrados. Sin indisponibilidad. |
| Acciones de contención | Ver sección 6 (registro del incidente y línea de tiempo). En la simulación no hubo que bloquear nada. Un caso real requeriría bloquear el origen; esa respuesta automática no está implementada (ver limitaciones). |
| Estado actual | Contenido. Incidente 1 en `resuelto` desde el 06/10/2026 18:07:52 UTC. |
| Contacto de reporte | Andrés Varela (Blue Team). Teléfono y correo de contacto oficial: **pendiente**, no están definidos para la entrega. |
| Plazo cumplido | No aplica. No hay plazo de notificación porque el caso no se considera relevante para el BCU (ver sección 5). |

## 3. Formulario de notificación a la URCDP (violación de datos personales)

| Campo | Valor |
|---|---|
| Titular del tratamiento | No aplica en la simulación. En producción sería la institución que contrata el servicio. |
| Encargado del tratamiento (si aplica) | No aplica en la simulación. |
| Fecha del incidente | 05/10/2026 |
| Descripción clara y completa del incidente | Serie de cinco intentos fallidos de desbloqueo de la bóveda con la misma identidad de agente (`agente-dev-01`), dentro de 3 segundos. El control central lo detectó con la regla de fuerza bruta (umbral 5 en 120 s). |
| Categoría de datos personales afectados | Ninguna. Los eventos contienen tipo, sistema, agente, fecha y firma. No contienen contraseñas ni contenido de la bóveda (ver `infra/logs/audit-events.jsonl`). |
| Número de personas afectadas | 0. |
| Riesgos para los afectados | Ninguno confirmado. Un ataque real podría poner en riesgo la bóveda de un usuario si la contraseña maestra fuera débil; el umbral y la alerta reducen esa ventana. |
| Medidas adoptadas para mitigar | Alerta automática, registro en el incidente 1, y verificación de que la base se puede restaurar con el incidente cargado. |
| Medidas de notificación a los titulares (se realizó/plan) | No corresponde. No hay violación confirmada. |
| Datos del responsable del informe | Andrés Varela (Blue Team). Contacto oficial: **pendiente**. |

### Criterios de decisión de notificación a la URCDP

| Criterio | Respuesta en el incidente 1 |
|---|---|
| ¿Existe compromiso de datos personales? | **No.** Ningún intento tuvo éxito y no se accedió a datos. |
| ¿Hay riesgo real para los derechos de los titulares? | **No confirmado.** No hubo acceso a la bóveda ni a la base de usuarios. |
| Resultado | **No se notifica a la URCDP.** Se conserva esta evaluación como prueba. |

Si el análisis hubiera mostrado un acceso exitoso a la bóveda (por ejemplo, el quinto intento hubiera tenido éxito), la respuesta cambiaría: notificar a la URCDP y a los titulares dentro del plazo de la Ley 18.331. Ese plazo está como `[plazo legal]` en `04-Gestion-Incidentes.md` y no se verificó contra el texto de la ley para este documento.

## 4. Registro de notificaciones enviadas

| ID incidente | Autoridad (BCU/URCDP) | Fecha de envío | Canal | Estado respuesta | Responsable |
|---|---|---|---|---|---|
| 1 (simulado) | BCU | No enviada | No aplica | No aplica | Andrés Varela |
| 1 (simulado) | URCDP | No enviada | No aplica | No aplica | Andrés Varela |

## 5. Decisión de notificación

| Autoridad | Decisión | Fundamento |
|---|---|---|
| BCU | No notificar (caso no relevante) | Simulación sin impacto en una entidad real ni en sus servicios. |
| URCDP | No notificar | No hay compromiso de datos personales (sección 3). |

Esta decisión la propone el Blue Team. La aprueba el RSI, que todavía no firmó (ver `11-SoA-Plan-Tratamiento.md`, control 5.26).

## 6. Resumen del incidente (para el registro de la 04)

| Campo | Valor |
|---|---|
| ID de incidente | Incidente 1 en `control-central` (código de documento propuesto: `INC-2026-001`) |
| Alerta de origen | 1, `fuerza_bruta_maestra`, evento 10 |
| Severidad | S2 (intento de intrusión sin compromiso confirmado, según `04-Gestion-Incidentes.md`) |
| Vector de ingreso | Red del laboratorio (`ip_origen` 172.18.0.1, red Docker) |
| Estado | `resuelto` el 06/10/2026 18:07:52 UTC |
| Tiempo de detección | Desde el primer intento hasta la alerta: 3 s (20:29:48 → 20:29:51) |
| Tiempo desde la alerta hasta la resolución (MTTR del panel) | 21 h 38 min (05/10 20:29:51 → 06/10 18:07:52 UTC). Incluye la espera entre la alerta y la creación manual del caso (ver `28-Informe-Blue-Team.md`). |

Alertas relacionadas del mismo día, que no forman parte de este incidente: 2 (`borrado_masivo`, CU-02), 3 y 4 (`cambio_maestra`, CU-03). Se pueden registrar como incidentes aparte si el RSI lo decide.

## 7. Limitaciones de este caso

- Es una simulación. Los eventos los generó `scripts/simular_casos_uso.py` y no hubo un atacante real.
- No hay bloqueo automático del origen. Un incidente real requeriría esa respuesta, y hoy solo hay alerta y registro manual.
- Los contactos oficiales (teléfono y correo de reporte) no están definidos para la entrega.
- La redacción se basó en la plantilla del curso y en los criterios de `04-Gestion-Incidentes.md`. Los plazos legales no se verificaron contra el texto de la normativa.

---

## Guía de llenado (Blue Team)

| Fase | Cómo completar | Con qué evidencia |
|---|---|---|
| Blue Team - Demo | Redactar un borrador de notificación para un incidente simulado | Este documento, con el incidente 1 |
| Red Team | Cada hallazgo grave debe evaluar notificación según criterios | Informe Red Team (pendiente) |
| Cierre | Verificar el circuito (tiempos, contactos, plantillas cargadas) | Contactos oficiales pendientes de definir |

## Check de aceptación

- [x] Plantilla de notificación BCU completa (sección 2, con los campos que no aplican justificados).
- [x] Plantilla de notificación URCDP completa (sección 3).
- [x] Criterios de decisión definidos y aplicados al incidente 1 (secciones 3 y 5).
- [x] Al menos un incidente simulado documentado de punta a punta (incidente 1, desde la alerta hasta la resolución).
- [ ] Contactos oficiales de reporte definidos.
- [ ] Aprobación del RSI.

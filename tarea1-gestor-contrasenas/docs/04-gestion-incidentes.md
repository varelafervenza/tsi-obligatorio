# Plantilla ISACA/COBIT 2019 — Gestión de Incidentes de Seguridad

---

## Encabezado de mapeo normativo

| Marco | Ítem | Detalle / aporte |
|---|---|---|
| **MCU 5.0 (función)** | **Detectar** + **Responder** + **Recuperar** | Detección de eventos (DE-01...), respuesta (RS), recuperación (RC). |
| **MCU 5.0 (categorías)** | DE-01 a DE-03, RS-01 a RS-05, RC-01 a RC-03 | Registro, análisis, respuesta y recuperación de incidentes. |
| **COBIT 2019** | DSS02 (Gestión de peticiones e incidentes), DSS04, APO12 | Proceso de operación de incidentes. |
| **ISO/IEC 27001:2022** | A.5.24, A.5.25, A.5.26, A.5.27, A.5.28 | Planificación y respuesta a incidentes. |
| **ISO/IEC 27035** | Todo el estándar | Marco internacional de gestión de incidentes. |
| **BCU — Guía de Seguridad de la Información** | Requisito de incidentes y notificación | Notificación de incidentes relevantes al BCU y definición de escalamiento. |
| **URCDP — Ley 18.331** | Art. 20 (notificación de violaciones) | Notificación de incidentes de datos personales a la URCDP. |
| **MCU 5.0 + BCU Com. 2026/098** | Respuesta a incidentes reportados trimestralmente | Evidencia del nivel de madurez de respuesta. |

---

## Control del documento

| Campo | Valor |
|---|---|
| Código | SI-INC-04 |
| Versión | 0.2 (borrador) |
| Responsable | Blue Team (Horacio Duarte, Pablo Morales, Andrés Varela) |
| Fecha | 27/09/2026 |

### Historial de versiones

| Versión | Fecha | Autor | Cambios |
|---|---|---|---|
| 0.1 | 22/09/2026 | Blue Team | Plantilla inicial de gestión de incidentes. |

---

## 1. Definiciones (marco)

- **Evento** de seguridad: ocurrencia observable. No todo evento es incidente.
- **Incidente** de seguridad: evento que compromete la C-I-A o viola la política.
- **Banda de severidad**: S0 crítica, S1 alta, S2 media y S3 baja, según el impacto y el alcance definidos abajo.

| Severidad | Criterio | Ejemplo |
|---|---|---|
| S0 | Impacto catastrófico o fuga masiva de secretos | Exfiltración confirmada de una bóveda o de credenciales del panel |
| S1 | Compromiso de un componente central o afectación de múltiples agentes | Control central comprometido o falsificación confirmada de eventos |
| S2 | Intento de intrusión o alerta relevante sin compromiso confirmado | Fuerza bruta, firma inválida repetida o modificación sospechosa detectada |
| S3 | Anomalía aislada sin impacto confirmado | Log anómalo o fallo operativo sin evidencia de ataque |

## 2. Clasificación y registro de incidentes

| Campo | Descripción |
|---|---|
| ID del incidente | `INC-2026-###` |
| Fecha y hora (UTC) | `[fecha/hora]` |
| Descripción / síntomas | `[descripción]` |
| Severidad inicial | `[S0-S3]` |
| Vector de ingreso | `[red/Web/mail/local/físico]` |
| Activos comprometidos | `[ID de activo]` |
| Evidencia preservada | `[logs, memoria, tráfico]` |
| Estado | `abierto`, `en análisis` o `resuelto` (RF-13) |
| Alerta de origen | ID de alerta Wazuh o referencia al evento que originó el caso |
| Responsable | RSI o integrante asignado por el RSI |
| Acciones y evidencias | Medidas de contención, erradicación, recuperación y enlaces a evidencias |
| Fecha de resolución | Obligatoria al pasar a `resuelto` |

### Registro de incidentes

| ID | Fecha | Descripción | Severidad | Vector | Activo | Estado | Responsable |
|---|---|---|---|---|---|---|---|
| | | | | | | | |

## 3. Procedimiento de respuesta (línea de tiempo)

1. **Detección**: recibir una alerta de Wazuh, un reporte del agente, un correo o un hallazgo reproducible.
2. **Registro**: crear el `Incident` en `control-central`, conservar la alerta/evento de origen, asignar ID `INC-2026-###`, severidad y estado `abierto`.
3. **Análisis y contención**: el RSI asigna responsable y cambia el estado a `en análisis`; preservar logs, aislar el activo afectado y evitar la propagación o exposición de secretos.
4. **Erradicación**: eliminar la causa mediante reversión, parche, rotación de claves o revocación del agente, dejando evidencia de la acción.
5. **Recuperación**: restaurar el servicio o activo desde una copia probada, verificar su funcionamiento y documentar el resultado.
6. **Cierre**: el RSI valida la solución, registra responsable, fecha, evidencias y lecciones aprendidas, y cambia el estado a `resuelto`.

## 4. Notificación y escalamiento

| Escenario | Notificar a | Plazo | Utilizar plantilla |
|---|---|---|---|
| Incidente de datos personales | URCDP (según Art. 20) | `[plazo legal]` | `docs/12-Notificacion-Incidentes.md` |
| Incidente relevante (BCU) | BCU según GSI | `[plazo]` | `docs/12-Notificacion-Incidentes.md` |
| Incidente operativo interno | Dirección / RSI | `[plazo]` | Registro interno |

## 5. Plantilla de lecciones aprendidas

| Campo | Valor |
|---|---|
| Qué ocurrió | `[descripción]` |
| Por qué ocurrió | `[causa raíz]` |
| Qué funcionó | `[positivo]` |
| Qué falló | `[negativo]` |
| Acciones de mejora | `[lista]` |
| Responsable y fecha | `[nom/fecha]` |

---

## Guía de llenado (Blue Team)

| Fase | Cómo completar | Con qué evidencia |
|---|---|---|
| Blue Team - Implementación | Redactar procedimiento y probarlo con incidentes simulados | KPIs: MTTR, N° de incidentes |
| Blue Team - Panel/dashboard | Registrar cada alerta materializada | Capturas de alertas → registro |
| Red Team → entregables | Cada hallazgo del Red Team DEBE consolidarse como registro de incidente | Informe técnico del Red Team → INC-… |
| Cierre | Lecciones aprendidas y acciones de mejora | Plan de mejora continua |

## Check de aceptación

- [ ] Definiciones de severidad y clasificación.
- [ ] Procedimiento de respuesta completo (D→C→E→R→L).
- [ ] Registro de incidentes con los simulados y los hallazgos del Red Team.
- [ ] Procedimiento de notificación a BCU/URCDP.
# Plantilla ISACA/COBIT 2019 — Notificación de Incidentes (BCU / URCDP)

---

## Encabezado de mapeo normativo

| Marco | Ítem | Detalle / aporte |
|---|---|---|
| **MCU 5.0 (función)** | **Responder** | Notificación y comunicación de incidentes (RS-05). |
| **MCU 5.0 (categorías)** | RS-04, RS-05 | Análisis y reporte de incidentes. |
| **COBIT 2019** | DSS02, MEA01 | Escalamiento y aseguramiento. |
| **ISO/IEC 27001:2022** | A.5.26, A.5.27 | Respuesta a incidentes, aprendizaje, evidencia. |
| **BCU — Guía de Seguridad de la Información** | Requisito de notificación de incidentes | **Notificar incidentes relevantes al BCU** en plazos definidos. |
| **URCDP — Ley 18.331 + Dec. 414/009** | Art. 20 | **Notificación de violaciones de datos personales a la URCDP** y a los titulares. |

---

## Control del documento

| Campo | Valor |
|---|---|
| Código | SI-NOT-12 |
| Versión | 1.0 |
| Responsable | `[Nombre]` |
| Fecha | `[DD/MM/AAAA]` |

---

## 1. Instrucciones de llenado

- Use **un formulario por incidente**.
- Siempre registre fecha/hora **UTC** y hora local de Uruguay (`UTC-3`).
- Adjunte la **evidencia técnica** (logs, capturas, tickets del SIEM) en el repositorio `docs/evidencias/INC-####`.

## 2. Formulario de notificación al BCU (incidentes relevantes)

| Campo | Valor |
|---|---|
| Fecha y hora del incidente (UTC) | |
| Fecha y hora de detección (UTC) | |
| Institución/financiera | |
| Tipo de incidente | `[Denegación de servicio / Fuga de datos / Ransomware / Intrusión / Fraude / Otro]` |
| Sistemas afectados | `[IDs de activos]` |
| Datos afectados | `[sí/no, cuáles]` |
| Impacto estimado | `[ej.: indisponibilidad X h, nº de registros]` |
| Acciones de contención | |
| Estado actual | `[Contenido / En análisis / En curso]` |
| Contacto de reporte | `[nombre, teléfono, correo]` |
| Plazo cumplido | `[según GSI BCU]` |

## 3. Formulario de notificación a la URCDP (violación de datos personales)

| Campo | Valor |
|---|---|
| Titular del tratamiento | |
| Encargado del tratamiento (si aplica) | |
| Fecha del incidente | |
| Descripción clara y completa del incidente | |
| Categoría de datos personales afectados | `[identidad, económica, biométrica, etc.]` |
| Número de personas afectadas | |
| Riesgos para los afectados | `[robo de identidad, fraude]` |
| Medidas adoptadas para mitigar | |
| Medidas de notificación a los titulares (se realizó/plan) | |
| Datos del responsable del informe | |

### Criterios de decisión de notificación a la URCDP

- ¿Existe compromiso de **datos personales**? → si la respuesta es afirmativa aplicar procedimiento.
- ¿Hay riesgo real para los derechos de los titulares? → notificar.
- Conservar la prueba de la evaluación de riesgo de la notificación.

## 4. Registro de notificaciones enviadas

| ID incidente | Autoridad (BCU/URCDP) | Fecha de envío | Canal | Estado respuesta | Responsable |
|---|---|---|---|---|---|
| | | | | | |

---

## Guía de llenado (Blue Team)

| Fase | Cómo completar | Con qué evidencia |
|---|---|---|
| Blue Team - Demo | Redactar un borrador de notificación para un incidente simulado | Registro de simulación |
| Red Team | Cada hallazgo grave debe evaluar notificación según criterios | Informe Red Team |
| Cierre | Verificación del circuito (tiempos, contactos, plantillas cargadas) | Constancia de prueba |

## Check de aceptación

- [ ] Plantilla de notificación BCU completa.
- [ ] Plantilla de notificación URCDP completa.
- [ ] Criterios de decisión definidos.
- [ ] Al menos un incidente simulado documentado de punta a punta.
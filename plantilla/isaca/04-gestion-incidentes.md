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
| Versión | 1.0 |
| Responsable | `[Nombre]` |
| Fecha | `[DD/MM/AAAA]` |

### Historial de versiones

| Versión | Fecha | Autor | Cambios |
|---|---|---|---|
| 1.0 | | | |

---

## 1. Definiciones (marco)

- **Evento** de seguridad: ocurrencia observable. No todo evento es incidente.
- **Incidente** de seguridad: evento que compromete la C-I-A o viola la política.
- **Banda de severidad**: `[S0-CRÍTICA / S1-ALTA / S2-MEDIA / S3-BAJA]` (definir criterios).

| Severidad | Criterio | Ejemplo |
|---|---|---|
| S0 | Impacto catastrófico / fuga masiva de secretos | Dump de la DB de credenciales |
| S1 | Compromiso de un componente central | Controlador comprometido |
| S2 | Intento de intrusión detectado por SIEM/HIDS | Brute force registrado |
| S3 | Sospecha menor | Log anómalo |

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
| Estado | `[Nuevo / Analizando / Contenido / Cerrado]` |

### Registro de incidentes

| ID | Fecha | Descripción | Severidad | Vector | Activo | Estado | Responsable |
|---|---|---|---|---|---|---|---|
| | | | | | | | |

## 3. Procedimiento de respuesta (línea de tiempo)

1. **Detección**: sensor/SIEM/HIDS/alerta/correo.
2. **Registro**: cargar en el registro y clasificar severidad.
3. **Contención**: aislar activo/no filtrar; `[acción definida]`.
4. **Erradicación**: eliminar la causa (reversión, parche).
5. **Recuperación**: restaurar con evidencia de backup probado.
6. **Lecciones aprendidas**: informe post-incidente.

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
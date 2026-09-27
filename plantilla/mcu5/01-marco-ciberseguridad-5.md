# MCU 5.0 — Marco de Ciberseguridad del Uruguay (AGESIC)

---

## 1. ¿Qué es el MCU 5.0?

El **Marco de Ciberseguridad (MCU) 5.0** es la herramienta de referencia de AGESIC (Agencia de Gobierno Electrónico y Sociedad de la Información y del Conocimiento) para la seguridad de la información y ciberseguridad de las organizaciones uruguayas.

**Objetivo**: reducir el riesgo vinculado a las amenazas que pueden comprometer la seguridad de la información, ofreciendo un **abordaje integral** del ciclo de vida de la ciberseguridad.

## 2. Estructura

### 2.1 Funciones del ciclo de vida (ejes del Marco)

| Función | Nombre | Qué aborda |
|---|---|---|
| **GV** | **Gobernar** | Gobierno de la gestión del riesgo de ciberseguridad, roles, estrategia, políticas, análisis de la organización. |
| **ID** | **Identificar** | Comprender el contexto: activos, riesgos, vulnerabilidades y prioridades de la organización. |
| **PR** | **Proteger** | Salvaguardas para la gestión de los riesgos: identidad, accesos, protección de datos, mantenimiento, etc. |
| **DE** | **Detectar** | Actividades para detectar tempranamente eventos y anomalías de seguridad. |
| **RS** | **Responder** | Actividades de respuesta a incidentes y mitigación de impacto. |
| **RC** | **Recuperar** | Actividades de recuperación y resiliencia (continuidad, mejora). |

### 2.2 Categorías y subcategorías

Cada función agrupa **categorías** y **subcategorías** (resultados deseados). La convención es `FUNCIÓN-NroCategoría.NroSubcategoría` (por ejemplo, GV-01.01, ID-03.04, PR-01.05, DE-01.02, RS-02.03, RC-01.01).

> En las plantillas del curso se referencia por función + categoría (de manera genérica) para asociar cada entregable al eje del Marco.

## 3. Perfiles comunitarios

El MCU 5.0 define tres **perfiles comunitarios** que sirven de punto de partida según el tipo de organización:

| Perfil | Público objetivo | Aproximación |
|---|---|---|
| **Básico** | Organizaciones con recursos limitados | Conjunto mínimo de controles de ciberseguridad esenciales. |
| **Estándar** | Organizaciones medianas | Cubre los ejes completos del marco con esfuerzo razonable. |
| **Avanzado** | Organizaciones con mayor exposición/recursos | Controles profundos, automatización y mejora continua. |

Cada perfil define una **línea base de implementación** (metas a 3 años) y se publican **planillas** oficiales de cumplimiento (formato Excel/XLSX).

## 4. Uso del Marco por las organizaciones

1. **Comprender el contexto** de la organización: identificar activos que soportan procesos críticos y los riesgos asociados.
2. **Dirigir recursos e inversiones** hacia medidas de protección de procesos y activos.
3. **Detectar tempranamente** eventos de seguridad.
4. **Responder** reduciendo el impacto de incidentes.
5. **Recuperarse** con resiliencia y oportunidad.

## 5. Relación con el BCU

El BCU exige a las entidades supervisadas reportar su **nivel de madurez y cumplimiento del MCU 5.0** con frecuencia **trimestral** (Comunicación 2026/098), mediante el Portal IDI con el **Tipo de dato 957 – Cumplimiento MCU 5.0 – Nivel de Madurez**, plazo de **7 días hábiles** del mes siguiente al cierre del trimestre (ver `bcu/01-requerimientos-minimos-bcu.md`).

## 6. Niveles de madurez (orientativo para el reporte)

| Nivel | Descripción |
|---|---|
| 0 | Inicial / inexistente |
| 1 | Repetible / ad hoc |
| 2 | Definido |
| 3 | Gestionado |
| 4 | Optimizado / medido |

## 7. Mapeo con marcos internacionales (orientación)

| MCU 5.0 (función) | NIST CSF 2.0 (aprox.) | ISO 27001 (aprox.) | COBIT 2019 (aprox.) |
|---|---|---|---|
| Gobernar (GV) | Governance (GV) | Cláusula 5 (Liderazgo) | EDM, APO01, APO13 |
| Identificar (ID) | Identify | Cláusula 6.1; A.5.9; A.8.8 | APO03, APO12 |
| Proteger (PR) | Protect | A.5.15-A.5.19; A.8.x | DSS05, APO13 |
| Detectar (DE) | Detect | A.8.15, A.8.16 | DSS04, DSS05 |
| Responder (RS) | Respond | A.5.24-A.5.28 | DSS02 |
| Recuperar (RC) | Recover | A.5.29, A.8.13 | DSS04 |

## 8. Enlaces oficiales

- Marco MCU 5.0 (AGESIC): https://www.gub.uy/agencia-gobierno-electronico-sociedad-informacion-conocimiento/comunicacion/publicaciones/marco-ciberseguridad-50
- Guía de Implementación: https://www.gub.uy/agencia-gobierno-electronico-sociedad-informacion-conocimiento/comunicacion/publicaciones/guia-implementacion-del-marco-ciberseguridad-50
- Glosario del MCU 5.0: disponible en el sitio de AGESIC.
- Planillas de perfiles comunitarios (Básico/Estándar/Avanzado): descargables desde AGESIC (.xlsx).
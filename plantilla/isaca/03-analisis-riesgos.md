# Plantilla ISACA/COBIT 2019 — Análisis y Tratamiento de Riesgos de TI / Ciberseguridad

---

## Encabezado de mapeo normativo

| Marco | Ítem | Detalle / aporte |
|---|---|---|
| **MCU 5.0 (función)** | **Identificar** + **Responder** | Identificación y evaluación de riesgos (ID-02, ID-03); respuesta al riesgo. |
| **MCU 5.0 (categorías)** | ID-02, ID-03, RS-01 | Evaluación de riesgos, gestión de riesgos internos/externos. |
| **COBIT 2019** | APO12 (Gestión del Riesgo), EDM03 | Proceso central de gestión de riesgo. |
| **ISO/IEC 27001:2022** | A.5.1, A.5.8, A.5.28 | Cláusula 6.1 (acciones frente a riesgos) y controles. |
| **ISO 31000** | Todas | Marco internacional de gestión del riesgo. |
| **BCU — Guía de Seguridad de la Información** | Requisito de gestión de riesgo | Evaluación de riesgos periódica y documentada por la Alta Dirección. |
| **URCDP — Ley 18.331** | Art. 9 (medidas de seguridad) | Riesgo del tratamiento de datos personales. |
| **Documento de cumplimiento** | `docs/03-Analisis-Riesgos.md` | Entregable del hito de análisis. |

---

## Control del documento

| Campo | Valor |
|---|---|
| Código | SI-RSK-03 |
| Versión | 1.0 |
| Metodología | ISO 31000 + COBIT APO12 + MCU 5.0 |
| Fecha | `[DD/MM/AAAA]` |

### Historial de versiones

| Versión | Fecha | Autor | Cambios |
|---|---|---|---|
| 1.0 | | | |

---

## 1. Contexto

- **Organización**: `[Organización]`
- **Solución analizada**: `[Solución y alcance]`
- **Criterios de riesgo aceptados por la organización**: `[Ej.: aceptar riesgos < X; exigir tratamiento de los demás]`
- **Horizonte temporal**: `[trimestre / año]`

## 2. Identificación de activos y amenazas

Identifique, para los activos críticos del inventario (`02-registro-activos`), las amenazas relevantes:

| Activo | Amenaza | Origen (interno/externo/accidental/deliberado) | Vulnerabilidad asociada |
|---|---|---|---|
| | | | |

## 3. Análisis de riesgo (cualitativo)

### 3.1 Escala de probabilidad

| Nivel | Valor | Descripción |
|---|---|---|
| Muy baja | 1 | Improbable |
| Baja | 2 | Poco probable |
| Media | 3 | Posible |
| Alta | 4 | Probable |
| Muy alta | 5 | Casi seguro |

### 3.2 Escala de impacto

| Nivel | Valor | Descripción |
|---|---|---|
| Insignificante | 1 | Impacto despreciable |
| Menor | 2 | Pérdida aislada |
| Moderado | 3 | Daño operativo recuperable |
| Mayor | 4 | Daño significativo (multas, clientes) |
| Catastrófico | 5 | Pérdida de negocio / sanción grave |

### 3.3 Matriz de riesgo (5×5)

| I \ P | 1 | 2 | 3 | 4 | 5 |
|---|---|---|---|---|---|
| 5 | M | M | A | A | C |
| 4 | M | M | A | A | A |
| 3 | B | M | M | A | A |
| 2 | B | B | M | M | A |
| 1 | B | B | B | M | M |

> B: Bajo; M: Medio; A: Alto; C: Crítico.

## 4. Registro de riesgos y evaluación

| ID | Riesgo | Activos afectados | Prob. | Impacto | Nivel (B/M/A/C) | Resultado | Tratamiento (M/T/R/A) | Responsable | Plan | Estado | Fecha |
|---|---|---|---|---|---|---|---|---|---|---|---|
| R01 | | | | | | | | | | | |
| R02 | | | | | | | | | | | |

- M = Mitigar; T = Transferir; R = Retener (aceptar); A = Evitar.

## 5. Plan de tratamiento de riesgos

| ID Riesgo | Acción de tratamiento | Control/medida a implementar | Recurso | Prioridad | Fecha objetivo |
|---|---|---|---|---|---|
| | | | | | |

## 6. Riesgo residual y aceptación

| ID | Riesgo residual (post-tratamiento) | Aceptado por | Firma | Fecha |
|---|---|---|---|---|
| | | RSI | | |

---

## Guía de llenado (Blue Team)

| Fase | Cómo completar | Con qué evidencia |
|---|---|---|
| Blue Team - Diseño | Riesgos preliminares de arquitectura y diseño | Diagrama, RF |
| Blue Team - Implementación | Riesgos reales del entorno desplegado (ej.: secretos en BD, puertos expuestos, mail sin TLS) | Escaneos NMAP, `docker exec`, config |
| Pre-Red Team | Identificar riesgos residuales a validar por el Red Team | Formulario/especificación de ataque |
| Red Team | El Red Team usa esta matriz para priorizar vectores de ataque | Informe técnico |

## Check de aceptación

- [ ] Activos críticos con amenazas identificadas.
- [ ] Riesgos evaluados con probabilidad/impacto y nivel.
- [ ] Plan de tratamiento con responsables y fechas.
- [ ] Riesgo residual aceptado formalmente por el RSI.
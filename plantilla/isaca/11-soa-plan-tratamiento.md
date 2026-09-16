# Plantilla ISACA/COBIT 2019 — Declaración de Aplicabilidad (SoA) y Plan de Tratamiento

---

## Encabezado de mapeo normativo

| Marco | Ítem | Detalle / aporte |
|---|---|---|
| **MCU 5.0 (función)** | **Gobernar + Proteger** | Canvas de perfil de cumplimiento y priorización. |
| **MCU 5.0 (categorías)** | GV-01 a GV-06, PR-01 a PR-08 | Perfil Básico/Estándar/Avanzado según corresponda. |
| **COBIT 2019** | EDM01, EDM03, APO13, MEA01 | Gobierno, riesgo y aseguramento. |
| **ISO/IEC 27001:2022** | Anexo A completo + Cláusula 6.1.3 | Documento obligatorio de la norma: controles aplicables o no, con justificación. |
| **ISO/IEC 27002** | Controles del Anexo A | Definición de aplicabilidad de cada control. |
| **BCU — Guía de Seguridad de la Información** | Esquema de requerimientos | RSE/SoA evidencia del cumplimiento de requerimientos BCU. |
| **URCDP — Ley 18.331, Art. 9 y 12** | Medidas de seguridad y confidencialidad | La SoA declara los controles que protegen datos personales. |
| **MCU 5.0 + BCU Com. 2026/098** | Análisis de brechas de madurez | Nivel de madurez por cada función MCU 5.0. |

---

## Control del documento

| Campo | Valor |
|---|---|
| Código | SI-SOA-11 |
| Versión | 1.0 |
| Metodología | ISO 27001 (Anexo A) + MCU 5.0 |
| Fecha | `[DD/MM/AAAA]` |

---

## 1. Alcance de la declaración

- **Sistema**: `[Solución de la letra]`
- **Componentes en alcance**: `[lista de subsistemas]`
- **Criterios de aplicabilidad**: por ejemplo: se excluyen controles organizacionales que no aportan a la solución técnica, mantenibles justificadamente.

## 2. Resumen de estado

| Categoría | Total controles | Aplicables | N/A (con justif.) |
|---|---|---|---|
| A.5 Controles organizacionales | 37 | | |
| A.6 Controles de personas | 4 | | |
| A.7 Controles físicos | 5 | | |
| A.8 Controles tecnológicos | 33 | | |

## 3. Declaración de aplicabilidad

| ID ISO 27001 | Control | ¿Aplica? (Sí/No/N/A) | Justificación | Insumos de la solución | Plan de tratamiento / Estado |
|---|---|---|---|---|---|
| A.5.1 | Políticas de seguridad | Sí | | Política de seguridad | Entregada |
| A.5.9 | Inventario de activos | Sí | | Registro de activos | Entregado |
| A.5.15 | Control de accesos | Sí | | IAM | Implementado |
| A.5.24 | Plan de respuesta a incidentes | Sí | | Gestión incidentes | Implementado |
| A.5.29 | Continuidad de negocio | Sí | | BCP | Implementado |
| A.8.1 | Controles criptográficos | Sí | | encriptación contraseñas | Implementado |
| A.8.8 | Gestión de vulnerabilidades | Sí | | Gestión vuln. | Implementado |
| A.8.15 | Registro y monitorización | Sí | | SIEM/logs | Implementado |
| A.8.16 | Actividades de monitorización | Sí | | SIEM | Implementado |
| ... | ... | | | | |

> La lista debe completarse con el **Anexo A completo** (79 controles ISO 27001:2022), no solo el ejemplo.

## 4. Análisis de brecha MCU 5.0

| Función MCU 5.0 | Perfil objetivo | Evidencia que lo cumple | Madurez actual (0-4) | Acciones |
|---|---|---|---|---|
| Gobernar | Básico/Estándar | Política, roles | | |
| Identificar | Básico/Estándar | Inventario, riesgo | | |
| Proteger | Básico/Estándar | IAM, cifrado, backups | | |
| Detectar | Básico/Estándar | SIEM, logs | | |
| Responder | Básico/Estándar | Incidentes, SOAR | | |
| Recuperar | Básico/Estándar | BCP, restauración | | |

## 5. Plan de tratamiento (resumen)

| ID | Riesgo/Control | Acción | Prioridad | Responsable | Fecha límite |
|---|---|---|---|---|---|
| | | | | | |

---

## Guía de llenado (Blue Team)

| Fase | Cómo completar | Con qué evidencia |
|---|---|---|
| Blue Team - Diseño | Declarar aplicabilidad preliminar | RF/RNF, arquitectura |
| Blue Team - Implementación | Completar el Anexo A completo con evidencia | Evidencias de despliegue |
| Blue Team - MCU 5.0 | Calcular madurez por función | Evidencias por categoría |
| Red Team | Validar las afirmaciones de la SoA | Informe Red Team |
| Cierre | Actualizar brechas y brechas residuales | Plan de mejora |

## Check de aceptación

- [ ] Anexo A completo con aplicabilidad justificada.
- [ ] Brecha MCU 5.0 por función con objetivo de perfil.
- [ ] Plan de tratamiento con fechas y responsables.
- [ ] Vinculados los entregables `docs/*` como evidencia.
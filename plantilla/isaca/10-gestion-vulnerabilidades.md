# Plantilla ISACA/COBIT 2019 — Gestión de Vulnerabilidades

---

## Encabezado de mapeo normativo

| Marco | Ítem | Detalle / aporte |
|---|---|---|
| **MCU 5.0 (función)** | **Identificar** + **Proteger** | Gestión de vulnerabilidades y remediación. |
| **MCU 5.0 (categorías)** | ID-03 Vulnerabilidades, PR-01 | Detección/remediación de vulnerabilidades. |
| **COBIT 2019** | APO12, DSS05 | Proceso de gestión de vulnerabilidades y parches. |
| **ISO/IEC 27001:2022** | A.8.8 Gestión de vulnerabilidades técnicas | Pruebas, priorización, parcheo. |
| **ISO/IEC 27002** | 8.8 | Guía de gestión de vulnerabilidades. |
| **BCU — Guía de Seguridad de la Información** | Requisito de gestión de vulnerabilidades/parches | Mantener sistemas actualizados y gestionar vulnerabilidades críticas. |
| **URCDP — Ley 18.331, Art. 9** | Seguridad de datos personales | La remediación de vulnerabilidades previene incidentes de datos personales. |
| **OWASP / MITRE ATT&CK (referencial)** | Pruebas de seguridad | Marco técnico de pruebas y técnicas. |

---

## Control del documento

| Campo | Valor |
|---|---|
| Código | SI-VUL-10 |
| Versión | 1.0 |
| Responsable | `[Nombre]` |
| Fecha | `[DD/MM/AAAA]` |

---

## 1. Herramientas utilizadas

| Herramienta | Rol | Alcance |
|---|---|---|
| OpenVAS / Greenbone | Escaneo de vulnerabilidades | Red, hosts |
| NMAP / Masscan | Inventario de puertos y servicios | Red |
| nuclei / nikto | Web app testing | Apps |
| Dependency-check / Trivy | Dependencias y contenedores | Docker images |
| gitleaks / bandit / semgrep | Secretos / SAST | Código |
| Wazuh FIM/CVE | Detección de parches faltantes | Hosts |

## 2. Registro de vulnerabilidades

| ID | CVSS v3 | Activo afectado | Herramienta | Descripción | Estado (abierta/rem) | Fecha detect | Fecha rem | Responsable |
|---|---|---|---|---|---|---|---|---|
| V01 | | | | | | | | |

## 3. Priorización de remediación

| Gravedad | CVSS | SLA de remediación |
|---|---|---|
| Crítica | 9.0–10.0 | 72 h |
| Alta | 7.0–8.9 | 7 días |
| Media | 4.0–6.9 | 30 días |
| Baja | 0.1–3.9 | 90 días |

## 4. Ventana de parcheo

- Lunes a `[días]`, ventana `[hora]`.
- Se debe aplicar en entorno de prueba primero.
- Revisión mensual de tendencias.

## 5. Tratamiento de falsos positivos

| ID | Hallazgo | Justificación | Aprobado por | Fecha |
|---|---|---|---|---|
| | | | | |

---

## Guía de llenado (Blue Team)

| Fase | Cómo completar | Con qué evidencia |
|---|---|---|
| Blue Team - Implementación | Escaneo inicial y remediación antes de entrega | Escaneo OpenVAS/Nmap |
| Blue Team - Seguridad por diseño | SAST análisis de código | Informe semgrep/bandit |
| Red Team | Ejecuta su propio escaneo/pentest | Informe Red Team → nuevos IDs V |
| Cierre | Cerrar vulnerabilidades residuales justificando | Tablero de cierre |

## Check de aceptación

- [ ] Al inicio de la entrega a Red Team: sin vulnerabilidades críticas/altas abiertas sin mitigación.
- [ ] Registro con CVSS y SLA.
- [ ] Evidencia de remediación o justificación de riesgo aceptado.
- [ ] SAST incorporado al ciclo de desarrollo.
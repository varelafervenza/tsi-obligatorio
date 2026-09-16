# Plantilla ISACA/COBIT 2019 — Monitoreo, Logs y Registro de Eventos (SIEM/SOAR)

---

## Encabezado de mapeo normativo

| Marco | Ítem | Detalle / aporte |
|---|---|---|
| **MCU 5.0 (función)** | **Detectar** | Detección temprana de eventos y análisis continuo. |
| **MCU 5.0 (categorías)** | DE-01 Anomalías y eventos, DE-02 Monitoreo continuo, DE-03 Análisis de detección | Monitoreo de red, hosts y apps. |
| **COBIT 2019** | DSS04 (Continuidad y monitoreo), DSS05 (Operación de seguridad) | Registro y revisión de eventos. |
| **ISO/IEC 27001:2022** | A.8.15 Registro y monitorización, A.8.16 Actividades de monitorización | Logging, SIEM, revisión. |
| **NIST SP 800-92 (referencial)** | Guía de gestión de logs; SIEM | Retención y correlación. |
| **BCU — Guía de Seguridad de la Información** | Requisitos de monitoreo y registro | Monitoreo de eventos de seguridad y cambios en datos sensibles. |
| **URCDP — Ley 18.331, Art. 9 y 12** | Seguridad de datos personales | Registro de accesos y trazabilidad del tratamiento de datos personales. |
| **MCU 5.0 + BCU Com. 2026/098** | Evidencia del nivel de madurez en Detectar | Indicadores de monitoreo. |

---

## Control del documento

| Campo | Valor |
|---|---|
| Código | SI-MON-07 |
| Versión | 1.0 |
| Responsable | `[Nombre]` |
| Fecha | `[DD/MM/AAAA]` |

---

## 1. Arquitectura de monitoreo

| Capa | Herramienta utilizada (open source) | Rol |
|---|---|---|
| Red (NIDS) | Suricata / Zeek / Security Onion | Detección de tráfico de red |
| Host (HIDS) | Wazuh / OSSEC | Integridad y detección en endpoints |
| SIEM | Wazuh / ELK / Security Onion | Correlación y almacenamiento |
| SOAR / Orquestación | Wazuh Active Response / TheHive / Shuffle | Respuesta automatizada |
| Alertas | Alertmanager / Grafana / TheHive | Notificación |

## 2. Fuentes de log

| Fuente | Componente | Eventos a registrar | Relevancia |
|---|---|---|---|
| BD PostgreSQL/MySQL | `[gestor]` | Conexiones, errores, DDL/DML | Alta |
| Servidor web / API | `[nginx]` | Accesos, códigos 4xx/5xx, intentos | Alta |
| Mail | `[postfix/dovecot]` | Auth, envío, fallos TLS | Media |
| Marco de identidad (TOTP/SSO) | `[Keycloak etc.]` | Autenticaciones, TOTP | Alta |
| SO (servidores y clientes) | `[auditd/journal]` | Login, sudo, cambios | Alta |

## 3. Casos de uso / reglas de detección

| ID | Nombre | Fuente | Condición de alerta | Severidad | Respuesta |
|---|---|---|---|---|---|
| CU-01 | Fuerza bruta SSH | suricata/wazuh | N intentos fallidos en X min | Media | Bloqueo + alerta |
| CU-02 | Cambio de password maestra | gestor contraseñas | Evento "master_password_change" | Crítica | Alerta por mail, revalidación |
| CU-03 | Root login externo | auditd | login como root | Alta | Investigación |
| CU-04 | exfiltración de archivos | suricata | TLS a dominio malicioso | Alta | Análisis de SOC |

## 4. Retención y almacenamiento

| Índice/log | Retención online | Retención fría | Formato |
|---|---|---|---|
| Eventos SIEM | `[30 d]` | `[90 d]` | JSON / parquet |
| Logs crudos (auditd) | `[30 d]` | `[180 d]` | Mensaje |
| Alertas y casos (TheHive) | `[1 año]` | `[2 años]` | Caso |

## 5. Revisión y operación

- Revisión diaria de `[alertas/casos]` por `[responsable]`.
- SLA de triage: `[ej. alta en < 1 h]`.
- Métricas: número de eventos, alertas, falsos positivos, MTTR de detección.

## 6. Evidencias de detección (KPIs)

| Indicador | Valor medido durante la prueba |
|---|---|
| Eventos de anormalidad detectados | `[n]` |
| Alertas críticas generadas | `[n]` |
| Tiempo de detección promedio | `[min]` |
| Falsos positivos | `[n]` |

---

## Guía de llenado (Blue Team)

| Fase | Cómo completar | Con qué evidencia |
|---|---|---|
| Blue Team - Implementación | Desplegar SIEM/agentes, definir fuentes de log | Capturas de paneles, `agents list` |
| Blue Team - Reglas | Implementar casos de uso y probarlos con ataques simulados | Log de detección |
| Blue Team - Demo | Generar alertas reales y registrarlas | Alertas → ticket |
| Red Team | Provee tráfico/actividad real para medir detección | Informe Red Team |
| Cierre | Ajustar reglas con lecciones aprendidas | Mejora continua |

## Check de aceptación

- [ ] Todos los componentes de la solución emiten logs al SIEM.
- [ ] Al menos 4 casos de uso implementados (incluye cambio de master password).
- [ ] Retención definida y respaldo de logs.
- [ ] Evidencia de alertas reales durante la validación.
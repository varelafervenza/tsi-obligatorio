# Monitoreo, Logs y Registro de Eventos — Gestor de Contraseñas con Control Centralizado

> Basado en `plantilla/isaca/07-monitoreo-logs.md`. Función **Detectar** del MCU 5.0. Describe lo
> que está implementado hoy y lo que falta, sin declarar detecciones que todavía no se probaron.

---

## Encabezado de mapeo normativo

| Marco | Ítem | Detalle / aporte |
|---|---|---|
| **MCU 5.0 (función)** | Detectar | DE-01 (eventos y anomalías), DE-02 (monitoreo continuo), DE-03 (análisis). |
| **COBIT 2019** | DSS04, DSS05 | Registro y revisión de eventos. |
| **ISO/IEC 27001:2022** | A.8.15, A.8.16 | Registro y monitorización. |
| **BCU — GSI** | Monitoreo de eventos y cambios en datos sensibles | Cambio de contraseña maestra y borrados masivos. |
| **URCDP — Ley 18.331, Art. 9 y 12** | Trazabilidad del tratamiento | Registro de cada alta, modificación, borrado y cambio de maestra. |

---

## Control del documento

| Campo | Valor |
|---|---|
| Código | SI-MON-07 |
| Versión | 0.1 (borrador) |
| Responsable | Blue Team (Horacio Duarte, Pablo Morales, Andrés Varela) |
| Fecha | 05/10/2026 |

---

## 1. Arquitectura de monitoreo

| Capa | Herramienta | Estado | Rol |
|---|---|---|---|
| Red (NIDS) | Suricata / Zeek | **No implementado** (decisión de alcance, ver `03-Analisis-Riesgos.md` y el Excel de controles). | — |
| Host (HIDS) | Wazuh (agente en el cliente) | **Planificado.** El agente y su FIM sobre la bóveda no están desplegados. | Integridad del archivo de bóveda |
| Eventos de la aplicación | `control-central` → JSONL | **Implementado.** Cada evento se escribe en `infra/logs/audit-events.jsonl` sin secretos. | Fuente para el SIEM |
| SIEM | Wazuh manager + reglas `infra/wazuh/local_rules.xml` | **Reglas escritas, manager no levantado.** | Correlación |
| Correlación local | API `control-central` (`app/siem/reglas.py`) | **Implementado.** Aplica los mismos umbrales que las reglas de Wazuh y guarda alertas en la tabla `alerts`. | Alertas del panel |
| Panel | Grafana (tablero "Control central") y `GET /api/dashboard/kpis` | **Implementado.** | Visualización |
| Notificación | SMTP a Mailpit (dev) | **Implementado** con Mailpit. Mailu queda como limitación. | Aviso al RSI |

---

## 2. Fuentes de log

| Fuente | Qué se registra | Estado | Relevancia |
|---|---|---|---|
| `control-central` (eventos) | Tipo, sistema, agente, `occurred_at`, `received_at`, IP de origen y resultado de la firma. Sin `firma_jws` ni secretos. | Implementado | Alta |
| Cliente `cliente-gestor` | Eventos firmados: alta, modificación, borrado, cambio de maestra, intento fallido de apertura, vencimiento. | Implementado. Los intentos locales sólo se envían al central cuando hay red; si no, quedan en la cola `cola-eventos.jsonl`. | Alta |
| PostgreSQL | Conexiones y errores | **No configurado.** | Media |
| Sistema operativo (Windows) | Inicios de sesión, cambios | **No configurado.** Depende del agente de Wazuh. | Media |
| Panel del central (usuarios, login) | Logins del panel | **No registrado como evento.** Pendiente. | Alta |

---

## 3. Casos de uso / reglas de detección

| ID | Nombre | Fuente | Condición de alerta | Severidad | Estado |
|---|---|---|---|---|---|
| CU-01 | Fuerza bruta de la maestra | Eventos `intento_fallido_maestra` | 5 intentos del mismo agente en 2 minutos. Regla Wazuh 100101 y correlación en la API. | Alta (nivel 10) | **Probado.** Alerta `fuerza_bruta_maestra` (id 1) generada al quinto intento. |
| CU-02 | Borrado masivo de credenciales | Eventos `borrado_credencial` | 5 borrados del mismo agente en 2 minutos. Regla Wazuh 100111 y correlación en la API. | Alta (nivel 10) | **Probado.** Alerta `borrado_masivo` (id 2) generada al quinto borrado. |
| CU-03 | Cambio de contraseña maestra | Eventos `cambio_maestra` | Una alerta por evento. Regla Wazuh 100120 y correlación en la API. Además, correo inmediato al RSI. | Crítica (nivel 12) | **Probado.** Alerta `cambio_maestra` (id 3) y correo recibido en Mailpit. |
| CU-04 | Firma inválida repetida | Eventos con `firma_valida: false` | Se registra en el panel (`alertas.firma_invalida`). No hay regla de correlación. | Media | Parcial: visible en el panel, sin regla. |

Los casos de uso típicos de SSH, root o exfiltración por TLS no aplican a esta solución: no hay
servidores SSH ni un NIDS de red.

---

## 4. Retención y almacenamiento

| Índice / log | Retención online | Retención fría | Formato | Estado |
|---|---|---|---|---|
| Eventos del central (`audit_events`) | 90 días | No definida | Tabla SQL | Se purgan al arrancar `control-central` los eventos con más de 90 días. Se conservan los que son origen de un incidente o una alerta. |
| Archivo JSONL para el SIEM | No definida | No definida | JSON por línea | **Pendiente:** rotación y retención del archivo. |
| Alertas (`alerts`) | No definida | No definida | Tabla SQL | Pendiente. |
| Incidentes (`incidents`) | No definida | No definida | Tabla SQL | Pendiente. |

---

## 5. Revisión y operación

- Revisión de alertas y de incidentes abiertos por el RSI (Andrés Varela), según la RACI
  (`docs/mcu5/excel/03-matriz-raci-mcu5.xlsx`).
- Triage y clasificación de falsos positivos con `PATCH /api/alerts/{id}`.
- **SLA de triage:** pendiente de definir por el equipo.
- Métricas: eventos, alertas, falsos positivos (tasa) y MTTD/MTTR, expuestos en `GET /api/dashboard/kpis`.

---

## 6. Evidencias de detección (KPIs)

Valores leídos de `GET /api/dashboard/kpis` el 05/10/2026, después de la demo:

| Indicador | Valor medido | Comentario |
|---|---|---|
| Eventos recibidos | 5 (antes de la simulación) | Todos de tipo `alta_credencial`. Después de la simulación: 17 en total, con 10 `intento_fallido_maestra`, 10 `borrado_credencial` y 2 `cambio_maestra`. |
| Agentes activos | 2 | `agente-dev-01` y `agente-f3815a26`. |
| Firmas inválidas | 2 | Eventos sin clave instalada o sin firma, por diseño de la prueba. |
| Alertas de las reglas | 4 | fuerza_bruta_maestra (1), borrado_masivo (1), cambio_maestra (2). Incluye un cambio de maestra extra por la segunda corrida (ver nota). |
| Incidentes abiertos | 0 | |
| MTTD / MTTR | MTTD 0.2 s; MTTR 1.9 s | MTTD: primera alerta menos `occurred_at` del evento (06/10). MTTR: creado → resuelto del incidente 1; pendiente medirlo desde la alerta. |
| Falsos positivos | N/D | Sin alertas clasificadas. |

**Evidencia de la simulación:** `docs/evidencias/07-simulacion-casos-uso.txt`, generada con
`scripts/simular_casos_uso.py`. El script se ejecutó **dos veces** el 05/10/2026: la primera
generó las alertas 1 a 3, y la segunda, que quedó en la evidencia, agregó la alerta 4 y más
eventos. Los eventos son simulados (agente de prueba), no un ataque real.

**Pendiente:** registrar el incidente asociado a una alerta (flujo alerta → incidente → cierre),
y repetir la simulación con una sola corrida limpia si se quiere una evidencia sin duplicados.

---

## Guía de llenado (Blue Team)

| Fase | Cómo completar | Con qué evidencia |
|---|---|---|
| Implementación | Levantar el manager de Wazuh, conectar el JSONL y registrar los agentes | `agents list`, capturas del manager |
| Reglas | Disparar CU-01, CU-02 y CU-03 con eventos reales | Alertas en `/api/alerts/` y en Wazuh |
| Demo | Mostrar alerta → incidente → cierre | Capturas en `docs/evidencias/` |
| Red Team | Atacará la detección (evasión, borrado de reglas) | Informe Red Team |

---

## Check de aceptación

- [x] Los eventos del cliente y del central llegan al log del SIEM (JSONL).
- [ ] Al menos 4 casos de uso probados con alertas reales (hoy: 3 probados, CU-04 es parcial).
- [x] Retención de eventos de 90 días definida en el código.
- [ ] Retención de logs crudos y respaldo del archivo JSONL.
- [x] Evidencia de alertas durante la validación (simulada, ver `07-simulacion-casos-uso.txt`).

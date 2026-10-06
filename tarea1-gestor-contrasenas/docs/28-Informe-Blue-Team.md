# Informe final del Blue Team — Gestor de contraseñas con control centralizado

| Campo | Valor |
|---|---|
| Equipo | Blue Team: Horacio Duarte, Pablo Morales, Andrés Varela |
| Versión | 0.1 (borrador para la pre-entrega del 07/10/2026) |
| Fecha | 06/10/2026 |
| Estado | Borrador. Pendiente la firma del RSI y el tag `v1.0` |
| Documentos de referencia | `00-arquitectura-c4.md`, `02-Registro-Activos.md`, `03-Analisis-Riesgos.md`, `06-Plan-Continuidad.md`, `07-Monitoreo-Logs-SIEM.md`, `09-Gestion-Accesos.md`, `10-Gestion-Vulnerabilidades.md`, `11-SoA-Plan-Tratamiento.md`, `12-Notificacion-Incidentes.md`, `guia-de-pruebas.md` |

---

## 1. Resumen ejecutivo

El sistema se compone de un cliente de escritorio (Tauri 2, Windows), un servidor de control central (FastAPI y PostgreSQL) y un canal de eventos firmados. El control central recibe la auditoría, la guarda, dispara alertas por reglas, envía correos y muestra un panel con KPIs.

Lo que está demostrado con evidencia:

- Los eventos firmados con RS-256 se aceptan y los que no tienen firma válida se rechazan con `firma_valida: false`.
- Las tres reglas de detección (fuerza bruta de la maestra, borrado masivo y cambio de la maestra) generan alertas en vivo. La simulación queda registrada en `docs/evidencias/07-simulacion-casos-uso.txt`.
- Hay un incidente de prueba abierto desde una alerta y resuelto, con su línea de tiempo.
- La restauración de la base con datos (29 eventos, 4 alertas, 1 incidente, 1 usuario del panel) coincide en las cuatro tablas, en 1 s, con el hash del dump verificado.
- La bóveda se puede exportar e importar cifrada, y eso se prueba en CI.

Lo que no cumple todavía, y se declara abierto en la sección 6:

- La cobertura de eventos del RNF/RF-07 queda en 75 %, no en 100 %. Falta el tipo `modificacion_credencial`.
- Los KPIs MTTD y MTTR del panel no miden lo que dice su nombre (ver sección 4.2).
- El SIEM Wazuh está modelado y su regla escrita, pero el manager no está levantado. Por eso la tasa de falsos positivos del SIEM no se puede calcular.
- Hay limitaciones de alcance aceptadas con la cátedra: TLS en el laboratorio, y Windows Hello y WebAuthn fuera de la entrega.

---

## 2. Alcance y arquitectura

El detalle está en `00-arquitectura-c4.md`. Resumen de componentes:

| Componente | Tecnología | Rol |
|---|---|---|
| `cliente-gestor` | Tauri 2, React 18, Rust | Bóveda local cifrada, agente de eventos, modo offline |
| `control-central` | FastAPI 0.115, SQLAlchemy 2, PostgreSQL 16 | Recepción de eventos, reglas, alertas, incidentes, panel |
| Correo | Mailpit (prototipo); Mailu previsto | Notificaciones a RSI |
| Panel | Grafana 11.2 | Visualización sobre PostgreSQL |
| SIEM | Wazuh (reglas escritas; manager no desplegado) | Correlación externa prevista |

Dependencia del canal: los eventos se firman con una clave privada por agente (RS-256). El control central tiene la clave pública de cada agente en `keys/agentes/`.

---

## 3. Cumplimiento por función del MCU 5.0

Los detalles por control están en `11-SoA-Plan-Tratamiento.md` y en el Excel `docs/mcu5/excel/01-controles-mcu5-perfil-avanzado.xlsx`.

| Función | Evidencia principal | Estado |
|---|---|---|
| **Gobernar** | Política (`01`, borrador 0.1), registro de activos (14 activos), análisis de riesgos (12 riesgos), SoA (93 controles: 73 aplican, 20 N.A.) | Parcial. Política sin firma del RSI |
| **Identificar** | Activos A01–A14, riesgos R01–R12 de RT-01 a RT-12, vulnerabilidades V01–V04 | Parcial |
| **Proteger** | Argon2id, XChaCha20-Poly1305, RS-256 para eventos, TOTP en login y en el panel, token por login, listado sin secretos | Parcial. TLS y Hello/WebAuthn fuera de la entrega |
| **Detectar** | Reglas de la API con alertas en vivo (CU-01 a CU-03), panel de KPIs, log JSONL sin secretos | Parcial. Wazuh sin manager |
| **Responder** | Procedimiento de incidentes (`04`), incidente de prueba resuelto, notificación simulada BCU/URCDP (`12`) | Parcial. Sin bloqueo automático del origen |
| **Recuperar** | Plan de continuidad (`06`), backup `pg_dump` con SHA-256, restauración probada con datos | Parcial. Backup diario y copia externa pendientes |

Madurez MCU 5.0 estimada por el equipo: entre 2 y 3 sobre 5. Es una estimación que la auditoría del 14/10 debe validar.

---

## 4. KPIs de la sección 6.4 de la letra

### 4.1 Valores medidos

Los valores salen de `GET /api/dashboard/kpis` el 06/10/2026 (18:43 UTC).

| KPI | Requisito | Valor en el panel | Muestras | Cumple |
|---|---|---|---|---|
| MTTD | Tiempo de detección de un ataque simulado | 77 880 s (21 h 38 min) | 1 | No medido de forma correcta (ver 4.2) |
| MTTR | Tiempo de respuesta a incidente | 1.9 s | 1 | No medido de forma correcta (ver 4.2) |
| Cobertura de eventos | 100 % de alta, modificación, borrado y cambio de maestra | 75 % (faltó `modificacion_credencial`) | — | **No** |
| Falsos positivos del SIEM | Tasa de falsos positivos | Sin valor (`null`) | 0 SIEM / 4 de reglas locales | No medido |
| Uptime del control central | Durante la validación | 2 884 s desde el último arranque | — | No medido para el periodo de validación |

### 4.2 Lectura de los KPIs (lo que hay que saber antes de defenderlos)

Los tiempos de detección y respuesta reales del caso de prueba son estos, tomados de la alerta y del incidente:

| Tiempo | Valor real | Cómo se obtiene |
|---|---|---|
| Primer intento fallido → alerta (detección) | **3 s** (20:29:48 → 20:29:51 UTC, 05/10) | `occurred_at` del evento 6 y `creado_en` de la alerta 1 |
| Alerta → resolución del incidente | **21 h 38 min** (05/10 20:29:51 → 06/10 18:07:52 UTC) | `creado_en` de la alerta y `resuelto_en` del incidente |

Problema detectado en la revisión de este informe: el panel calcula:

- **MTTD** como `creado_en` del incidente menos `occurred_at` del evento. Es el tiempo que tardó una persona en abrir el caso, no el de detección. Por eso da 21 h 38 min en vez de 3 s.
- **MTTR** como `resuelto_en` menos `creado_en` del incidente. El incidente se creó y se resolvió en la misma prueba, así que da 1.9 s y no mide la respuesta real.

Decisión: el informe reporta los dos tiempos reales (3 s y 21 h 38 min) y no los valores del panel como si fueran la respuesta del equipo. Ajustar la definición del MTTD en el código (usar la fecha de la alerta, no la del incidente) queda como tarea pendiente antes del 07/10. Si no se llega, se declara en la defensa.

### 4.3 Qué falta para cumplir 6.4

- **Cobertura del 100 %:** generar un evento `modificacion_credencial` con la guía de pruebas y repetir la medición.
- **Falsos positivos:** requiere alertas de Wazuh clasificadas. Hay que levantar el manager.
- **Uptime:** medir durante la ventana de validación con una sonda de `healthz` registrada en el tiempo.

---

## 5. Incidentes y simulaciones

| ID | Escenario | Evidencia | Resultado |
|---|---|---|---|
| CU-01 | 5 intentos fallidos de la maestra en menos de 2 min | `07-simulacion-casos-uso.txt`, alerta 1 | Alerta generada al quinto intento, 3 s después del primero |
| CU-02 | 5 borrados de credenciales | Alerta 2 | Alerta `borrado_masivo` generada |
| CU-03 | Cambio de contraseña maestra | Alertas 3 y 4 | Alerta inmediata. Hay una alerta extra por la segunda corrida del script, documentada |
| Incidente 1 | Creado desde la alerta 1 | `06-restauracion-bd-con-datos.txt`, `12-Notificacion-Incidentes.md` | Resuelto. Severidad S2. Sin notificación a BCU ni URCDP (no hubo acceso a datos) |

Nota de la corrida doble: el script `simular_casos_uso.py` se ejecutó dos veces el 05/10. La primera corrida generó las alertas 1 a 3, y la segunda agregó la 4. Está documentado en la evidencia y en `07`.

---

## 6. Vulnerabilidades y limitaciones abiertas

### 6.1 Vulnerabilidades (`10-Gestion-Vulnerabilidades.md`)

| ID | Descripción | Estado |
|---|---|---|
| V01 | `python-jose` con avisos | Remediado (3.4.0) |
| V02 | `python-multipart` con avisos | Remediado (0.0.31) |
| V03 | `starlette` (dependencia de FastAPI 0.115.0) con avisos | **Abierta.** Subir FastAPI queda para el final por riesgo de regresión. CVSS pendiente |
| V04 | `ecdsa` sin corrección | Riesgo aceptado: RS-256 no usa ECDSA |

Herramientas corridas: `pip-audit` antes y después, `bandit` con 0 issues, `npm audit` con 0 vulnerabilidades.

No corridas todavía: escaneo de red y web (nmap, OpenVAS o nuclei), Trivy y `cargo audit`.

### 6.2 Limitaciones de alcance aceptadas

| Limitación | Motivo | Riesgo residual |
|---|---|---|
| TLS en el laboratorio | Costo del certificado; una CA privada obliga a instalar la raíz en cada equipo de evaluación | Tráfico en claro dentro de la red del laboratorio. Documentado en `00-arquitectura-c4.md` |
| Windows Hello y WebAuthn | Fuera de la entrega, acordado con la cátedra. No se pudo validar sin sensor biométrico | Sin segundo factor biométrico. Queda TOTP |
| Smart App Control | El instalador no está firmado (costo de firma) | Requiere desactivar SAC o usar Windows sin esa función para evaluar |

### 6.3 Pendientes que afectan la entrega

- Wazuh manager levantado con al menos una alerta real, o limitación documentada.
- Mailu con SPF y DKIM, o justificar Mailpit ante la cátedra.
- Consulta por escrito a la cátedra sobre el Anexo A (no está en el repo) y la postura sobre Mailpit y Wazuh.
- Firma del RSI en `01`, `06`, `11` y `12`.
- Backup diario automático (tarea programada) y copia fuera del equipo.
- Copia de `keys/agentes/` (decisión de custodia de la clave privada).
- Tests automáticos en Python para `control-central` (firma, retención e incidentes): no existen.
- Medir RNF-02 (apertura en menos de 2 s) y RNF-05 (carga básica), y definir el plan de rollback (RNF-10).
- Video de la demo de 5 min o menos.
- Tag `v1.0` y hash SHA-256 con `scripts/verificar_integridad_tag.sh` el 07/10.

---

## 7. Trazabilidad

Cada control validado en la auditoría se relaciona con entradas de `99-bitacora-trabajo.md` por fecha y responsable. Las entradas del 04/10 al 06/10 cubren la simulación, la restauración con datos, la guía de pruebas, la SoA y esta notificación.

Pendiente: la bitácora Excel `04-bitacora-planilla.xlsx` tiene las entradas hasta el 05/10. Hay que espejar las del 06/10 en adelante.

---

## 8. Conclusión

El equipo tiene un sistema que funciona de punta a punta: firma, persistencia, reglas, alertas, incidente, notificación simulada y restauración. Lo que queda abierto es medible y está nombrado: cobertura al 100 %, definición correcta de MTTD y MTTR, SIEM con manager levantado, vulnerabilidades V03 y firma del RSI. Para la auditoría del 14/10, el equipo presenta estos puntos como están, sin ajustar los valores para que cumplan.

---

## Check de aceptación

- [x] Resumen de cumplimiento por función MCU 5.0 con evidencias.
- [x] KPIs de 6.4 con valores medidos, definición y estado.
- [x] Incidentes simulados y su resultado.
- [x] Vulnerabilidades y limitaciones de alcance.
- [ ] Cobertura de eventos al 100 %.
- [ ] MTTD y MTTR con la definición corregida.
- [ ] Firma del RSI.

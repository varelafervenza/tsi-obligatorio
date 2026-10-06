# Análisis y Tratamiento de Riesgos — Gestor de Contraseñas con Control Centralizado

> Basado en `plantilla/isaca/03-analisis-riesgos.md`. Los riesgos se derivaron de los **objetivos del Red Team
> (RT-01 a RT-12, sección 7.2 de `LETRA.md`)**. Es la misma lista de ataques que el Red Team va a intentar entre el
> 28/10 y el 09/11, así que el Blue Team la trata como su propio catálogo de amenazas. La versión 2 agrega el estado
> verificado de cada control al 06/10/2026 y un riesgo nuevo (R13).

---

## Encabezado de mapeo normativo

| Marco | Ítem | Detalle / aporte |
|---|---|---|
| **MCU 5.0** | **Identificar** (ID-02, ID-03) + **Responder** (RS-01) | Evaluación y tratamiento de riesgos. |
| **COBIT 2019** | APO12, EDM03 | Gestión del riesgo. |
| **ISO/IEC 27001:2022** | A.5.1, A.5.8, A.5.28 | Cláusula 6.1 y controles asociados. |
| **BCU — GSI** | Evaluación de riesgo periódica | Insumo para la Alta Dirección. |
| **URCDP — Ley 18.331, Art. 9** | Riesgo del tratamiento de datos personales | Ver riesgos sobre A02, A05, A07, A10. |

---

## Control del documento

| Campo | Valor |
|---|---|
| Código | SI-RSK-03 |
| Versión | 2.0 (revisión del 06/10/2026, borrador; falta la firma del RSI) |
| Metodología | ISO 31000 + COBIT APO12 + MCU 5.0. Catálogo de amenazas = objetivos RT-01..RT-12, más R13 |
| Fecha | 06/10/2026 |

### Historial de versiones

| Versión | Fecha | Autor | Cambios |
|---|---|---|---|
| 0.1 | 22/09/2026 | Blue Team | Primera versión: 12 riesgos derivados de RT-01..RT-12, sobre el inventario `02-Registro-Activos.md`. |
| 2.0 | 06/10/2026 | Blue Team | Estado verificado de cada control en el código y la infraestructura. Se corrige el conteo de riesgos Alto (5, no 4). Se agrega R13 (PostgreSQL publicado en el host). Se actualizan los tratamientos y las fechas vencidas. |

## 1. Contexto

- **Organización**: Laboratorio del curso Seguridad de la Información (equipo Blue Team).
- **Solución analizada**: Gestor de Contraseñas con Control Centralizado (Tarea 1). Alcance completo:
  `cliente-gestor`, `control-central`, Wazuh, Mailu (previsto; hoy Mailpit), Grafana, repositorio Git.
- **Criterios de riesgo aceptados por la organización**: los de nivel **Bajo** se aceptan sin tratamiento adicional. Los de
  nivel **Medio** se mitigan si el costo y el tiempo lo permiten antes del 07/10/2026. Los de nivel **Alto** y **Crítico**
  deben tratarse antes de la pre-entrega (H4, 07/10/2026), porque son lo que el Red Team va a intentar explotar.
- **Horizonte temporal**: hasta el cierre de la Parte 2 / Red Team (09/11/2026).

## 2. Identificación de activos y amenazas

| Activo | Amenaza (objetivo Red Team) | Origen | Vulnerabilidad asociada |
|---|---|---|---|
| A01, A02 | Fuerza bruta / crackeo de la derivación de clave de la maestra (RT-01) | Externo, deliberado | KDF mal parametrizada o sin delay adaptativo |
| A03, A04, A06 | Forjado de eventos de auditoría (RT-02) | Externo, deliberado | Clave de firma compartida o débilmente protegida |
| A04, A05, A07 | IDOR / XSS / inyección SQL sobre el control central (RT-03) | Externo, deliberado | Validación de entrada insuficiente, queries no parametrizadas |
| A10 | Suplantación del servidor de correo (RT-04) | Externo, deliberado | SPF/DKIM/DMARC ausentes o relay abierto |
| A04, A14 | Man-in-the-middle por TLS mal configurado (RT-05) | Externo, deliberado | Certificados autofirmados aceptados sin validación/pinning |
| A01, A07 | Bypass de MFA (phishing TOTP, enroll ajeno) (RT-06) | Externo, deliberado | Falta de verificación fuera de banda al enrolar un factor |
| A11 | XSS almacenado en el dashboard (RT-07) | Externo, deliberado | Campos de evento sin sanitizar al renderizar |
| A08, A09 | Evasión de la detección del SIEM (RT-08) | Interno/externo, deliberado | Reglas/agentes sin protección de integridad ni alerta de desconexión |
| A02 | Abuso de import/export para degradar cifrado (RT-09) | Externo, deliberado | Parámetros del KDF tomados del archivo sin límites |
| A02, A05 | Exfiltración de secretos por errores de la app (RT-10) | Interno, accidental | Logs de depuración o stack traces expuestos en producción |
| A13 | OSINT del repositorio: secretos commiteados, Dockerfiles vulnerables (RT-11) | Externo, deliberado | Falta de escaneo de secretos previo al push |
| A04 | Denegación de servicio puntual al control central (RT-12) | Externo, deliberado | Ausencia de límites de tasa en la API |
| A04, A05 | Acceso directo a la base desde la red del host (R13, hallazgo nuevo) | Externo o interno | PostgreSQL publicado en el puerto 5432 del host |

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

Las columnas de nivel son las de la v1 (evaluación inherente). La columna **Estado verificado al 06/10** resume lo que
se comprobó en el código y en la infraestructura. Ver la sección 5 para la acción.

| ID | Riesgo | Activos | Prob. | Imp. | Nivel | Tratamiento | Estado verificado al 06/10 | Fecha objetivo |
|---|---|---|---|---|---|---|---|---|
| R01 | Fuerza bruta / crackeo de la maestra (RT-01) | A01, A02 | 3 | 5 | **A** | Mitigar | **Parcial.** Argon2id y delay adaptativo implementados (`vault/commands.rs`). Medidor de fortaleza de la maestra no encontrado en el código. | 02/10/2026 (vencida) |
| R02 | Forjado de eventos de auditoría (RT-02) | A03, A04, A06 | 2 | 4 | M | Mitigar | **Parcial.** Firma RS-256 con clave por agente implementada y verificada. Alerta por firma inválida repetida: no hay regla. | 29/09/2026 (vencida) |
| R03 | IDOR / XSS / SQLi en el control central (RT-03) | A04, A05, A07 | 3 | 5 | **A** | Mitigar | **Parcial.** SQLAlchemy parametrizado y validación con Pydantic. `bandit` sin hallazgos (`evidencias/10-bandit.txt`). Sin pruebas DAST ni chequeo de IDOR. | 05/10/2026 (vencida) |
| R04 | Suplantación del servidor de correo (RT-04) | A10 | 3 | 3 | M | Mitigar | **No tratado.** Mailu no está desplegado (prototipo con Mailpit). Sin SPF, DKIM ni DMARC. | 29/09/2026 (vencida) |
| R05 | MITM por TLS mal configurado (RT-05) | A04, A14 | 2 | 4 | M | Retener (limitación aceptada) | **Aceptado, con mitigación no aplicada.** La API se publica en `0.0.0.0:8001` (`docker-compose.yml`), no en `127.0.0.1`. | Aceptado |
| R06 | Bypass de MFA (RT-06) | A01, A07 | 3 | 4 | **A** | Mitigar | **Mitigado en lo técnico (06/10).** TOTP con enroll solo por el propio usuario (403 para otro). Al confirmar un TOTP se envía aviso al titular y al RSI (`users.py`, `confirmar_totp`; `mailer.enviar_aviso_mfa_seguro`). Probado: el correo llegó a Mailpit (`evidencias/03-r06-r11.txt`). Pendiente: el phishing que obtenga el código en el momento sigue siendo posible sin factor resistente. | 06/10/2026 |
| R07 | XSS almacenado en el dashboard (RT-07) | A11 | 3 | 3 | M | Mitigar | **No tratado.** Sin CSP en el control central ni en Grafana. | 29/09/2026 (vencida) |
| R08 | Evasión de detección del SIEM (RT-08) | A08, A09 | 2 | 5 | M | Mitigar | **No tratado.** `local_rules.xml` tiene 5 reglas de maestra, borrado y cambio. Sin FIM ni alerta de agente desconectado. Wazuh manager no levantado. | 05/10/2026 (vencida) |
| R09 | Abuso de import/export (RT-09) | A02 | 2 | 4 | M | Mitigar | **Parcial.** AEAD XChaCha20-Poly1305 y cabecera `GEX1`. Los parámetros del KDF se leen del archivo sin límites mínimos ni máximos (`store.rs`, `importar`). | 02/10/2026 (vencida) |
| R10 | Exfiltración de secretos por errores de la app (RT-10) | A02, A05 | 3 | 5 | **A** | Mitigar | **Parcial.** Sin `traceback` ni modo debug en `control-central`. El JSONL no tiene secretos (verificado). No se hizo el barrido de patrones en logs antes de H4. | 05/10/2026 (vencida) |
| R11 | OSINT del repositorio (RT-11) | A13 | 3 | 4 | **A** | Mitigar | **Mitigado en lo técnico (06/10).** Configurados `.pre-commit-config.yaml` (hook de `gitleaks`) y `.gitleaks.toml`. Barrido del historial completo: 1 hallazgo, falso positivo (valor de prueba en `vault/store.rs`), excusado en la configuración. Pendiente: probar el hook en un commit real, porque requiere `pre-commit` instalado en cada equipo. Las imágenes tienen versión fijada, sin digest. | 06/10/2026 (verificar hook) |
| R12 | DoS puntual al control central (RT-12) | A04 | 3 | 2 | M | Retener (aceptar) | **Aceptado.** Sin límites de tasa en la API. El cliente opera offline (RNF-01). | Monitoreo continuo |
| R13 | PostgreSQL accesible desde la red del host (nuevo) | A05, A07 | 2 | 4 | M | Mitigar | **No tratado.** `docker-compose.yml` publica `5432:5432`. Falta confirmar si algún script o herramienta de la guía depende de ese puerto antes de quitarlo. | 07/10/2026 |

> **Corrección de la v1**: la v1 decía que había 4 riesgos Alto. La matriz tiene **5** (R01, R03, R06, R10 y R11).
> Los 5 deben tratarse antes del 07/10/2026 según el criterio de la sección 1. Ninguno está completo al 06/10.

## 5. Plan de tratamiento de riesgos

Acciones propuestas, ordenadas por prioridad. Las de los riesgos Alto son obligatorias antes del 07/10.

| ID Riesgo | Acción de tratamiento | Control/medida a implementar | Prioridad | Fecha objetivo |
|---|---|---|---|---|
| R11 | Escaneo de secretos antes de cada commit | Hook de pre-commit con `gitleaks`, y un barrido del historial actual | Alta | 07/10/2026 |
| R10 | Eliminar fugas por errores | Barrido de patrones de secretos en los logs y en el código; manejadores de error sin stack trace | Alta | 07/10/2026 |
| R06 | Avisar al titular de un enroll nuevo | Correo al RSI y al usuario cuando se confirma un TOTP (`users.py`, `confirmar`) | Alta | 07/10/2026 |
| R01 | Agregar fricción a la maestra | Medidor de fortaleza en la creación de la maestra (cliente) | Alta | 07/10/2026 |
| R03 | Endurecer la superficie de la API | Pruebas de IDOR sobre `/api/users`, `/api/incidents` y `/api/events`; DAST básico | Alta | 07/10/2026 |
| R13 | Dejar de exponer la base | Quitar la publicación de `5432` en `docker-compose.yml`; la base solo queda en la red de Docker | Media | 07/10/2026 |
| R05 | Limitar la API al equipo local | `ports: "127.0.0.1:8001:8000"`. Confirmar antes que los evaluadores accedan desde su propio equipo | Media | 07/10/2026 |
| R09 | Validar el KDF del archivo | Rechazar memoria, iteraciones o paralelismo fuera de rangos mínimos y máximos antes de derivar | Media | 07/10/2026 |
| R02 | Alerta por firma inválida repetida | Regla en `reglas.py` y en `local_rules.xml`: N firmas inválidas de un agente en una ventana | Media | 07/10/2026 |
| R07 | Sanitizar y limitar el render | CSP en Grafana y escape de los campos de evento antes de mostrarlos | Media | 07/10/2026 |
| R08 | Proteger la integridad del SIEM | FIM sobre la configuración de Wazuh; alerta de agente desconectado. Depende de levantar el manager | Media | Después del 07/10 |
| R04 | Configurar correctamente el correo | Desplegar Mailu con SPF, DKIM y DMARC, o justificar Mailpit ante la cátedra | Media | Consulta a la cátedra antes del 07/10 |
| R12 | Acotar el impacto de un DoS | Límites de tasa básicos en la API | Baja | Aceptado |

## 6. Riesgo residual y aceptación

| ID | Riesgo residual (post-tratamiento) | Aceptado por | Fecha |
|---|---|---|---|
| R01 | Un ataque dirigido con hardware dedicado sigue siendo posible si la maestra es débil; queda a criterio del usuario | RSI | Pendiente de firma |
| R02 | Compromiso total del endpoint del agente aún permitiría firmar eventos falsos de ese agente puntual | RSI | Pendiente de firma |
| R03 | Vulnerabilidades de día cero en dependencias de FastAPI/SQLAlchemy no cubiertas por SAST. V03 (starlette) abierta | RSI | Pendiente de firma |
| R04 | Ingeniería social directa al RSI (fuera del alcance técnico) | RSI | Pendiente de firma |
| R05 | Intercepción del tráfico HTTP en la red del laboratorio, sin TLS (limitación acordada con el docente) | RSI | Pendiente de firma |
| R06 | Phishing de TOTP al no haber factor resistente (WebAuthn/Hello fuera de alcance); robo físico del dispositivo con TOTP | RSI | Pendiente de firma |
| R07 | XSS reflejado no cubierto si se agregan nuevos campos sin sanitizar | RSI | Pendiente de firma |
| R08 | Un insider con acceso root al manager de Wazuh igual podría desactivarlo | RSI | Pendiente de firma |
| R09 | Ataques de fuerza bruta contra la contraseña de transporte si es débil | RSI | Pendiente de firma |
| R10 | Fugas por canales no contemplados (ej. volcado de memoria) quedan fuera de alcance de esta tarea | RSI | Pendiente de firma |
| R11 | Historial de Git anterior a la adopción del hook no queda cubierto retroactivamente | RSI | Pendiente de firma |
| R12 | Un DoS sostenido y distribuido sigue afectando la disponibilidad del control central (RNF-03) | RSI | Aceptado, sin tratamiento más allá del límite de tasa básico |
| R13 | Si la base queda publicada hasta el 07/10, un acceso directo al host expone los datos. Se acepta solo hasta esa fecha | RSI | Pendiente de firma |

---

## Guía de llenado (Blue Team)

| Fase | Cómo completar | Con qué evidencia |
|---|---|---|
| Diseño | Riesgos derivados de RT-01..RT-12 sobre la arquitectura y el inventario de activos | v1, 22/09 |
| Implementación (H2-H3) | Verificar cada control implementado realmente, no solo declarado | Este documento, sección 4 (estado verificado), y `docs/evidencias/` |
| Pre-Red Team (H4) | Confirmar qué riesgos Alto y Crítico quedaron tratados antes de congelar | Sección 4 y sección 5, con cierre de cada acción |
| Red Team | Usa esta matriz para priorizar vectores de ataque | Informe técnico Red Team |

## Check de aceptación

- [x] Activos críticos con amenazas identificadas (sección 2, ligadas a RT-01..RT-12, más R13).
- [x] Riesgos evaluados con probabilidad, impacto y nivel (sección 4).
- [x] Estado verificado de cada control al 06/10 (sección 4).
- [x] Plan de tratamiento con responsables y fechas (sección 5).
- [ ] Los 5 riesgos Alto tratados antes del 07/10 (R01, R03, R06, R10 y R11 hoy parciales o no tratados).
- [ ] Riesgo residual aceptado formalmente por el RSI (hoy son fechas objetivo).

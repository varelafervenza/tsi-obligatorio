# Análisis y Tratamiento de Riesgos — Gestor de Contraseñas con Control Centralizado

> Basado en `plantilla/isaca/03-analisis-riesgos.md`. Los riesgos se derivaron directamente de
> los **objetivos del Red Team (RT-01 a RT-12, sección 7.2 de `LETRA.md`)**: es la misma lista de
> ataques que el Red Team va a intentar entre el 28/10 y el 09/11, así que conviene que el Blue
> Team la trate como su propio catálogo de amenazas en vez de inventar uno paralelo.

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
| Versión | 0.1 (borrador — v1 prevista para hito H1/H2; v2 el 06/10/2026 según sección 8.1 de `LETRA.md`) |
| Metodología | ISO 31000 + COBIT APO12 + MCU 5.0, catálogo de amenazas = objetivos RT-01..RT-12 |
| Fecha | 22/09/2026 |

### Historial de versiones

| Versión | Fecha | Autor | Cambios |
|---|---|---|---|
| 0.1 | 22/09/2026 | Blue Team | Primera versión: 12 riesgos derivados de RT-01..RT-12, sobre el inventario `02-Registro-Activos.md`. |

## 1. Contexto

- **Organización**: Laboratorio del curso Seguridad de la Información (equipo Blue Team).
- **Solución analizada**: Gestor de Contraseñas con Control Centralizado (Tarea 1) — alcance
  completo: `cliente-gestor`, `control-central`, Wazuh, Mailu, Grafana, repositorio Git.
- **Criterios de riesgo aceptados por la organización**: los riesgos de nivel **Bajo** se aceptan
  sin tratamiento adicional; los de nivel **Medio** se mitigan si el costo/tiempo lo permite antes
  del 07/10/2026; los de nivel **Alto** y **Crítico** deben tratarse obligatoriamente antes de la
  pre-entrega (H4, 07/10/2026), porque son exactamente lo que el Red Team va a intentar explotar.
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
| A11 | XSS almacenado en el dashboard (RT-07) | Externo, deliberado | Campos de evento (sistema/notas) sin sanitizar al renderizar |
| A08, A09 | Evasión de la detección del SIEM (RT-08) | Interno/externo, deliberado | Reglas/agentes sin protección de integridad ni alerta de desconexión |
| A02 | Abuso de import/export para degradar cifrado (RT-09) | Externo, deliberado | Formato de exportación sin verificación de algoritmo/versión |
| A02, A05 | Exfiltración de secretos por errores de la app (RT-10) | Interno, accidental | Logs de depuración o stack traces expuestos en producción |
| A13 | OSINT del repositorio: secretos commiteados, Dockerfiles vulnerables (RT-11) | Externo, deliberado | Falta de escaneo de secretos previo al push |
| A04 | Denegación de servicio puntual al control central (RT-12) | Externo, deliberado | Ausencia de límites de tasa en la API |

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

| ID | Riesgo | Activos afectados | Prob. | Impacto | Nivel | Tratamiento | Responsable | Fecha objetivo |
|---|---|---|---|---|---|---|---|---|
| R01 | Fuerza bruta / crackeo de la maestra (RT-01) | A01, A02 | 3 | 5 | **A** | Mitigar | Blue Team (cliente-gestor) | 02/10/2026 |
| R02 | Forjado de eventos de auditoría (RT-02) | A03, A04, A06 | 2 | 4 | M | Mitigar | Blue Team (control-central) | 29/09/2026 |
| R03 | IDOR / XSS / SQLi en el control central (RT-03) | A04, A05, A07 | 3 | 5 | **A** | Mitigar | Blue Team (control-central) | 05/10/2026 |
| R04 | Suplantación del servidor de correo (RT-04) | A10 | 3 | 3 | M | Mitigar | Blue Team (infra/mailu) | 29/09/2026 |
| R05 | MITM por TLS mal configurado (RT-05) | A04, A14 | 2 | 4 | M | Mitigar | Blue Team | 02/10/2026 |
| R06 | Bypass de MFA (RT-06) | A01, A07 | 3 | 4 | **A** | Mitigar | Blue Team | 02/10/2026 |
| R07 | XSS almacenado en el dashboard (RT-07) | A11 | 3 | 3 | M | Mitigar | Blue Team | 29/09/2026 |
| R08 | Evasión de detección del SIEM (RT-08) | A08, A09 | 2 | 5 | M | Mitigar | Blue Team | 05/10/2026 |
| R09 | Abuso de import/export (RT-09) | A02 | 2 | 4 | M | Mitigar | Blue Team (cliente-gestor) | 02/10/2026 |
| R10 | Exfiltración de secretos por errores de la app (RT-10) | A02, A05 | 3 | 5 | **A** | Mitigar | Blue Team | 05/10/2026 |
| R11 | OSINT del repositorio (RT-11) | A13 | 3 | 4 | **A** | Mitigar | Blue Team | Continuo (cierre 07/10/2026) |
| R12 | DoS puntual al control central (RT-12) | A04 | 3 | 2 | M | Retener (aceptar) | RSI | Monitoreo continuo |

> Los 4 riesgos marcados **A** (Alto) son los de tratamiento obligatorio antes del 07/10/2026 según
> el criterio de aceptación de la sección 1.

## 5. Plan de tratamiento de riesgos

| ID Riesgo | Acción de tratamiento | Control/medida a implementar | Prioridad | Fecha objetivo |
|---|---|---|---|---|
| R01 | Endurecer KDF y agregar fricción a intentos fallidos | Argon2id con parámetros altos (RF-16) + delay adaptativo + medidor de fortaleza de la maestra | Alta | 02/10/2026 |
| R02 | Aislar la confianza por agente | Clave privada JWS única por agente (no HMAC compartida); alerta en `control-central` ante firma inválida repetida | Media | 29/09/2026 |
| R03 | Endurecer la superficie de la API | ORM con queries parametrizadas (SQLAlchemy), validación Pydantic, pruebas SAST/DAST antes de H4 | Alta | 05/10/2026 |
| R04 | Configurar correctamente el correo propio | SPF + DKIM + DMARC, sin relay abierto, TLS obligatorio en submission (587) | Media | 29/09/2026 |
| R05 | Forzar validación de certificados | TLS 1.2+ obligatorio; CA propia del laboratorio o pinning en el cliente; rechazar certificados inválidos | Media | 02/10/2026 |
| R06 | Preferir factores resistentes a phishing | WebAuthn/Windows Hello como opción preferida sobre TOTP; notificación por correo ante un nuevo enroll | Alta | 02/10/2026 |
| R07 | Sanitizar toda entrada renderizada | Escapar/sanitizar campos de evento antes de mostrarlos; CSP estricta en el dashboard | Media | 29/09/2026 |
| R08 | Proteger la integridad del SIEM | FIM sobre configuración de Wazuh; alerta de "agente desconectado" o "regla modificada"; acceso restringido al manager | Media | 05/10/2026 |
| R09 | Fijar el formato de exportación | Formato de exportación sin downgrade de algoritmo; KDF propia para la contraseña de transporte; verificación AEAD al importar | Media | 02/10/2026 |
| R10 | Eliminar fugas por errores | Deshabilitar modo debug en builds de entrega; manejadores de error genéricos sin stack trace; grep de patrones de secretos en logs antes de H4 | Alta | 05/10/2026 |
| R11 | Evitar secretos commiteados | Hook de pre-commit con escaneo de secretos (ej. gitleaks) antes de cada push; imágenes base de Docker fijadas por versión | Alta | Continuo |
| R12 | Acotar el impacto de un DoS | Límites de tasa básicos en la API; el cliente sigue operando offline (RNF-01) mientras el control central se recupera | Baja | Monitoreo continuo |

## 6. Riesgo residual y aceptación

| ID | Riesgo residual (post-tratamiento) | Aceptado por | Fecha |
|---|---|---|---|
| R01 | Un ataque dirigido con hardware dedicado sigue siendo posible si la maestra es débil; queda a criterio del usuario | RSI | 07/10/2026 |
| R02 | Compromiso total del endpoint del agente aún permitiría firmar eventos falsos de ese agente puntual | RSI | 07/10/2026 |
| R03 | Vulnerabilidades de día cero en dependencias de FastAPI/SQLAlchemy no cubiertas por SAST | RSI | 07/10/2026 |
| R04 | Ingeniería social directa al RSI (fuera del alcance técnico) | RSI | 07/10/2026 |
| R05 | Compromiso del endpoint cliente con una CA maliciosa instalada localmente | RSI | 07/10/2026 |
| R06 | Robo físico del segundo factor (llave FIDO2/dispositivo) | RSI | 07/10/2026 |
| R07 | XSS reflejado no cubierto si se agregan nuevos campos sin sanitizar | RSI | 07/10/2026 |
| R08 | Un insider con acceso root al manager de Wazuh igual podría desactivarlo | RSI | 07/10/2026 |
| R09 | Ataques de fuerza bruta contra la contraseña de transporte si es débil | RSI | 07/10/2026 |
| R10 | Fugas por canales no contemplados (ej. volcado de memoria) quedan fuera de alcance de esta tarea | RSI | 07/10/2026 |
| R11 | Historial de Git anterior a la adopción del hook no queda cubierto retroactivamente | RSI | 07/10/2026 |
| R12 | Un DoS sostenido y distribuido sigue afectando la disponibilidad del control central (RNF-03) | RSI | Aceptado — no se trata más allá del rate limiting básico |

---

## Guía de llenado (Blue Team)

| Fase | Cómo completar | Con qué evidencia |
|---|---|---|
| Diseño (actual) | Riesgos preliminares derivados de RT-01..RT-12 sobre la arquitectura y el inventario de activos | Este documento |
| Implementación (H2-H3) | Verificar cada control implementado realmente (no solo declarado) | Configs, pruebas, capturas en `docs/evidencias/` |
| Pre-Red Team (H4) | Confirmar qué riesgos Alto/Crítico quedaron tratados antes de congelar | Checklist de esta sección 4 |
| Red Team | Usa esta matriz para priorizar vectores de ataque (es literalmente su lista RT-01..RT-12) | Informe técnico Red Team |

## Check de aceptación

- [x] Activos críticos con amenazas identificadas (sección 2, ligadas a RT-01..RT-12).
- [x] Riesgos evaluados con probabilidad/impacto y nivel (sección 4).
- [x] Plan de tratamiento con responsables y fechas (sección 5).
- [ ] Riesgo residual aceptado **formalmente** por el RSI (falta firma real; hoy son fechas objetivo).
- [ ] v1 formal vencía el 18/09/2026 (ya pasada) — dejar registrado en la bitácora y priorizar tratar los 4 riesgos **Alto** antes del 07/10.

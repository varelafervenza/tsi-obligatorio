# BCU — Guía de Seguridad de la Información y Requerimientos Mínimos para el Sistema Financiero

Documento de referencia del **Banco Central del Uruguay (BCU)** para las tres tareas.

---

## 1. Marco normativo para el sistema financiero

El BCU fue elevando estándares de ciberseguridad para el sistema financiero. Las instituciones supervisadas (bancos, instituciones de intermediación financiera, emisores de dinero electrónico, etc.) deben:

1. Documentar y aprobar por la Alta Dirección una **Gestión de Seguridad de la Información**.
2. Adoptar prácticas de referencia internacional:
   - **ISO/IEC 27001:2013/2022** (SGSI).
   - **COBIT 5/2019 para Seguridad de la Información** (gobernanza y gestión de TI).
   - **NIST SP 800-53** (catálogo de controles).
3. Reportar periódicamente el **nivel de madurez y cumplimiento del MCU 5.0 (AGESIC)**.

## 2. Comunicación BCU 2026/098 (mayo 2026) — Cumplimiento MCU 5.0

Aplicable a **instituciones emisoras de dinero electrónico** (y extendido progresivamente a otros actores del sistema financiero):

| Campo | Detalle |
|---|---|
| Objeto | Reporte trimestral de nivel de madurez y cumplimiento del **MCU 5.0** y sistema de pagos |
| Formato | XML, Anexos 1 a 5 de la Comunicación 2026/098 |
| Tipo de dato | **957 - Cumplimiento MCU 5.0 - Nivel de Madurez** |
| Frecuencia | **Trimestral** |
| Plazo | 7 días hábiles del mes siguiente al cierre de cada trimestre |
| Canal | Portal IDI (Sistema de Envío Centralizado del BCU) |
| Vigencia | Desde junio de 2026 |

> Para el curso: los equipos deben preparar el **conjunto de evidencia y métricas** que permitiría a la organización completar este reporte, aunque no lo envíen realmente.

## 3. Requerimientos mínimos de seguridad de la información (síntesis de la GSI BCU)

| # | Requerimiento | Descripción |
|---|---|---|
| 1 | Política formal de seguridad | Aprobada por la dirección, comunicada, revisada. |
| 2 | Responsable de Seguridad de la Información (RSI) | Rol designado con autoridad y recursos. |
| 3 | Análisis de riesgos periódico | Metodología ISO 31000, documentado y revisado por dirección. |
| 4 | Gestión de accesos y privilegios | Mínimo privilegio; revisión periódica. |
| 5 | **Autenticación de dos factores** | Obligatoria para transferencias, pagos a terceros y operaciones de alto valor. |
| 6 | Controles criptográficos | Uso de criptografía robusta y gestión de claves. |
| 7 | **Backups diarios** | Copia diaria de los datos críticos. |
| 8 | **Copia fuera del centro de procesamiento** | Respaldo en ubicación física separada del sitio principal. |
| 9 | **Restauración validada periódicamente** | Pruebas de restauración documentadas. |
| 10 | **Monitoreo de cambios en datos sensibles** | Registro y control de cambios en identificación, dirección, teléfono y correo de clientes. |
| 11 | Sistema de detección y prevención de intrusiones | IDS/IPS y monitoreo de eventos. |
| 12 | Gestión de vulnerabilidades y parches | Escaneos periódicos, remediación con plazos. |
| 13 | Gestión de incidentes y notificación | Procedimiento, escalamiento y **notificación de incidentes relevantes al BCU**. |
| 14 | Plan de continuidad | BCP/DRP probado; RTO/RPO definidos. |
| 15 | Capacitación y concientización | Programa continuo para el personal. |
| 16 | Logs y retención | Registro de eventos con retención definida. |
| 17 | Gestión de proveedores y terceros | Evaluar la seguridad de terceros (incluidos SaaS). |
| 18 | Reporte de madurez MCU 5.0 | Trimestral según Comunicación vigente. |

## 4. Sanciones

El incumplimiento continuado puede derivar en **sanciones económicas** (caso de referencia: multa al Banco Hipotecario del Uruguay en 2022 por incumplimiento sostenido de requerimientos de protección).

## 5. Cómo se vincula con cada tarea del curso

| Tarea | Requerimientos BCU más relevantes |
|---|---|
| Tarea 1 — Gestor de contraseñas | 2FA (5), criptografía (6), monitoreo de cambios (10), logs (16), incidentes (13). |
| Tarea 2 — Plataforma de gestión del RSI | Política (1), RSI (2), riesgos (3), accesos (4), madurez MCU 5.0 (18). |
| Tarea 3 — Infraestructura de red | IDS/IPS (11), SIEM/logs (16), vulnerabilidades (12), backups (7-9), continuidad (14). |

## 6. Fuentes oficiales

- BCU — Comunicación Nº 2026/098: `https://www.bcu.gub.uy/Comunicados/seggco26098.pdf`
- BCU — Comunicados sobre estándares de ciberseguridad: `https://www.bcu.gub.uy/comunicaciones`
- AGESIC — MCU 5.0: `https://www.gub.uy/agencia-gobierno-electronico-sociedad-informacion-conocimiento/comunicacion/publicaciones/marco-ciberseguridad-50`

> **Nota**: la GSI completa es aplicable a entidades supervisadas; en el marco educativo se utiliza como referencia de buenas prácticas de un marco regulado y supervisado.
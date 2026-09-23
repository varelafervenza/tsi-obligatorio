# Política de Seguridad de la Información — Gestor de Contraseñas con Control Centralizado

> Basado en `plantilla/isaca/01-politica-seguridad.md`. Documento de **Gobernar** (MCU 5.0):
> fija los principios y roles que el resto de la documentación (accesos, monitoreo, incidentes,
> continuidad) debe respetar. Se redacta después de tener la arquitectura (`docs/00-*`), el
> inventario de activos (`docs/02-Registro-Activos.md`) y el análisis de riesgos
> (`docs/03-Analisis-Riesgos.md`) ya definidos, para que sea coherente con lo que realmente se
> construyó y no una declaración genérica.

---

## Encabezado de mapeo normativo

| Marco | Ítem | Detalle / aporte |
|---|---|---|
| **MCU 5.0 (función)** | Gobernar | GV-01 (política aprobada), GV-02 (roles y responsabilidades). |
| **Perfil comunitario** | **Avanzado** (obligatorio para las 3 tareas del curso, sección 0.1 de `LETRA.md`) | |
| **COBIT 2019** | APO13, APO01, EDM03 | La política es el insumo controlador de APO13. |
| **ISO/IEC 27001:2022** | A.5.1, A.5.9, A.5.24 | Políticas, inventario, incidentes. |
| **BCU — GSI** | Requisito 1 (política formal) | Base para el resto de los requisitos BCU (2FA, monitoreo, notificación). |
| **URCDP — Ley 18.331** | Art. 9 y 12 | Medidas de seguridad para los datos personales tratados (ver `03-Analisis-Riesgos.md`, activos con dato personal = S). |

---

## Control del documento

| Campo | Valor |
|---|---|
| Nombre del documento | Política de Seguridad de la Información — Gestor de Contraseñas |
| Código | SI-POL-01 |
| Versión | 0.1 (borrador) |
| Fecha de aprobación | Pendiente — a formalizar en la auditoría del 14/10/2026 |
| Aprobado por | Pendiente (RSI del equipo / docente en la auditoría) |
| Autor | Blue Team (Horacio Duarte, Pablo Morales, Andrés Varela) |
| Próxima revisión | 07/10/2026 (pre-entrega) y luego ante cualquier cambio significativo de arquitectura |

### Historial de versiones

| Versión | Fecha | Autor | Descripción de cambios |
|---|---|---|---|
| 0.1 | 22/09/2026 | Blue Team | Primera versión, alineada a la arquitectura, activos y riesgos ya documentados. |

## 1. Objetivo

Establecer el marco directivo que fija los principios, responsabilidades y reglas para proteger
la confidencialidad, integridad y disponibilidad del **Gestor de Contraseñas con Control
Centralizado**, de forma que cada decisión técnica (cripto, accesos, monitoreo, continuidad)
pueda justificarse contra esta política y no quede como una elección aislada.

## 2. Alcance

Aplica a todos los componentes de la solución y a quienes interactúan con ellos:

- **`cliente-gestor`**: la aplicación de escritorio y la bóveda cifrada local de cada usuario.
- **`control-central`**: la API, la base de datos de eventos/usuarios y el dashboard.
- **Infraestructura de soporte**: Wazuh (SIEM/HIDS), Mailu (correo), Grafana, y el mecanismo de
  gestión de incidentes (TheHive o tabla interna, según se resuelva).
- **El repositorio de código** (`github.com/varelafervenza/tsi-obligatorio`) y su historial.
- **Personas**: usuario final, RSI, administrador de la plataforma, equipo Blue Team y, en la
  Parte 2, el equipo Red Team (como evaluador externo autorizado, no como usuario del sistema).

Fuera de alcance: cualquier sistema del usuario ajeno al gestor (SO, otras apps), y cualquier
ataque fuera de las VMs del laboratorio del curso (ver reglas de compromiso, sección 7.1 de
`LETRA.md`).

## 3. Marco normativo y su posición

- **MCU 5.0**: esta política es el artefacto de la función **Gobernar**; las demás funciones
  (Identificar, Proteger, Detectar, Responder, Recuperar) se documentan en los archivos
  numerados de esta misma carpeta y se auditan contra el Excel
  `plantilla/mcu5/excel/01-controles-mcu5-perfil-avanzado.xlsx` en **perfil Avanzado**.
- **BCU — GSI**: esta política cubre el "Requisito 1"; los requisitos de 2FA, monitoreo y
  notificación de incidentes de la GSI se resuelven en `09-Gestion-Accesos.md`,
  `07-Monitoreo-Logs-SIEM.md` y `12-Notificacion-Incidentes.md` respectivamente.
- **ISO/IEC 27001:2022 y Ley 18.331**: los activos que tratan datos personales (A02, A05, A07,
  A10 — ver `02-Registro-Activos.md`) están sujetos además a las medidas de seguridad del Art. 9
  y 12 de la Ley 18.331.

## 4. Principios de seguridad

| Principio | Cómo se aplica en esta solución |
|---|---|
| Confidencialidad | Zero-knowledge: la bóveda se cifra y descifra solo en `cliente-gestor`; `control-central` nunca recibe ni puede reconstruir una contraseña, solo metadata firmada (RF-08). |
| Integridad | Cifrado AEAD (XChaCha20-Poly1305) en la bóveda y firma JWS por agente en cada evento de auditoría; cualquier alteración se detecta antes de aceptarse. |
| Disponibilidad | El gestor funciona 100% offline (RNF-01); la caída del control central no interrumpe el uso diario, solo retrasa la sincronización de eventos. |
| Mínimo privilegio | RBAC en `control-central` (roles por usuario del panel); las claves de firma son una por agente, nunca compartidas entre clientes. |
| Defensa en profundidad / Zero Trust | Cada evento se verifica como si viniera de un origen hostil (NIST SP 800-207); MFA tanto en el cliente (desbloqueo de bóveda) como en el panel del control central. |

## 5. Estructura de la política

Jerarquía documental: esta política ⮕ políticas específicas de esta misma carpeta ⮕
configuración real desplegada (`infra/`, `control-central/`, `cliente-gestor/`).

| Política específica | Contenido mínimo | Referencia | Estado |
|---|---|---|---|
| Política de control de accesos | MFA local (TOTP/WebAuthn/Windows Hello), RBAC del panel, elección de Argon2id/bcrypt | `09-Gestion-Accesos.md` | Pendiente |
| Política de gestión de contraseñas/vulnerabilidades | Reglas de generación (RF-04/RF-05), rotación, escaneos | `10-Gestion-Vulnerabilidades.md` | Pendiente |
| Política de gestión de incidentes | Detección, reporte, escalamiento, estados | `04-Gestion-Incidentes.md` | Pendiente |
| Política de monitoreo y logs | Eventos, reglas SIEM, retención ≥ 90 días (RNF-07) | `07-Monitoreo-Logs-SIEM.md` | Pendiente |
| Política de copias de seguridad | Exportación de bóvedas, prueba de restauración | `06-Plan-Continuidad.md` | Pendiente |
| Inventario y riesgos (insumo, no política) | Activos y amenazas que justifican los controles anteriores | `02-Registro-Activos.md`, `03-Analisis-Riesgos.md` | Hecho |

## 6. Roles y responsabilidades

| Rol | Responsabilidades |
|---|---|
| RSI (del equipo, rotativo o fijo) | Aprobar esta política y sus revisiones; aceptar formalmente el riesgo residual (`03-Analisis-Riesgos.md`, sección 6); ser el destinatario de las notificaciones de auditoría. |
| Administrador de la plataforma | Mantener `infra/` (Docker Compose, Wazuh, Mailu, Grafana) desplegada y actualizada; ejecutar y probar las copias de seguridad. |
| Usuario final | Elegir una contraseña maestra robusta; habilitar MFA local; no compartir ni exportar la bóveda fuera de los mecanismos previstos (RF-12). |
| Equipo de desarrollo (Blue Team) | Implementar los RF/RNF de `LETRA.md`, mantener la bitácora al día, y poder demostrar en vivo cualquier control declarado (condición de aprobación de la auditoría, sección 6.6). |
| Equipo de validación (Red Team) | Evaluar la solución congelada sin modificarla; documentar hallazgos según `plantilla/informe-red-team.md`, dentro de la ventana 28/10-09/11/2026. |

## 7. Concientización y cumplimiento

- **Capacitación**: inducción del equipo al inicio del proyecto (ya cubierta por el análisis de
  `LETRA.md` y la definición de arquitectura); repaso antes de cada hito (H2, H3, H4).
- **Consecuencias por incumplimiento**: en este contexto académico, un control declarado y no
  demostrable en la auditoría es un **hallazgo de auditoría** que descuenta puntaje (sección 9 de
  `LETRA.md`); un incumplimiento grave (ej. secretos commiteados) se trata como incidente propio
  (`04-Gestion-Incidentes.md`).
- **Canal de reporte**: la bitácora diaria (`99-bitacora-trabajo.md`) es el canal formal para
  reportar fallos, hallazgos internos y desvíos del cronograma — no se omiten fallos, un error
  registrado y corregido suma en la auditoría.

## 8. Vigencia y revisión

- Esta política entra en vigencia a partir del 22/09/2026 (fecha de este borrador).
- Se revisa formalmente en cada hito (H2 29/09, H3 02/10, H4 07/10) y se congela junto con el
  resto de la documentación en la pre-entrega (`git tag v1.0`).
- Fuera del curso, la vigencia esperada sería anual o ante cambios significativos; para esta
  tarea, el cierre natural es el 09/11/2026 (fin de la Parte 2 / Red Team).

---

## Guía de llenado (para el curso)

| Fase | Cómo completar | Con qué evidencia |
|---|---|---|
| Blue Team - Inicio | Redactada sobre la arquitectura, activos y riesgos ya definidos (no al revés) | `docs/00-*`, `02-Registro-Activos.md`, `03-Analisis-Riesgos.md` |
| Blue Team - Implementación | Ajustar la sección 5 a medida que cada política específica deje de estar "Pendiente" | Los propios documentos referenciados |
| Entrega a Red Team | Congelar la versión firmada | `git tag v1.0` + hash SHA-256 |
| Red Team - Evaluación | No modifica esta política; la evalúa contra sus hallazgos técnicos | Informe de hallazgos del Red Team |

## Check de aceptación

- [ ] Aprobada formalmente con fecha y firmas (falta: hoy es borrador sin aprobación real).
- [x] Menciona MCU 5.0, BCU, ISO y Ley 18.331.
- [x] Define roles y responsabilidades.
- [x] Referencia el resto de las políticas de la carpeta `docs/` (todas, con su estado real: la mayoría **Pendiente**).

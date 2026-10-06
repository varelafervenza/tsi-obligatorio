# Declaración de Aplicabilidad (SoA) y Plan de Tratamiento — Gestor de Contraseñas con Control Centralizado

> Basado en `plantilla/isaca/11-soa-plan-tratamiento.md`. Cubre los **93 controles** del Anexo A de
> ISO/IEC 27001:2022. Función **Gobernar + Proteger** del MCU 5.0.
>
> **Corrección a la plantilla:** la plantilla indica 37/4/5/33 controles y dice "79". El Anexo A de la norma
> tiene 37 controles organizacionales (5.1 a 5.37), 8 de personas (6.1 a 6.8), 14 físicos (7.1 a 7.14) y
> 34 tecnológicos (8.1 a 8.34): 93 en total.
>
> **Estados:** *Implementado* (funciona y tiene evidencia), *Parcial* (existe algo, pero le falta una parte),
> *Pendiente* (aplica y no está hecho), *N.A.* (no aplica, con justificación).

---

## Control del documento

| Campo | Valor |
|---|---|
| Código | SI-SOA-11 |
| Versión | 0.1 (borrador) |
| Responsable | Blue Team (Horacio Duarte, Pablo Morales, Andrés Varela) |
| Metodología | ISO/IEC 27001:2022 (Anexo A) + MCU 5.0 |
| Fecha | 06/10/2026 |

---

## 1. Alcance de la declaración

- **Sistema:** Gestor de Contraseñas con Control Centralizado (Tarea 1).
- **Componentes en alcance:** `cliente-gestor` (aplicación de escritorio y bóveda local), `control-central`
  (API, PostgreSQL, Grafana), Mailpit (correo de desarrollo), scripts de backup, repositorio Git y la
  documentación de `docs/`.
- **Fuera de alcance:** instalaciones físicas (la solución corre en el laboratorio del curso y en los equipos
  de los integrantes), Wazuh y Mailu en producción (todavía no desplegados), y la organización que
  contrata a los integrantes (es un proyecto académico).
- **Criterio de aplicabilidad:** se marca N.A. sólo lo que no existe en el contexto del proyecto (instalaciones
  físicas propias, contratación de personal, proveedores con contrato). Todo lo técnico aplica, aunque esté
  pendiente.

---

## 2. Resumen de estado

| Categoría | Total | Aplicables (Sí) | N.A. (justificados) | Implementado | Parcial | Pendiente |
|---|---|---|---|---|---|---|
| A.5 Organizacionales | 37 | 34 | 3 | 8 | 20 | 6 |
| A.6 Personas | 8 | 4 | 4 | 0 | 3 | 1 |
| A.7 Físicos | 14 | 3 | 11 | 0 | 1 | 2 |
| A.8 Tecnológicos | 34 | 32 | 2 | 4 | 24 | 4 |
| **Total** | **93** | **73** | **20** | **12** | **48** | **13** |

> Los conteos se calcularon desde la tabla de la sección 3: los implementados, parciales y pendientes suman
> los 73 controles que aplican.

---

## 3. Declaración de aplicabilidad

| ID | Control | ¿Aplica? | Justificación | Insumos (docs y evidencias) | Estado |
|---|---|---|---|---|---|
| 5.1 | Políticas de seguridad de la información | Sí | Hay un borrador de política (v0.2). Falta la aprobación y firma del RSI. | `01-Politica-Seguridad.md` | Parcial |
| 5.2 | Roles y responsabilidades de seguridad | Sí | RACI con roles por componente. | `03-matriz-raci-mcu5.xlsx` | Implementado |
| 5.3 | Segregación de funciones | Sí | Con tres integrantes la separación es parcial: quien implementa no es quien aprueba el riesgo residual. | RACI, `03-Analisis-Riesgos.md` | Parcial |
| 5.4 | Responsabilidades de la dirección | Sí | El RSI (Andrés Varela) fija políticas y acepta riesgos; no hay comité. | `01-Politica-Seguridad.md` | Parcial |
| 5.5 | Contacto con autoridades | Sí | Notificación a BCU y URCDP definida, pero el borrador simulado está pendiente. | `04-Gestion-Incidentes.md`; `12-...` pendiente | Pendiente |
| 5.6 | Contacto con grupos de interés especiales | N.A. | No hay grupos formales asociados al proyecto; la fuente de referencia es la cátedra. | — | N.A. |
| 5.7 | Inteligencia de amenazas | Sí | Los riesgos se derivan de los objetivos del Red Team (RT-01 a RT-12), no de un feed de inteligencia. | `03-Analisis-Riesgos.md` | Parcial |
| 5.8 | Seguridad en la gestión de proyectos | Sí | Riesgos y decisiones registrados en la bitácora y en la política. | `99-bitacora-trabajo.md`, `03-Analisis-Riesgos.md` | Parcial |
| 5.9 | Inventario de información y activos | Sí | Catorce activos con dueño y clasificación. | `02-Registro-Activos.md`, `02-registro-activos-mcu5.xlsx` | Implementado |
| 5.10 | Uso aceptable de activos | Sí | No hay una política escrita de uso aceptable. Las reglas de secretos están en `01` y `09`. | `09-Gestion-Accesos.md` (sección 4) | Pendiente |
| 5.11 | Devolución de activos | N.A. | No hay personal con activos asignados; el proyecto es académico. | — | N.A. |
| 5.12 | Clasificación de la información | Sí | Cuatro niveles (público, interno, confidencial, secreto) aplicados a los activos. | `02-Registro-Activos.md` | Implementado |
| 5.13 | Etiquetado de la información | Sí | La clasificación vive en el registro de activos; no hay etiquetado dentro de los archivos. | `02-Registro-Activos.md` | Pendiente |
| 5.14 | Transferencia de información | Sí | Eventos firmados (RS256); el canal cliente → API va sin TLS (limitación acordada). | `00-arquitectura-c4.md`, `09-...` | Parcial |
| 5.15 | Control de acceso | Sí | Bóveda protegida por maestra y TOTP; panel con roles y token. | `09-Gestion-Accesos.md`, `guia-de-pruebas.md` (B-14, C-09) | Parcial |
| 5.16 | Gestión de identidades | Sí | Usuarios del panel con alta; la baja no está automatizada. | `09-Gestion-Accesos.md` (sección 2) | Parcial |
| 5.17 | Información de autenticación | Sí | Maestra con Argon2id, TOTP con secreto cifrado, hash del panel. | `09-Gestion-Accesos.md`, `kdf.rs` | Implementado |
| 5.18 | Derechos de acceso | Sí | Roles admin, auditor y rsi en el panel. No hay revisión periódica registrada. | `09-Gestion-Accesos.md` | Parcial |
| 5.19 | Seguridad en relación con proveedores | Sí | Dependencias de código abierto identificadas en el registro de activos. | `02-Registro-Activos.md` (A13 y dependencias) | Parcial |
| 5.20 | Seguridad en acuerdos con proveedores | N.A. | El software es de código abierto sin contrato. | — | N.A. |
| 5.21 | Seguridad en la cadena de suministro TIC | Sí | Versiones fijadas en `requirements.txt`, `package-lock.json` y `Cargo.lock`; `pip-audit` y `npm audit`. | `10-Gestion-Vulnerabilidades.md` | Parcial |
| 5.22 | Monitoreo de servicios de proveedores | Sí | No hay monitoreo periódico de avisos; los escaneos se corrieron una vez. | `10-...` | Pendiente |
| 5.23 | Seguridad en servicios en la nube | Sí | El repositorio está en GitHub. No se guardan datos personales ni secretos allí (`.gitignore`). | `.gitignore`, `06-Plan-Continuidad.md` | Parcial |
| 5.24 | Planificación de la gestión de incidentes | Sí | Procedimiento de incidentes con severidades y estados. | `04-Gestion-Incidentes.md` | Implementado |
| 5.25 | Evaluación de eventos de seguridad | Sí | Umbrales de las reglas (5 intentos en 2 min) y alertas en la API. | `07-Monitoreo-Logs-SIEM.md`, `07-simulacion-casos-uso.txt` | Implementado |
| 5.26 | Respuesta a incidentes | Sí | Incidente 1 creado desde la alerta 1 y resuelto (datos de prueba). | `06-restauracion-bd-con-datos.txt`, `04-Gestion-Incidentes.md` | Implementado |
| 5.27 | Aprendizaje de los incidentes | Sí | La plantilla de lecciones existe, pero no hay lecciones redactadas a partir de un incidente real. | `04-Gestion-Incidentes.md` (sección 5) | Parcial |
| 5.28 | Recolección de evidencias | Sí | Evidencias en `docs/evidencias/` con hash para backups y logs. | `docs/evidencias/`, `06-restauracion-bd.txt` | Parcial |
| 5.29 | Seguridad de la información durante una interrupción | Sí | Procedimientos de recuperación por escenario. | `06-Plan-Continuidad.md` (sección 4) | Parcial |
| 5.30 | Preparación de TIC para la continuidad | Sí | Restauración de la base probada. Sin copia fuera del equipo. | `06-Plan-Continuidad.md` (sección 3) | Parcial |
| 5.31 | Requisitos legales y contractuales | Sí | Ley 18.331, GSI del BCU y MCU 5.0 identificados en la política. | `01-Politica-Seguridad.md` (sección 3) | Parcial |
| 5.32 | Derechos de propiedad intelectual | Sí | Dependencias de código abierto; falta revisar las licencias de cada una. | `10-...`, `LETRA.md` (RNF-09) | Pendiente |
| 5.33 | Protección de registros | Sí | Bitácora, JSONL y backups con hash. Sin firma digital de los registros. | `99-bitacora-trabajo.md`, `06-...` | Parcial |
| 5.34 | Privacidad y protección de datos personales | Sí | Activos con datos personales identificados (A02, A05, A07, A10). No hay evaluación formal de impacto. | `02-Registro-Activos.md`, `03-Analisis-Riesgos.md` | Parcial |
| 5.35 | Revisión independiente de la seguridad | Sí | Auditoría de la cátedra (14/10) y Red Team (28/10 a 09/11). | `LETRA.md` (sección 6.6 y 7) | Pendiente |
| 5.36 | Cumplimiento de políticas y normas | Sí | Controles del MCU 5.0 documentados con su estado. | `01-controles-mcu5-perfil-avanzado.xlsx` | Parcial |
| 5.37 | Procedimientos operativos documentados | Sí | Guía de pruebas, scripts y READMEs de cada componente. | `docs/guia-de-pruebas.md`, `scripts/`, `README.md` | Implementado |
| 6.1 | Verificación de antecedentes | N.A. | No se contrata personal. | — | N.A. |
| 6.2 | Términos y condiciones de empleo | N.A. | No hay relación laboral. | — | N.A. |
| 6.3 | Concientización, educación y capacitación | Sí | Inducción del equipo en la bitácora (15/09) y repaso de la letra. No hay plan formal. | `99-bitacora-trabajo.md` | Parcial |
| 6.4 | Proceso disciplinario | N.A. | No hay personal sujeto a sanciones. | — | N.A. |
| 6.5 | Responsabilidades después del cese | Sí | Si un integrante deja el proyecto, se da de baja su acceso al panel. No hay procedimiento escrito. | `09-Gestion-Accesos.md` (sección 2) | Parcial |
| 6.6 | Acuerdos de confidencialidad | N.A. | Proyecto académico; los datos son de prueba. | — | N.A. |
| 6.7 | Trabajo remoto | Sí | Los integrantes trabajan desde equipos propios. No hay política escrita. | — | Pendiente |
| 6.8 | Reporte de eventos de seguridad | Sí | Canal definido: bitácora y registro de incidentes. | `04-Gestion-Incidentes.md`, `01-...` (sección 7) | Parcial |
| 7.1 | Perímetros de seguridad física | N.A. | No hay instalaciones propias; la solución corre en el laboratorio del curso. | — | N.A. |
| 7.2 | Ingreso físico | N.A. | Ídem 7.1. | — | N.A. |
| 7.3 | Oficinas, salas e instalaciones | N.A. | Ídem 7.1. | — | N.A. |
| 7.4 | Monitoreo de seguridad física | N.A. | Ídem 7.1. | — | N.A. |
| 7.5 | Protección contra amenazas físicas y ambientales | N.A. | Ídem 7.1. | — | N.A. |
| 7.6 | Trabajo en áreas seguras | N.A. | Ídem 7.1. | — | N.A. |
| 7.7 | Escritorio y pantalla limpios | N.A. | Los equipos son de los integrantes y están fuera del alcance de la solución. | — | N.A. |
| 7.8 | Emplazamiento y protección de equipos | N.A. | Ídem 7.1. | — | N.A. |
| 7.9 | Seguridad de equipos fuera de las instalaciones | Sí | Las bóvedas y las claves viven en equipos portátiles que salen de las instalaciones. No hay política de cifrado de disco. | `02-Registro-Activos.md` (A02, A03) | Pendiente |
| 7.10 | Medios de almacenamiento | Sí | Los backups y las copias del JSONL están en un disco local sin cifrar. | `06-Plan-Continuidad.md` (sección 2) | Parcial |
| 7.11 | Servicios de soporte (energía, etc.) | N.A. | Ídem 7.1. | — | N.A. |
| 7.12 | Seguridad del cableado | N.A. | Ídem 7.1. | — | N.A. |
| 7.13 | Mantenimiento de equipos | N.A. | Ídem 7.1. | — | N.A. |
| 7.14 | Eliminación o reutilización segura de equipos | Sí | Los dumps de prueba y las claves de prueba no se borran de forma segura. | `06-Plan-Continuidad.md` | Pendiente |
| 8.1 | Dispositivos de usuario final | Sí | El cliente corre en los equipos de los usuarios. El agente de Wazuh para esos equipos no está desplegado. | `02-Registro-Activos.md` (A01, A09) | Parcial |
| 8.2 | Derechos de acceso privilegiado | Sí | Rol admin en el panel. El TOTP es opcional para todos los roles. | `09-Gestion-Accesos.md` | Parcial |
| 8.3 | Restricción del acceso a la información | Sí | Bóveda cifrada, claves por agente y RBAC del panel. | `09-...`, `03-...` (R02) | Parcial |
| 8.4 | Acceso al código fuente | Sí | GitHub con escritura para el equipo. Sin protección de la rama `master`. | Git | Parcial |
| 8.5 | Autenticación segura | Sí | Argon2id, TOTP y espera escalonada. WebAuthn y Windows Hello fuera de la entrega. | `09-...`, `guia-de-pruebas.md` (C-10) | Parcial |
| 8.6 | Gestión de la capacidad | Sí | RNF-05 (100 usuarios y 5000 credenciales) no medido. | `LETRA.md` (RNF-05) | Pendiente |
| 8.7 | Protección contra código malicioso | Sí | No hay antivirus ni EDR configurado. El agente de Wazuh con FIM está previsto y no desplegado. | `infra/wazuh/README.md` | Pendiente |
| 8.8 | Gestión de vulnerabilidades técnicas | Sí | Escaneos de dependencias y SAST corridos; red y web pendientes. Dos avisos sin corrección y siete de starlette abiertos. | `10-Gestion-Vulnerabilidades.md` | Parcial |
| 8.9 | Gestión de la configuración | Sí | Configuración en `.env.example`, `docker-compose.yml` y READMEs. Los valores de desarrollo siguen siendo `changeme`. | `infra/`, `control-central/README.md` | Parcial |
| 8.10 | Eliminación de la información | Sí | Los eventos de más de 90 días se purgan al arrancar, salvo los vinculados a incidentes o alertas. | `control-central/app/db/retention.py`, `07-...` | Implementado |
| 8.11 | Enmascaramiento de datos | Sí | Logs, correos y KPIs no incluyen secretos ni el hash de la maestra. Los datos de prueba son ficticios. | `07-...`, `guia-de-pruebas.md` (B-08) | Parcial |
| 8.12 | Prevención de fuga de datos | Sí | Sin secretos en el JSONL ni en el correo. Sin TLS en el canal de eventos (limitación acordada). | `00-arquitectura-c4.md`, `07-...` | Parcial |
| 8.13 | Copias de respaldo de la información | Sí | Backup de PostgreSQL y copia del JSONL, probados. Falta copia fuera del equipo y backup diario automático. | `06-Plan-Continuidad.md` | Parcial |
| 8.14 | Redundancia de instalaciones de tratamiento | Sí | Un solo nodo; no hay alta disponibilidad. RTO y RPO de RNF-03 sin redundancia. | `06-Plan-Continuidad.md` (sección 1) | Pendiente |
| 8.15 | Registro de eventos (logging) | Sí | Eventos del central en Postgres y en JSONL. Los logs de PostgreSQL y del sistema operativo no se registran. | `07-Monitoreo-Logs-SIEM.md` (sección 2) | Parcial |
| 8.16 | Actividades de monitoreo | Sí | Reglas de detección probadas con la API; Wazuh no está levantado. | `07-Monitoreo-Logs-SIEM.md` (sección 6) | Parcial |
| 8.17 | Sincronización de relojes | Sí | Los eventos usan UTC. La validez de la firma depende de la hora del equipo; no hay NTP documentado. | `security.py`, `signer.rs` | Parcial |
| 8.18 | Uso de utilidades con privilegios | Sí | Los scripts usan `docker exec` y pg_dump con el usuario de la base. | `scripts/backup_bd.sh`, `scripts/restaurar_bd_prueba.sh` | Parcial |
| 8.19 | Instalación de software en sistemas operativos | Sí | Las imágenes y dependencias están fijadas. No hay un procedimiento escrito de instalación. | `docker-compose.yml`, `requirements.txt` | Parcial |
| 8.20 | Seguridad de las redes | Sí | Red interna de Docker. Sin VLAN ni firewall del laboratorio configurados. | `02-Registro-Activos.md` (A14) | Parcial |
| 8.21 | Seguridad de los servicios de red | Sí | Puertos publicados: 8001, 3000, 5432, 1025 y 8025. El puerto 5432 es accesible desde el equipo. | `infra/docker-compose.yml` | Parcial |
| 8.22 | Segregación de redes | Sí | Una sola red `blue-team-net` para todos los servicios. | `infra/docker-compose.yml` | Parcial |
| 8.23 | Filtrado web | N.A. | No hay navegación gestionada por la solución. | — | N.A. |
| 8.24 | Uso de criptografía | Sí | Argon2id, XChaCha20-Poly1305, RS256 para los eventos. TLS en tránsito pendiente (limitación). | `cipher.rs`, `kdf.rs`, `security.py`, `00-...` | Parcial |
| 8.25 | Ciclo de vida de desarrollo seguro | Sí | CI en GitHub Actions con tests de Rust. El backend en Python todavía no tiene tests automáticos; bandit y pip-audit se corrieron a mano. | `.github/workflows/cliente-gestor.yml`, `10-...` | Parcial |
| 8.26 | Requisitos de seguridad de aplicaciones | Sí | RF y RNF de la letra, y riesgos RT-01 a RT-12 como requisitos de seguridad. | `LETRA.md`, `03-Analisis-Riesgos.md` | Implementado |
| 8.27 | Principios de arquitectura segura | Sí | Zero-knowledge, Zero Trust y mínimo privilegio en la arquitectura. | `00-arquitectura-4mas1.md`, `00-arquitectura-c4.md` | Implementado |
| 8.28 | Codificación segura | Sí | Bandit sin hallazgos. Validación de entradas con Pydantic. | `10-bandit.txt` | Parcial |
| 8.29 | Pruebas de seguridad | Sí | Tests de cripto y bóveda, simulación de casos de uso y escaneos. No hay pentest propio. | `07-simulacion-casos-uso.txt`, `10-...` | Parcial |
| 8.30 | Desarrollo externalizado | N.A. | No se tercerizó el desarrollo. | — | N.A. |
| 8.31 | Separación de entornos | Sí | Sólo existe el entorno de desarrollo; no hay un entorno de pruebas separado de la base de trabajo. | `06-Plan-Continuidad.md` | Pendiente |
| 8.32 | Gestión de cambios | Sí | Cambios por commit con bitácora, CI y tags. | `99-bitacora-trabajo.md`, Git | Parcial |
| 8.33 | Protección de los datos de prueba | Sí | Datos ficticios (`sistema-prueba`, `agente-dev-01`). | `guia-de-pruebas.md` | Implementado |
| 8.34 | Protección de sistemas durante auditorías | Sí | Las evidencias no contienen secretos ni datos reales. Las claves de prueba se generan en el laboratorio. | `docs/evidencias/` | Parcial |

---

## 4. Análisis de brecha MCU 5.0

La madurez es una estimación del equipo (escala 0 a 4) y debe validarla la auditoría del 14/10.

| Función MCU 5.0 | Perfil objetivo | Evidencia que lo sostiene | Madurez estimada (0-4) | Acciones para subir |
|---|---|---|---|---|
| Gobernar | Avanzado | Política (borrador), RACI, bitácora, controles Excel | 2 | Aprobar la política y definir el procedimiento de revisión |
| Identificar | Avanzado | Inventario, riesgos, vulnerabilidades (parcial) | 3 | Asignar CVSS y completar el escaneo de red y web |
| Proteger | Avanzado | Cifrado, MFA TOTP, RBAC, backup | 2 | Subir FastAPI, TLS y copia externa |
| Detectar | Avanzado | Reglas de detección probadas con la API, JSONL | 2 | Levantar el manager de Wazuh y desplegar el agente |
| Responder | Avanzado | Procedimiento y un incidente simulado resuelto | 2 | Registrar lecciones aprendidas y el flujo de notificación |
| Recuperar | Avanzado | Backup y restauración probados con datos | 2 | Copia externa, backup diario automático y firma del responsable |

---

## 5. Plan de tratamiento (resumen de los pendientes)

| ID | Control | Acción | Prioridad | Responsable (RACI) | Fecha límite |
|---|---|---|---|---|---|
| T-01 | 5.1 | Aprobar la política de seguridad y registrar la firma del RSI | Alta | Andrés Varela | 07/10/2026 |
| T-02 | 5.10, 6.7, 7.9 | Escribir la política de uso aceptable, de trabajo remoto y de dispositivos portátiles | Media | Andrés Varela | 14/10/2026 |
| T-03 | 5.5, 6.8 | Terminar la notificación simulada a BCU y URCDP (`12-...`) | Alta | Andrés Varela | 07/10/2026 |
| T-04 | 8.13, 5.30 | Backup diario automático y copia fuera del equipo | Alta | Pablo Morales | 07/10/2026 |
| T-05 | 8.8, 5.22 | Asignar CVSS a los avisos abiertos y completar el escaneo de red y web | Media | Pablo Morales | 14/10/2026 |
| T-06 | 8.16, 8.15 | Levantar el manager de Wazuh y desplegar el agente (o justificarlo como limitación) | Alta | Pablo Morales | 14/10/2026 |
| T-07 | 8.24, 8.21 | TLS en el borde (limitación acordada: queda documentada, no implementada) | Baja | Pablo Morales | Limitación documentada |
| T-08 | 8.7 | Antivirus o EDR en los equipos de los usuarios (el agente de Wazuh cubre parte) | Media | Horacio Duarte | 14/10/2026 |
| T-09 | 8.6 | Medir RNF-05 con carga básica | Media | Horacio Duarte | 14/10/2026 |
| T-10 | 8.14 | Alta disponibilidad (limitación: un solo nodo, documentado) | Baja | Pablo Morales | Limitación documentada |
| T-11 | 8.31 | Separar entorno de pruebas del de trabajo | Baja | Pablo Morales | 14/10/2026 |
| T-12 | 5.35 | Auditoría de la cátedra y Red Team | Alta | Equipo | 14/10/2026 y 09/11/2026 |
| T-13 | 7.14 | Procedimiento de borrado seguro de dumps y claves de prueba | Baja | Pablo Morales | 14/10/2026 |
| T-14 | 5.32 | Revisar licencias de las dependencias | Media | Andrés Varela | 14/10/2026 |

Las limitaciones de WebAuthn, Windows Hello y TLS ya están documentadas como decisión acordada con el
docente (`09-Gestion-Accesos.md` y `00-arquitectura-c4.md`).

---

## Check de aceptación

- [x] Anexo A completo, con los 93 controles y la aplicabilidad justificada.
- [x] Brecha MCU 5.0 por función con madurez estimada.
- [x] Plan de tratamiento con responsables y fechas.
- [ ] Vinculados los entregables de `docs/` como evidencia (hecho en la columna de insumos; falta revisión del RSI).
- [ ] Revisión y firma del RSI.

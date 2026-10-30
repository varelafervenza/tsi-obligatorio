# Alcance preliminar de evaluación Red Team

Fecha de preparación: 19/10/2026. Estado: propuesta documental, pendiente de confirmar con el docente y el Blue Team.

**Ejecución posterior:** esta propuesta inicial describe objetivos y restricciones de la letra, pero no la procedencia real de las pruebas. Según la aclaración del usuario, todas las pruebas Red Team documentadas se lanzaron desde su Windows principal; el backend y la base de datos continuaron en la VM Linux. La VM Windows preparada no se usó para P01–P07. La discrepancia entre esta ejecución y el alcance/ventana de la consigna requiere confirmación de la cátedra; no se altera la regla original para ocultarla. Véase [entorno del laboratorio](01-entorno-laboratorio.md).

## Objetivo

Evaluar los controles de seguridad del gestor de contraseñas y del control central de la Tarea 1. Registrar pruebas reproducibles, evidencia, impacto y respuesta de la defensa según la sección 7 de la letra. Esta preparación no contiene hallazgos ni resultados de ataques.

## Componentes identificados

Inventario obtenido del código y la documentación; los puertos son configuración declarada, no servicios verificados en ejecución. La inclusión definitiva depende del alcance acordado.

| Componente | Ubicación o acceso declarado | Aspectos a evaluar |
|---|---|---|
| Cliente de escritorio Tauri / React | `cliente-gestor/` | Acceso a bóveda, MFA, manejo de errores y exposición de secretos. |
| Bóveda local y archivos exportados | Ruta de datos de la app; documentada por defecto como `boveda-prueba.sqlite` | Cifrado, contraseña maestra e integridad de importación/exportación sobre copias de prueba. |
| Control central FastAPI | `control-central/`; `127.0.0.1:8001` | Autenticación, permisos, MFA del panel y validación de eventos firmados. |
| PostgreSQL | Servicio `postgres` en la red Docker `blue-team-net`, sin puerto publicado | Persistencia y restricciones de acceso; verificar backups/restauración en el entorno acordado. |
| Grafana | `127.0.0.1:3000` | Accesos y tratamiento de datos mostrados en el dashboard. |
| Mailpit | SMTP `127.0.0.1:1025`; interfaz `127.0.0.1:8025` | Notificaciones de laboratorio y posibilidad de suplantación dentro del alcance. |
| Wazuh manager y agente | Red interna Docker, sin puertos publicados | Detección de intentos fallidos, borrados y cambios de maestra; evidencia de alertas y respuesta. |
| Repositorio y configuración | Código, Dockerfiles, archivos de ejemplo y documentación | Secretos expuestos y diferencias entre controles declarados e implementados. No copiar secretos reales a evidencias. |

Mailu y TheHive requieren despliegues adicionales y no se incluyen en esta propuesta inicial. Confirmar si estarán presentes en el laboratorio. El README documenta TLS y WebAuthn/Windows Hello como limitaciones de entrega; su ausencia declarada no constituye por sí sola un hallazgo validado.

## Reglas tomadas de la consigna

- Evaluar únicamente el laboratorio del curso y las máquinas autorizadas.
- No realizar exfiltración real, DoS destructivo no pactado ni ataques fuera de las VMs docentes.
- Conservar la entrega evaluada sin modificarla. Trabajar con datos ficticios y copias de prueba para la bóveda.
- Registrar diariamente cada intento, su resultado y la evidencia en la bitácora.
- Acordar los límites antes de pruebas de disponibilidad o cambios de estado necesarios para evaluar controles.

## Versión e integridad

El 19/10/2026 se consultó el repositorio local: HEAD `4b96c91e98231be6f02ca3fcc1c7b05655b5fc41` y existe el tag `v1.0`. Esto no confirma que HEAD corresponda a la entrega congelada ni acredita su integridad.

Pendiente: identificar el commit exacto de entrega, obtener el SHA-256 de referencia del Blue Team y comparar el artefacto recibido. Registrar el resultado y conservar la referencia durante la evaluación.

## Datos por confirmar

| Dato | Estado |
|---|---|
| Integrantes y responsable del Red Team | Pendiente de completar. |
| IPs, VMs y hosts autorizados | Pendiente de acordar con el docente. |
| Técnicas autorizadas y límites de carga | Pendiente de acordar. |
| Cuentas y datos ficticios de prueba | Pendiente de coordinar. |
| Contacto del Blue Team y procedimiento de interrupción | Pendiente de coordinar. |
| Ventana de pruebas | La letra establece 28/10–09/11/2026. Confirmar porque el 28/10 también exige una preentrega del 70 %. |

## Próximos pasos

1. Completar responsable y hora real de la actividad en la bitácora. La fecha corresponde al 19/10/2026 en America/Montevideo; la columna de hora del Excel exige UTC.
2. Confirmar los datos pendientes y la versión congelada.
3. Leer `docs/guia-de-pruebas.md` y la arquitectura para comprender los flujos esperados.
4. Preparar una matriz de pruebas con objetivo, control esperado, procedimiento propuesto, evidencia y resultado. Mantener resultados pendientes hasta ejecutar las pruebas.
5. Registrar las pruebas autorizadas en la bitácora e incorporar únicamente hallazgos confirmados al informe.

## Fuentes del proyecto

- `../../LETRA.md`, secciones 7 y 8.2–8.3.
- `../../README.md` y `../../cliente-gestor/README.md`.
- `../../infra/docker-compose.yml`.
- `../../../plantilla/informe-red-team.md`.

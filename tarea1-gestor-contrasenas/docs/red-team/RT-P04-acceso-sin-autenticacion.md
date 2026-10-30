# RT-P04 — Consulta de auditoría sin autenticación

Fecha de la verificación inicial: **21/10/2026**. Comprobación adicional en Burp Suite Community: **28/10/2026**. Hallazgo relacionado: **RT-H01**. Estado: **lectura de auditoría sin autenticación reproducida en Postman y Burp**.

## Objetivo y alcance

Comprobar si una persona con conectividad a la API puede consultar auditoría y métricas sin iniciar sesión ni presentar un token. Son consultas GET acotadas: no crean eventos, no modifican la bóveda y no cambian permisos.

Las capturas manuales corresponden a Postman y Burp Suite Community ejecutados en el Windows principal, conectados a la API del backend Linux por el túnel local `127.0.0.1:18001`. No documentan una ejecución desde la VM Windows.

## Procedimiento reproducible en Postman

Usar la carpeta **RT-P04 - Acceso a auditoria sin autenticacion** de la colección en `postman/TSI-RedTeam-Manual.postman_collection.json`, o importar `postman/RT-P04.postman_collection.json`.

1. Configurar `base_url` con la dirección accesible del backend del laboratorio. Comprobar previamente su disponibilidad.
2. Seleccionar **No Auth**, quitar cualquier cabecera `Authorization` y eliminar/deshabilitar las cookies del dominio. No enviar un body.
3. Enviar `GET {{base_url}}/api/events/?limit=1`. Guardar el estado HTTP y el cuerpo recibido.
4. Como control comparativo, enviar `GET {{base_url}}/api/users/` en las mismas condiciones.
5. Opcionalmente, ampliar la comprobación con `GET {{base_url}}/api/dashboard/kpis`, también sin autenticación.

La prueba de auditoría de la colección espera **401 o 403**: si recibe 200, su aserción falla y muestra el hallazgo. El control de usuarios espera 401.

## Resultado verificado

| Solicitud sin token | Resultado inicial observado | Interpretación |
|---|---|---|
| `GET /api/events/?limit=1` | HTTP 200 y un registro | Permite consultar auditoría sin autenticación. |
| `GET /api/dashboard/kpis` | HTTP 200 y un objeto de métricas | Permite consultar datos operacionales sin autenticación. |
| `GET /api/users/` | HTTP 401 | Control comparativo: esta ruta sí exige autenticación. |

Los tres resultados de la tabla provienen de la comprobación HTTP instrumentada conservada en la revisión inicial. Además, una captura real de Postman muestra `GET {{base_url}}/api/events` con **No Auth**, respuesta **200 OK** y registros de auditoría en el cuerpo. Esa captura respalda de manera independiente la lectura de eventos, pero no la de KPI ni el control comparativo de usuarios. Una consulta posterior que agotó el tiempo de espera tampoco acredita corrección.

### Comprobación comparativa en Burp — 28/10/2026

En Burp Repeater se envió `GET /api/events/?limit=1` a `127.0.0.1:18001`, sin cabecera `Authorization` ni cookies visibles en la solicitud. La respuesta fue **HTTP 200** y devolvió un registro de auditoría (`id: 89`, `tipo: modificacion_credencial`, `sistema: RT-P02-Alterado`, `firma_valida: false`). La respuesta demuestra lectura de metadatos, no acceso a secretos de la bóveda.

Desde el navegador integrado de Burp, con Proxy Intercept activado, se capturó y reenvió `GET /api/alerts/?limit=1` bajo las mismas condiciones de autenticación. La historia HTTP mostró **401**. Es un control positivo: esa ruta sí rechazó la lectura sin sesión. La comparación refuerza RT-H01, sin constituir un hallazgo nuevo. La captura de historia muestra el 401, pero no el cuerpo de esa respuesta. El control de usuarios 401 y KPI 200 siguen sustentados solo por la verificación inicial.

## Impacto y causa

Un actor que alcance la API puede leer metadatos de actividad, sistemas, agentes, tipos de eventos y estados de firma sin pasar por el login/MFA del panel. No se observaron contraseñas de la bóveda y no se demostró acceso desde Internet. Severidad asignada: **Media, CVSS v3.1 5.3**, `CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:L/I:N/A:N`; justificación en [Gestión de vulnerabilidades](gestion-vulnerabilidades.md).

En el código revisado, `control-central/app/api/events.py` (`listar_eventos`) y `control-central/app/api/dashboard.py` (`/kpis`) no exigen la dependencia de usuario autenticado (`usuario_actual`). `control-central/app/main.py` tampoco impone autenticación global. Comparación: `control-central/app/core/auth.py` y las rutas de usuarios. La política declarada en `docs/09-Gestion-Accesos.md` exige dos factores para acceder al panel/dashboard; las consultas observadas permiten leer estos datos sin sesión del panel.

## Evidencias y pendientes

Se reutilizan los originales existentes, sin duplicarlos ni crear capturas artificiales:

- [Resultado de la reproducción inicial](evidencias/revision-inicial/resultado-reproduccion-aislada.json): interpretar solamente la sección de GET para RT-P04.
- [Script de la verificación](evidencias/revision-inicial/reproduccion-aislada.sh): condiciones y solicitudes realizadas. La parte de reglas en memoria queda fuera de esta prueba.
- [Hashes SHA-256](evidencias/revision-inicial/SHA256SUMS.txt): integridad de los archivos recopilados.
- [Captura de Postman sin autenticación](evidencias/RT-P04/01%20-%20Consultar%20auditoria%20sin%20token.PNG): solicitud GET, No Auth, respuesta 200 y cuerpo; [hash SHA-256](evidencias/RT-P04/SHA256SUMS.txt).
- [Burp Repeater: eventos 200](evidencias/RT-P04/02-burp-eventos-200.png): solicitud sin credenciales visibles y respuesta con un registro.
- [Burp Intercept: alertas retenidas](evidencias/RT-P04/03-burp-intercept-alertas.png): solicitud GET antes de enviarse.
- [Burp HTTP history: alertas 401](evidencias/RT-P04/04-burp-alertas-401.png): control comparativo tras pulsar Forward. Los hashes de las tres capturas se incluyen en [SHA256SUMS.txt](evidencias/RT-P04/SHA256SUMS.txt).

El script depende del contenedor del backend. Las consultas GET leen el servicio real y no generan usuarios, cambian roles o MFA ni ejecutan carga o borrado. El ensayo de reglas en memoria incluido en el archivo no forma parte de los hallazgos vigentes.

Pendiente: capturas o exportaciones de `GET /api/dashboard/kpis` y `GET /api/users/` en las mismas condiciones. No hay una captura íntegra de los cuerpos HTTP de la verificación inicial; se conserva su resumen instrumentado. La comparación con alertas 401 es un control adicional, distinto del control de usuarios pendiente de captura.

## Recomendación y criterio de cierre

Exigir autenticación y autorización para auditoría y métricas, conforme a los permisos del panel. La firma JWS de los eventos entrantes no sustituye ese control de lectura. Conservar el endpoint de salud público solo con información mínima si resulta necesario.

Después de una corrección autorizada, repetir las solicitudes: sin sesión deben responder 401/403 y una sesión con permisos debe poder consultar los datos. Este informe documenta el hallazgo; no se cambió el código para corregirlo.

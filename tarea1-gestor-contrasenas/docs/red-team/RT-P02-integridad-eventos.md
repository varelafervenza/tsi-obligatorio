# RT-P02 — Alteración de un evento firmado

## Incidencia de conexión durante una solicitud posterior — 27/10/2026

El usuario informó que una solicitud de Postman a `{{base_url}}/api/events/` quedó cargando sin mostrar respuesta. La consulta de salud por el túnel local agotó el tiempo de espera, mientras que la API dentro de Linux respondió 200 con la base disponible. Tras reiniciar únicamente el túnel, la salud volvió a responder 200.

Una consulta posterior `GET /api/events/?limit=1` devolvió el evento **ID 61**, `alta_credencial`, sistema `RT-P02-Original`, agente `agente-215b7431`, `occurred_at=2026-10-21T12:01:26Z`, `received_at=2026-10-27T22:42:15.981736Z` y `firma_valida=false`. [Respuesta GET conservada](evidencias/RT-P02/consulta-2026-10-27.json) y su hash en `evidencias/RT-P02/SHA256SUMS.txt`. El momento y el sistema son compatibles con la solicitud que el usuario intentaba enviar; no se conservó un registro de la petición POST ni su respuesta visible en Postman, por lo que no se atribuye con certeza absoluta el ID 61 a ese clic.

El timestamp firmado corresponde a seis días antes de su recepción; es compatible con una firma vencida, aunque esta consulta no incluye `firma_jws` ni permite determinar por sí sola la causa exacta de invalidez. **No reenviar** ese body histórico como prueba de firma válida. Esta incidencia no cambia el resultado original de RT-P02, que se documenta más abajo.

Fecha: 21/10/2026. Resultado: alteración detectada; el evento se almacena identificado como no válido.

## Objetivo

Comprobar que cambiar el campo `sistema` de un evento sin generar una firma nueva invalida la correspondencia entre el mensaje recibido y el contenido firmado.

## Procedimiento realizado

1. El usuario creó una credencial ficticia con sistema `RT-P02-Original` en el gestor ejecutado en su Windows principal.
2. Se consultó su evento en PostgreSQL: ID `11`, tipo `alta_credencial`, firma válida.
3. Se verificó nuevamente el evento con la función normal del backend antes de probar la alteración. La firma seguía vigente y válida.
4. Se copió la solicitud, cambiando únicamente `sistema` a `RT-P02-Alterado`. Se conservaron tipo, agente, timestamp y firma originales. No se usó ninguna clave privada ni se generó una firma nueva.
5. Se envió una única solicitud modificada a `POST /api/events/` del backend Linux por el acceso local del laboratorio.
6. Se consultó el registro nuevo y se comprobó que el original no había cambiado y su firma seguía siendo válida.

## Resultado

| Registro | ID | Sistema | Firma válida |
|---|---|---|---|
| Original | 11 | RT-P02-Original | true |
| Copia alterada | 43 | RT-P02-Alterado | false |

La API devolvió HTTP `201 Created` y `firma_valida=false` para la copia alterada. La comprobación del original antes y después evita atribuir el resultado al vencimiento de la firma. La bóveda y sus credenciales no se modificaron durante el envío de la copia.

El control de correspondencia entre mensaje y firma detectó la alteración. Esto no prueba que la firma RS256 en sí haya sido modificada: la misma firma permanece correcta para el contenido original, pero el campo recibido ya no coincide con el contenido firmado.

## Interpretación y límites

El backend almacena eventos no válidos identificándolos como tales; HTTP 201 no significa que los considere auténticos. Para evaluar si ese diseño produce un hallazgo, habría que probar por separado cómo consumen esos registros el dashboard, el correo, las reglas y la gestión de incidentes. Esta prueba no demuestra elevación de privilegios ni evasión de detección.

Antes del envío, el laboratorio dejó de responder por SSH y HTTP y se recuperó mediante reinicio. La solicitud alterada se envió únicamente después de recuperar la API y revalidar el original. Se conservaron por separado la respuesta HTTP y la consulta de PostgreSQL. Cualquier limitación de lectura del registro de auditoría se anota en las evidencias, sin modificar el archivo origen.

## Evidencias

Carpeta `evidencias/RT-P02/`:

- `solicitud-original.json` y `solicitud-alterada.json`: comparación de solicitudes, con la misma firma y un solo campo cambiado.
- `respuesta-api.json`: respuesta real de la API.
- `resultado.json`: resultado de las verificaciones de firma, identificadores y registro consultado.
- `auditoria-original.jsonl`: líneas recuperadas del registro de auditoría correspondientes a los eventos de la prueba.
- `SHA256SUMS.txt`: hashes de los archivos JSON y JSONL.

Las solicitudes contienen una firma de auditoría del laboratorio con vigencia limitada; no contienen la clave privada ni las contraseñas de la bóveda.

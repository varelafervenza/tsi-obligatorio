# RT-P03 — Reenvío de un evento firmado (replay)

Fecha de ejecución: **21/10/2026**, hora de Montevideo (UTC−03). Los registros originales conservan timestamps UTC del 22/10/2026. Estado: **duplicación de registros y notificaciones confirmada**.

## Objetivo

Determinar si el control central reconoce un evento firmado previamente recibido y evita duplicar sus registros y efectos cuando se reenvía el mismo mensaje durante la vigencia de la firma.

## Alcance y condiciones

- Endpoint: `POST /api/events/` del laboratorio autorizado.
- Datos ficticios: sistema `RT-P03-Original`, usuario de prueba y credencial de laboratorio.
- Evento original generado por el gestor: ID **51**.
- Dos envíos manuales en Postman usando exactamente el mismo cuerpo. No se alteró `sistema`, `tipo`, `agente_id`, `timestamp` ni `firma_jws`.
- El gestor y Postman se ejecutaron en el Windows principal del usuario; el destino fue la API del backend Linux accesible mediante el túnel local. No se usó la VM Windows para los dos reenvíos.
- No se usó una clave privada para el reenvío ni se modificó la bóveda. La recopilación posterior realizó únicamente lecturas.

## Procedimiento reproducible

1. Crear una credencial ficticia con sistema `RT-P03-Original` desde el gestor para obtener un evento recién firmado.
2. Comprobar que el evento original esté almacenado con `firma_valida=true` y que la firma siga vigente.
3. En Postman, seleccionar POST, la URL `/api/events/` del laboratorio y Body → raw → JSON.
4. Pegar la solicitud original completa, incluida su firma, sin modificar sus campos. Enviar una sola vez y registrar el ID y el estado de firma de la respuesta.
5. Enviar una segunda vez el mismo cuerpo. Registrar el nuevo ID y estado de firma.
6. Consultar PostgreSQL y el registro de auditoría para comparar los tres registros. Verificar si se repitieron las notificaciones en Mailpit.

Para esta ejecución se conserva el cuerpo en `body-identico-reconstruido.json`, obtenido de los campos almacenados en PostgreSQL. Sirve para comparar el mensaje; no es una captura HAR de los envíos. Su firma tiene vencimiento, por lo que una repetición futura requiere otro evento nuevo completo. Cambiar solamente el timestamp invalidaría la firma y probaría otra condición.

## Resultado observado

| Paso | ID | Firma válida | Recepción, hora de Montevideo (21/10/2026) |
|---|---:|---|---|
| Evento original del gestor | 51 | true | 22:17:03.748175 |
| Primer reenvío en Postman | 52 | true | 22:17:51.453214 |
| Segundo reenvío idéntico en Postman | 53 | true | 22:19:22.531800 |

Se verificó directamente en PostgreSQL que los tres registros conservan los mismos campos firmados y el mismo valor íntegro de `firma_jws`, pero tienen IDs y momentos de recepción diferentes. SHA-256 del valor de la firma común: `2c7216c977b82ec45e42dbcf10f45b8dc9416f61185623c30c0e908e9bf66ecb`.

El usuario aportó las respuestas de los POST de IDs 52 y 53, ambas con `firma_valida=true`. La consulta posterior corroboró los valores persistidos. No se dispone de una captura original de las cabeceras HTTP de esos dos envíos; por ello no se presenta como evidencia un estado HTTP capturado en Postman.

Los tres eventos aparecen en el archivo JSONL de auditoría. Mailpit contiene **tres notificaciones «Alta de credencial»**, una asociada al ID 51 y otras dos a los IDs 52 y 53. Por lo tanto, no solo se duplicó el registro: cada replay produjo otra notificación.

## Hallazgo RT-H02

**Título:** ausencia de deduplicación/idempotencia frente al reenvío de eventos firmados.

**Evidencia:** un mismo evento firmado se aceptó como tres registros distintos, incluyendo el envío original, y generó tres correos.

**Impacto confirmado:** duplicación de auditoría y notificaciones para una única acción original. Los duplicados pueden alterar el conteo de eventos por tipo, dado que el dashboard agrega registros sin identificar un evento lógico único.

**Impacto potencial, no demostrado aquí:** repetición de eventos sensibles para producir falsas correlaciones o consumo adicional de recursos. No se ejecutaron carga, envío masivo ni pruebas de eventos sensibles en este caso.

**Condición de explotación:** acceso de red al endpoint y posesión de un evento firmado todavía vigente. Esta prueba no demuestra cómo un tercero obtendría ese mensaje, ni accesibilidad de la API desde Internet. Capturar un evento no requiere necesariamente conocer la clave privada, pero el mecanismo de captura queda fuera de esta prueba.

La firma protege la integridad y autenticidad del mensaje, pero no impide por sí sola reutilizarlo. Aceptar reintentos puede ser necesario para una cola offline; el sistema debería reconocer el mismo evento y evitar duplicar sus efectos.

**Severidad:** Baja, CVSS v3.1 3.7, condicionada a los supuestos detallados en la sección Severidad de este documento.

## Causa observada en código

`control-central/app/api/events.py`, función `recibir_evento`, crea y persiste un `AuditEvent` en cada recepción, y después ejecuta el forwarder, las notificaciones y las reglas. No hay consulta de duplicados ni identificador lógico de evento con restricción única.

`control-central/app/models/event.py` define un ID autoincremental para cada fila, pero ese ID se genera después de recibir el mensaje y no permite reconocer un evento original reenviado.

## Recomendaciones

1. Generar en el cliente un identificador único por evento y cubrirlo con la firma.
2. Aplicar una restricción única en PostgreSQL sobre agente e identificador de evento, resolviendo también recepciones concurrentes.
3. Ante un reintento legítimo, devolver el resultado del evento ya registrado sin volver a ejecutar correo, forwarder ni correlación. También puede definirse un rechazo explícito de duplicados, según el contrato de la API.
4. Conservar la validación de firma y vencimiento; una ventana temporal corta no sustituye la deduplicación.
5. Verificar con una nueva prueba que dos envíos iguales produzcan un solo evento lógico y una sola notificación.

## Índice de evidencias

Carpeta: `evidencias/RT-P03/`.

| Archivo | Qué acredita |
|---|---|
| `eventos-postgresql-originales.json` | Los tres registros, sus campos, firmas y fechas de recepción. |
| `body-identico-reconstruido.json` | Cuerpo reconstruido a partir del original persistido. |
| `auditoria-original.jsonl` | Tres líneas originales de auditoría, conservadas sin editar. |
| `mailpit-listado-filtrado.json` | Metadatos y snippets originales de las tres notificaciones relacionadas. |
| `api-posts-contexto.log` | Líneas de POST recuperadas del log; contienen contexto de otras solicitudes y no prueban por sí solas qué ID corresponde a cada respuesta. |
| `resumen-verificacion.json` | Igualdad de campos y firma, IDs y procedencia de los archivos exportados. |
| `SHA256SUMS.txt` | Integridad de los archivos al momento de su recopilación. |

No se fabricaron capturas de Postman. Pueden adjuntarse como evidencia adicional las capturas o respuestas exportadas por el usuario de los pasos 1 y 2, mostrando los IDs y `firma_valida`.

## Severidad

**Baja, CVSS v3.1 3.7**, `CVSS:3.1/AV:N/AC:H/PR:N/UI:N/S:U/C:N/I:L/A:N`. Valoración condicionada a conseguir un mensaje legítimo vigente y sin sesión adicional exigida. Justificación y escenario alternativo AC:L (5.3, Media) en [Gestión de vulnerabilidades](gestion-vulnerabilidades.md). No se demostró indisponibilidad.

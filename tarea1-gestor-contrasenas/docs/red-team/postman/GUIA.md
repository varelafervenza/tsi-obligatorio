# Pruebas manuales en Postman

## Alternativa si la importación falla

Importar únicamente `TSI-RedTeam-Manual.postman_collection.json`. Su nombre visible es `TSI Red Team - Manual` y no incluye JWT en la colección. Los payloads están separados; no importarlos como colecciones.

La colección manual está ordenada en `00 - Comprobacion del backend`, `RT-P02 - Alteracion de eventos firmados` y `RT-P03 - Reenvio del mismo evento (replay)`. Cada prueba tiene dos envíos numerados y una consulta final. Los nombres de pasos descritos a continuación pueden aparecer abreviados; seguir su numeración y la carpeta correspondiente. La prueba RT-P01 de apertura de bóveda se realiza en el gestor y no se sustituye por envíos simulados desde Postman.

Para RT-P03, después de importar, abrir `1 - Primer reenvío idéntico` → Body → raw y reemplazar todo el ejemplo por el contenido de `rt03_payload.json`. El script de respuesta guarda ese cuerpo en la variable de colección; el paso `2 - Segundo reenvío idéntico` lo reutiliza sin cambios. Para RT-P02 se hace lo mismo con `rt02_payload.json` en su primer paso; el segundo altera solo `sistema` automáticamente.

Los archivos separados mantienen las mismas firmas y vencimientos indicados abajo. Separarlos no renueva su vigencia. Esta alternativa no confirma la causa del fallo de importación original y debe verificarse al importarla en Postman.

Importar `TSI-RedTeam.postman_collection.json` mediante Import → Files en Postman. Las solicitudes usan la variable de colección `base_url`. Linux y la conexión al backend deben estar activos. Para el acceso preparado, usar `http://127.0.0.1:18001`.

## RT-P03: repetir el mismo evento firmado

1. Ejecutar `00 - Salud / Comprobar backend`. Esperar HTTP 200 y `database: up`.
2. Abrir `RT-P03 - Replay / 1 - Primer reenvío idéntico`. El body usa `{{rt03_payload}}`, ya cargado en las variables de la colección. Pulsar Send una sola vez. Anotar el ID de respuesta y comprobar `firma_valida: true`. Si no es válido, detener la prueba y preparar un evento nuevo.
3. Abrir `2 - Segundo reenvío idéntico`. Pulsar Send una sola vez sin cambiar el body ni las variables. Comparar HTTP, ID y firma con la primera respuesta.
4. Ejecutar `3 - Consultar registros` y buscar `RT-P03-Original`. Existe previamente el evento 46 generado desde el gestor; los envíos manuales pueden agregar hasta otros dos registros. No confundir ese original con los IDs de los dos envíos de Postman.
5. Guardar las respuestas y capturas con los IDs, nombre de sistema y estado de firma en `docs/red-team/evidencias/RT-P03/`.

Interpretación: dos HTTP 201 con firma válida e IDs diferentes demuestran que esos dos reenvíos crearon registros nuevos. Esto es una observación sobre duplicados; analizar su impacto antes de clasificarla como vulnerabilidad. Un HTTP de error requiere revisar el cuerpo y no prueba por sí solo detección de replay.

La pestaña Test Results comprueba condiciones básicas. Las pruebas verdes no significan necesariamente que exista protección contra replay: revisar los IDs y la variable `rt03_observation`.

No usar Runner ni bucles para esta prueba. Cada POST agrega datos de auditoría y puede generar notificaciones de laboratorio.

## RT-P02: integridad del contenido

Opcionalmente, abrir la carpeta RT-P02. Enviar primero `1 - Original sin cambios` y verificar firma válida. Después enviar `2 - Solo sistema alterado`, que conserva la firma y cambia solo `sistema`. Debe figurar `firma_valida: false`. No concluir nada sobre alteración si el control original ya es inválido.

## Vigencia

Las firmas incluidas vencen. En esta preparación:

- RT-P02: `2026-10-21T16:01:26Z` (13:01:26 en Montevideo).
- RT-P03: `2026-10-21T16:21:41Z` (13:21:41 en Montevideo).

Después del vencimiento, crear un evento nuevo en el gestor y actualizar el payload firmado completo y su fecha de vigencia. No cambiar solo el timestamp o la firma a mano. Los payloads no contienen contraseñas de la bóveda ni claves privadas, pero deben mantenerse dentro del laboratorio durante su vigencia.

Estado de preparación: el evento original se verificó en PostgreSQL y con el validador de firma del backend. Los dos reenvíos de RT-P03 no fueron ejecutados al generar la colección. Se validó su estructura JSON y que ambos usan el mismo cuerpo; falta importar y ejecutar la colección en Postman.

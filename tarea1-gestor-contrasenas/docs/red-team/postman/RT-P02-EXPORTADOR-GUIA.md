# RT-P02 — Exportador de evento firmado

La API no entrega `firma_jws` en sus respuestas. El auxiliar `rtp02_exportador_local.py` añade exclusivamente un servidor de laboratorio en loopback `127.0.0.1:18002`, sin modificar el backend. Consulta por SSH el último evento de sistema `RT-P02-Original` y agente `agente-215b7431`. No lee claves privadas, no firma, no cambia la base ni envía eventos. Requiere las rutas del laboratorio D: indicadas en el script. No exponer el puerto a otras interfaces.

1. Con backend/SSH activos, ejecutar el auxiliar en el host Windows:

```powershell
& 'C:/Users/diego/.cache/codex-runtimes/codex-primary-runtime/dependencies/python/python.exe' './rtp02_exportador_local.py'
```

Ejecutar desde esta carpeta `postman`. Mantener la consola abierta; Ctrl+C lo detiene.

2. Desde el gestor del agente indicado, crear un alta ficticia con sistema exacto `RT-P02-Original` y confirmar envío al central. Esta acción sí genera un evento y posibles notificaciones. No usar datos personales.
3. Importar `RT-P02-Exportador.postman_collection.json`; enviar su GET. Si devuelve 404 falta el evento; 409 indica vencimiento/inconsistencia; 503 indica fallo de acceso. No continuar en esos casos.
4. Copiar únicamente el objeto `original` de la respuesta al body de 01 en `RT-P02.postman_collection.json`. Copiar el objeto `alterado` a 02. Quedan todos los campos visibles, incluida la firma. Los dos objetos conservan el mismo timestamp y firma; solo cambia sistema.
5. Enviar 01 y comprobar `firma_valida=true` antes de enviar 02. En 02 se espera `firma_valida=false`. Los POST crean nuevos registros y pueden generar notificaciones. No Runner ni bucles.

No basta pegar una firma nueva sobre el timestamp histórico: deben coincidir todos los campos firmados. Los cuerpos incluidos muestran los datos reales históricos de la prueba con la firma reemplazada por un marcador; no son válidos para ejecutar hasta sustituirlos por el evento nuevo completo. El auxiliar decodifica claims para comprobar fecha/coincidencia, pero la verificación criptográfica corresponde al backend. La colección exportadora obtiene una firma existente, no genera una nueva.

Preparación solamente: sintaxis/JSON verificados, sin prueba de integración del auxiliar ni envíos ejecutados. Conservar evidencia de cada futura ejecución por separado y no confundirla con los resultados históricos RT-P02.

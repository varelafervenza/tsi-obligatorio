# Generador reutilizable de firmas de laboratorio

Estado: preparado, no iniciado. Registro de clave pública en Linux pendiente de autorización explícita. Sin ese registro el backend no aceptará las firmas como válidas.

Importar `Generador-firma-JWS.postman_collection.json`. El auxiliar `generador_firmas_local.py` escucha solo en `127.0.0.1:18003`, sin CORS. No es un endpoint de producción ni necesita login del panel. Usa el agente exclusivo `agente-postman-pruebas` y una clave privada fuera del repositorio, en `%LOCALAPPDATA%/TSI-Laboratorio/firmas-postman/`. No exporta claves privadas a Postman ni Linux. Proteger el perfil local; no es un keystore de producción.

En el body visible elegir tipo y sistema ficticio con prefijo `RT-`. Tipos permitidos: alta_credencial, modificacion_credencial, borrado_credencial, cambio_maestra, intento_fallido_maestra y vencimiento_credencial. Cada Send devuelve `firma_jws` y `body_evento` completo, con agente y tiempo generados automáticamente, vigencia cinco minutos. Copiar body_evento completo a la prueba destino: la firma no sirve con timestamp, agente, sistema o tipo diferentes.

RT-P02: generar con RT-P02-Original, copiar el mismo body a ambas solicitudes y alterar únicamente sistema en la segunda. Replay: reenviar el mismo body sin generar otra firma entre envíos.

El generador no envía eventos al backend ni genera correos/alertas por sí mismo. Los envíos posteriores sí pueden hacerlo y forman parte de cada prueba, no de esta preparación.

Inicio manual después de autorizar y registrar la pública, desde esta carpeta:

```powershell
& 'C:/Users/diego/.cache/codex-runtimes/codex-primary-runtime/dependencies/python/python.exe' './generador_firmas_local.py'
```

No exponer fuera de loopback. La cabecera X-TSI-Lab no protege frente a otros procesos locales; sirve para restringir solicitudes casuales de navegador junto con rechazo de Origin/ausencia de CORS. Detener el auxiliar al terminar. Para retirar la confianza del agente hay que quitar su pública del backend mediante un cambio acordado.

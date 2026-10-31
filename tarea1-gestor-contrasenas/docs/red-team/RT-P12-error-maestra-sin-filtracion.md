# RT-P12 — Errores de la bóveda y exposición de secretos

**Fechas:** 30–31/10/2026 (America/Montevideo). **Objetivo:** RT-10, buscar exposición de secretos mediante errores de la aplicación. **Resultado:** no se observó filtración en los cuatro caminos de error probados. Prueba cerrada con el alcance indicado abajo.

## Qué se hizo

Con el gestor de escritorio en Windows principal, se intentó abrir `boveda-prueba.sqlite` con una contraseña incorrecta. Después se repitió una vez con un texto ficticio distintivo, creado solo para esta prueba. La bóveda no se abrió. No se usó una contraseña real como marcador ni se modificó la bóveda.

## Evidencia y resultado

- [Primer rechazo](evidencias/RT-P12/01-rechazo-inicial.png) y [rechazo con marcador ficticio](evidencias/RT-P12/02-rechazo-marcador.png): la interfaz mostró «Contraseña maestra incorrecta, o el archivo fue alterado.» y mantuvo la entrada oculta.
- La API de auditoría registró `intento_fallido_maestra` para `boveda` con firma válida. Se guardó [la respuesta de los eventos 94 y 95](evidencias/RT-P12/03-eventos-auditoria.json), recibidos el 30/10/2026 a las 23:13 y 23:16 de Montevideo (31/10 UTC). El archivo conserva solo los campos de metadatos que devuelve la consulta, sin firmas ni contraseñas. En ambos eventos, `occurred_at` supera a `received_at` por unos 33 segundos; parece existir un desfase de reloj entre cliente y servidor que no se investigó en esta prueba.
- En los archivos locales de texto y JSON de auditoría revisados (`auditoria.json` y `ultimo.txt`) no apareció el marcador ficticio. El código del cliente envía a `auditar` únicamente el tipo `intento_fallido_maestra` y el sistema `boveda`; `events::publicar` construye el evento con esos campos, identificador de agente, tiempo y firma.
- La [comprobación acotada de logs del backend](evidencias/RT-P12/04-verificacion-logs-backend.txt) no encontró el marcador ficticio en el contenedor `control-central` desde las 02:00 UTC del 31/10. Se evaluó presencia/ausencia sin copiar el contenido de los logs al informe.
- El usuario repitió la consulta en Linux y obtuvo `NO APARECE`; se conserva su [captura de la terminal](evidencias/RT-P12/05-consulta-logs-usuario.png). La captura corrobora la ausencia del marcador en ese intervalo de logs.

Los hashes SHA-256 de las capturas, el registro JSON y la verificación de logs están en [SHA256SUMS.txt](evidencias/RT-P12/SHA256SUMS.txt). Las capturas de la app no revelan el texto introducido porque el campo de contraseña lo oculta; su carácter ficticio consta por la instrucción de la prueba y la revisión local.

### Segundo camino: importar una copia inexistente

El 31/10/2026, con la bóveda ficticia vacía `boveda-passcorta.sqlite` abierta, el usuario intentó importar un archivo `.gex` que se había comprobado inexistente. En el campo de contraseña de transporte ingresó un marcador ficticio distinto del usado para la maestra. La [captura del resultado](evidencias/RT-P12/06-importacion-archivo-inexistente.png) muestra un error de archivo no encontrado (`os error 2`) sin revelar la contraseña de transporte. Después se verificó que el archivo de prueba seguía sin existir y que el marcador no aparecía en `auditoria.json` ni en `ultimo.txt`. La ruta de importación en el código lee el archivo antes de intentar descifrarlo; por eso este caso comprueba el error de lectura, **no** el error por contraseña de transporte incorrecta.

### Tercer camino: contraseña de transporte incorrecta

Con la misma bóveda ficticia vacía, el usuario generó una copia cifrada, confirmada por [«Copia cifrada guardada»](evidencias/RT-P12/07-copia-cifrada-guardada.png). La app utilizó su ruta sugerida `boveda-passcorta.gex`, no la ruta alternativa propuesta inicialmente. Luego intentó **Importar** esa copia con otra contraseña de transporte ficticia. La [captura del rechazo](evidencias/RT-P12/08-transporte-incorrecto.png) muestra «Contraseña de transporte incorrecta, o el archivo fue alterado.»; no revela la contraseña ingresada. La copia seguía presente con 74 bytes y SHA-256 `d7ad6d5f20d846b69658a58863f779abed2bece292e266bea157972430eaffd4`. El marcador incorrecto no apareció en `auditoria.json` ni en `ultimo.txt`. El código de importación aborta al fallar el descifrado, antes de escribir credenciales. No se inspeccionaron logs del sistema operativo del cliente.

### Cuarto camino: código TOTP incorrecto

El usuario confirmó que había abierto `boveda-prueba.sqlite` con TOTP válido. Cerró esa bóveda e intentó abrirla con la maestra correcta y el código ficticio `000000`. La [captura del rechazo](evidencias/RT-P12/09-totp-invalido.png) muestra «Código TOTP inválido.» y la bóveda cerrada. El error no revela la maestra ni el secreto TOTP. Los archivos locales de auditoría `auditoria.json` y `ultimo.txt` conservaron sus fechas previas; en la rama `TotpInvalido` de `abrir_boveda` el cliente devuelve el error sin llamar a `auditar`. Por ello no se esperaba un nuevo evento en el backend para ese fallo. No se inspeccionó memoria ni toda la telemetría del sistema operativo.

## Interpretación y límite

Los cuatro errores probados **no mostraron las contraseñas ficticias ni el secreto TOTP en la interfaz**. El intento de maestra tampoco incluyó la contraseña en el evento de auditoría observado ni se halló su marcador en los logs recientes del backend. Las importaciones fallidas no dejaron sus marcadores en los archivos locales de auditoría revisados. Son resultados negativos, no una vulnerabilidad confirmada. **Se cierra RT-10 para estos cuatro caminos:** maestra incorrecta, archivo de copia inexistente, contraseña de transporte incorrecta y TOTP inválido. No se revisaron exhaustivamente otros errores, logs de otros servicios, volcados de memoria ni trazas del cliente; por tanto, no se concluye que el sistema nunca pueda filtrar secretos.

## Recomendación

Conservar mensajes de error genéricos y eventos de auditoría sin contraseñas. Si se amplía el alcance de RT-10, revisar otras rutas de error que procesen secretos y comprobar sus logs con marcadores ficticios.

# RT-P13 — Comprobación acotada de inyección SQL en el login

**Fecha:** 31/10/2026. **Objetivo relacionado:** RT-03. **Resultado:** sqlmap no identificó inyección SQL en el parámetro `email` de `POST /api/users/login` con la configuración usada.

## Qué se hizo

Desde la VM Linux del laboratorio, el operador ejecutó sqlmap contra `http://192.168.77.10:8001/api/users/login` con un correo y una contraseña ficticios. Se seleccionó solo el parámetro JSON `email`, con `--risk=1 --level=1 --technique=BE --threads=1 --delay=0.2 --timeout=5 --retries=0` y límite externo de 180 segundos. No se solicitó extracción de datos ni modificación de la base.

La primera ejecución se detuvo ante HTTP 401, sin llegar a probar el parámetro. En la segunda se añadió `--ignore-code=401 --flush-session` para permitir la comprobación pese al rechazo esperado de las credenciales ficticias. La captura inicial es antecedente operativo; la segunda sustenta el resultado.

## Resultado observado

sqlmap informó que el contenido de la URL era estable, probó el parámetro JSON `email` y terminó con `all tested parameters do not appear to be injectable`. Registró 36 respuestas HTTP 401, coherentes con el login ficticio. No se observó acceso a datos ni elevación de privilegios.

El resultado solo corresponde a **este parámetro, ruta y configuración de sqlmap**. No demuestra ausencia de inyección SQL en todo el sistema. La lectura sin sesión de metadatos de auditoría ya demostrada en [RT-P04](RT-P04-acceso-sin-autenticacion.md) cubre el resultado de RT-03 previsto en la consigna; RT-P13 documenta un vector adicional que no produjo hallazgo.

## Evidencia

- [01 — ejecución detenida por 401](evidencias/RT-P13/01-sqlmap-401-inicial.png).
- [02 — análisis completo de `email`](evidencias/RT-P13/02-sqlmap-email-resultado.png).
- [Hashes SHA-256](evidencias/RT-P13/SHA256SUMS.txt).

Las capturas son originales aportadas por el operador. No se ha verificado independientemente el archivo de salida de sqlmap en la VM.

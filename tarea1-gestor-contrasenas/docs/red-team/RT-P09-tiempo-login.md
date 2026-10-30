# RT-P09 — Comparación exploratoria de tiempos del login

**Fecha:** 28/10/2026 (America/Montevideo). **Origen:** Burp Suite Community en el Windows principal, por túnel `127.0.0.1:18001` a la API en VM Linux. **Estado:** inconclusa; no se registra una vulnerabilidad confirmada.

## Objetivo y alcance

Explorar si `POST /api/users/login` permite distinguir por tiempo de respuesta entre un correo ficticio existente y otro inexistente, usando la misma contraseña deliberadamente incorrecta. Se hicieron solicitudes manuales y acotadas en Repeater. No se usaron credenciales reales ni automatización de intentos. Este caso es distinto de RT-P06, que estudia bloqueo o limitación de intentos.

## Procedimiento

Se enviaron cuerpos JSON con `email` igual a `prueba-login@example.com` (cuenta ficticia preparada) o `no-existe-rt@example.com`, y la misma contraseña falsa `IncorrectaSoloParaPrueba09`. El destino fue `127.0.0.1:18001`; no se envió token. Se observó el estado HTTP, el mensaje y el tiempo que Burp mostró en la esquina inferior derecha. Una solicitud intermedia escribió por error `nprueba-login@example.com`; se excluye de la comparación por ser otro correo inexistente.

## Resultado observado

| Correo usado | Estado y mensaje | Tiempos visibles en capturas |
|---|---|---|
| Existente ficticio | 401, `Credenciales inválidas.` | 1086 ms; 1032 ms |
| Inexistente ficticio | 401, `Credenciales inválidas.` | 83 ms; 1012 ms |

La primera respuesta de la cuenta existente también fue 401, pero la captura no mostró el tiempo y no se incluye en la comparación. El intento con el correo mal escrito devolvió 401 en 11 ms y tampoco se incluye. Las muestras de 1012, 1032 y 1086 ms se superponen prácticamente; **no se confirmó una diferencia temporal fiable**. La variabilidad de la VM, el túnel y el estado del servicio limita cualquier inferencia a partir de estas pocas muestras.

En el código de `control-central/app/api/users.py`, la rama de correo existente verifica el hash de la contraseña mientras que la rama de correo inexistente no lo hace. Esto justifica la hipótesis técnica, pero las capturas actuales no demuestran que una persona pueda clasificar usuarios de manera repetible a través de la red. No se asigna severidad ni ID de hallazgo.

## Evidencias

- [Inexistente, 83 ms](evidencias/RT-P09/01-inexistente-83ms.png).
- [Existente, 1086 ms](evidencias/RT-P09/02-existente-1086ms.png).
- [Inexistente, 1012 ms](evidencias/RT-P09/03-inexistente-1012ms.png).
- [Existente, 1032 ms](evidencias/RT-P09/04-existente-1032ms.png).
- [Hashes SHA-256](evidencias/RT-P09/SHA256SUMS.txt).

No se conserva un HAR ni una medición sistemática de latencia del servidor independiente de Burp. Si se retoma, se debe diseñar una medición repetible y controlar la variabilidad del entorno antes de calificar el riesgo; no se autorizan más intentos por esta ficha.

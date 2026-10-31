# RT-P14 — Envío SMTP ficticio al correo del laboratorio

**Fecha:** 31/10/2026. **Objetivo relacionado:** RT-04. **Estado:** comprobación parcial del entorno de pruebas, **sin vulnerabilidad confirmada**; no se demostró relay abierto ni suplantación de un servidor real.

## Límite del desarrollo evaluado

Este despliegue usa **Mailpit como bandeja de laboratorio**. La configuración del proyecto apunta el envío SMTP a `mailpit:1025`, con direcciones `@correo.local`; no incluye en este alcance un servidor de correo de producción ni un dominio público propio para comprobar sus registros DNS SPF/DKIM. Mailpit captura mensajes para pruebas y su aceptación de correo no equivale a que un servidor real permita reenviarlo hacia Internet.

Por esa razón, con el desarrollo y el entorno actuales **no es posible acreditar RT-04 completo**. Evaluar relay abierto, SPF/DKIM y entrega de mensajes falsificados a una casilla real requeriría un servidor y dominio de correo autorizados, configurados específicamente para esa evaluación. No se añadieron para esta prueba.

## Procedimiento

Con el backend y Mailpit activos en la VM Linux, el operador ejecutó Swaks 20240103.0 desde esa misma VM. Envió un único mensaje ficticio al SMTP del laboratorio en `192.168.77.10:1025`, sin credenciales, con remitente `noreply@correo.local` y destinatario `rsi@correo.local`:

```bash
swaks --server 192.168.77.10 --port 1025 --from noreply@correo.local --to rsi@correo.local --header "Subject: RT-04 prueba ficticia" --body "Mensaje de prueba enviado con Swaks dentro del laboratorio."
```

El remitente coincide con el valor predeterminado `smtp_from` del proyecto. No se cambió la configuración del servicio para esta prueba. El operador abrió la bandeja de Mailpit desde Windows mediante un túnel SSH local al puerto 8025.

## Resultado y límite

Swaks recibió `250 2.0.0 Ok: queued` después del contenido del mensaje. La bandeja de Mailpit mostró un único correo con ese remitente, destinatario, asunto y texto. Esto prueba que el receptor SMTP del laboratorio aceptó y almacenó el mensaje sin autenticación visible en la transcripción.

Mailpit es un receptor de pruebas diseñado para capturar mensajes. Esta observación **no prueba** que un servidor de producción permita relay a destinatarios externos, que existan errores SPF/DKIM, que el correo haya salido a Internet o que se haya falsificado una notificación real. Tampoco se probó un remitente distinto del configurado para la aplicación. La prueba es una observación funcional del laboratorio, **no un hallazgo de seguridad**. RT-04 permanece **parcial**.

## Evidencia

- [01 — transcripción de Swaks](evidencias/RT-P14/01-swaks-aceptado.png).
- [02 — correo en Mailpit](evidencias/RT-P14/02-mailpit-bandeja.png).
- [Hashes SHA-256](evidencias/RT-P14/SHA256SUMS.txt).

# RT-P06 — Intentos de login del control central

Actualización del 25/10/2026: se creó y verificó la cuenta ficticia `prueba-login@example.com`, id 1, admin por ser la primera cuenta. La evidencia de preparación está en `../evidencias/RT-P06/preparacion-cuenta.json`. No se ejecutaron intentos incorrectos.

La carpeta ahora incluye **00 — Control: login correcto** y las cinco solicitudes incorrectas. En 00 reemplazar `PEGAR_AQUI_LA_CLAVE_DEL_ARCHIVO_PRIVADO` por la clave del archivo privado entregado en el chat, fuera del repositorio. El correo ya está configurado en todas las solicitudes. Solo continuar con 01–05 si 00 responde 200; si falla, detenerse y revisar credenciales/servicio. No compartir ni exportar una copia con la contraseña completada; ocultar `access_token` en capturas. Los scripts no guardan tokens. Un login correcto reemplaza el token anterior de esa cuenta. Conservar MFA si se habilita posteriormente.

Estado al 25/10/2026: **colección preparada, no ejecutada**. Evalúa el login de la API, no «Abrir bóveda» ni Grafana. No es todavía un hallazgo confirmado.

La colección principal `TSI-RedTeam-Manual.postman_collection.json` incluye la carpeta RT-P06. Para importar solo este caso: `RT-P06.postman_collection.json`; luego puede arrastrarse la carpeta a la colección existente. No importar ambos si no se desean duplicados.

1. Backend disponible y `base_url` correcto. La preparación no lo inicia.
2. Usar una cuenta ficticia existente y autorizada. Confirmar por separado un login válido para saber que existe, conservando MFA si está activo. No crear cuentas con este caso ni usar cuentas personales.
3. En cada body visible, sustituir `prueba-login@example.com` por el correo ficticio verificado. Mantener las cinco contraseñas deliberadamente incorrectas. No se requiere token ni código TOTP para estos intentos incorrectos.
4. Ejecutar 01–05 manualmente, un Send por solicitud, esperando cada respuesta. No Runner ni bucles. Detenerse si hay 429, Retry-After, bloqueo, timeout o una demora importante. No superar cinco fallos en esta prueba.
5. Guardar estado HTTP, mensaje, hora UTC, tiempo y cabeceras relevantes. Registrar también cualquier fallo o protección encontrada. No capturar tokens o contraseñas válidas.

Los tests comprueban rechazo/no emisión de token; que pasen no prueba una vulnerabilidad. Cinco respuestas 401 acreditan cinco fallos rechazados, no intentos ilimitados. Los tiempos incluyen red, carga y hashing: no bastan por sí solos para demostrar ausencia de protección. Correlacionar con el código y comprobar controles externos antes de concluir falta de limitación. Una respuesta 200 con token ante una contraseña realmente incorrecta sería otro comportamiento que requiere investigación.

La guía no autoriza aumentar volumen ni probar indisponibilidad. No existen resultados, capturas ni severidad asignada para RT-P06 todavía.

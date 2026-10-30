# RT-P01 — Intentos fallidos de contraseña maestra y detección

Fecha: 21/10/2026. Estado: detección y auditoría verificadas; comprobación final de acceso pendiente.

## Objetivo y procedimiento

Comprobar que cinco intentos incorrectos de apertura de una bóveda ficticia se registran con firma válida y generan la alerta de correlación prevista.

El usuario ejecutó el gestor en su Windows principal. Confirmó acceso con la contraseña correcta y visualización de las credenciales antes de cerrar la bóveda. Después realizó cinco intentos de apertura con contraseñas incorrectas, esperando la respuesta entre intentos. No se usó la función de crear bóveda. Los eventos llegaron al control central alojado en Linux mediante el túnel local; la VM Windows no intervino en esta ejecución.

## Resultado comprobado

- PostgreSQL contiene cinco eventos `intento_fallido_maestra`, IDs 5 a 9, todos con `firma_valida=true`.
- Los cinco eventos están en el archivo de auditoría leído por Wazuh.
- Intervalo de recepción: 08:51:40.253699 a 08:52:12.455691, hora de Montevideo (UTC−03). Duración aproximada: 32,2 segundos, dentro de los dos minutos previstos.
- Wazuh generó cuatro alertas de regla base `100100` y, al quinto evento, la alerta real `100101`, nivel 10: «Fuerza bruta de contraseña maestra: 5 intentos del mismo agente en 2 minutos».
- La alerta de correlación tiene timestamp `2026-10-21T11:52:12.481+0000` y referencia el evento 9 junto con los cuatro anteriores.

La correlación y la auditoría funcionaron para esta ejecución. Este resultado acredita un control efectivo; no constituye por sí mismo una vulnerabilidad ni un bloqueo automático del acceso por parte de Wazuh.


## Evidencias

Carpeta: `evidencias/RT-P01/`.

- `eventos-postgresql.json`: exportación de los cinco registros, sin firmas JWT ni contraseñas.
- `auditoria-original.jsonl`: líneas originales de auditoría correspondientes a esos eventos.
- `alertas-wazuh-original.jsonl`: líneas originales de las alertas base y de correlación.
- `SHA256SUMS.txt`: hashes de los archivos anteriores para comprobar integridad.

Los registros conservan sus identificadores y campos originales. No se modificaron las evidencias para cambiar su procedencia.

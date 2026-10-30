# RT-P05 — Reconfiguración incompleta desactiva TOTP

Fecha local: **24/10/2026**, America/Montevideo. Hora exacta no registrada. Hallazgo **RT-H03**, según el inventario vigente. Reproducción manual del usuario en su Windows principal sobre la bóveda ficticia y vacía `prueba-mfa.sqlite`.

## Procedimiento y resultado

1. Crear una bóveda exclusiva de prueba, generar TOTP y confirmarlo con el autenticador. Aparece «TOTP activo. La próxima apertura pide el código de la app».
2. Cerrar e intentar abrir con contraseña correcta y TOTP vacío. La app rechaza: «Esta bóveda pide el código TOTP».
3. Abrir con contraseña y código vigente.
4. Pulsar nuevamente «Generar TOTP», no confirmar el candidato y cerrar la bóveda mediante «Cerrar».
5. Abrir con contraseña sin código. El usuario informa que la apertura fue exitosa y aporta captura de la misma bóveda abierta.

**Resultado:** abandonar la reconfiguración deja de exigir el segundo factor anterior. No se pulsó un botón específico de cancelar: se abandonó mediante cierre de la bóveda.

## Evidencias y límites

Carpeta `evidencias/RT-P05/`:

- `01-totp-activo.png`: confirmación inicial de activación.
- `02-rechazo-sin-totp.png`: campo TOTP vacío y rechazo antes de reconfigurar.
- `03-apertura-posterior.png`: misma bóveda abierta tras abandonar.
- `SHA256SUMS.txt`: integridad de las tres capturas originales copiadas sin modificar.

La última imagen muestra apertura, no los campos introducidos anteriormente. La ausencia de código en ese paso consta en el relato del usuario y la secuencia guiada; falta captura previa al último clic o video continuo para reforzarlo. No se presenta como prueba automatizada ni consulta de la base. No se archivó la imagen intermedia que contiene QR. No se guardan contraseñas ni secretos en este informe.

## Causa y alcance

En `cliente-gestor/src/App.tsx`, «Generar TOTP» invoca `enrolar_totp`. En `cliente-gestor/src-tauri/src/vault/store.rs`, ese método reemplaza el secreto y persiste `UPDATE meta SET totp_secreto = ?1, totp_activo = 0 WHERE id = 1` antes de confirmar. Abandonar deja TOTP inactivo.

Requiere acceso previo a una bóveda abierta. La contraseña maestra sigue siendo necesaria para abrirla después. No demuestra acceso a una bóveda ajena cerrada, extracción de secretos o privilegios del sistema operativo. La variante similar encontrada en el backend no fue ejecutada en este caso.

## Recomendación y pendientes

Mantener secreto y estado anteriores hasta confirmar el candidato, guardarlo por separado y sustituirlo atómicamente. Exigir reautenticación adecuada antes de cambiar factores. Abandonar debe conservar el MFA previo.

CVSS, CWE, MITRE, registro de incidente y valoración de riesgo pendientes. Sin corrección ni retest. Restauración del TOTP de la bóveda ficticia pendiente de confirmación del usuario.

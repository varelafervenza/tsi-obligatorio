# RT-P07 — Bóveda abierta tras 11 minutos sin interacción

## Objetivo y alcance

Comprobar si el cliente bloquea automáticamente una bóveda que permanece abierta e inactiva. El cliente se ejecutó en el Windows principal del usuario con la bóveda ficticia `prueba-mfa.sqlite`, sin credenciales guardadas; no se consultaron secretos.

## Procedimiento y resultado observado

El usuario abrió la bóveda y aportó una captura con el reloj del sistema a las **22:39 del 25/10/2026**. Luego dejó la app abierta sin interactuar y aportó otra captura a las **22:50 del mismo día**. En ambas se ve «Bóveda abierta» y la misma ruta. Transcurrieron al menos **11 minutos según el reloj mostrado** sin un cierre automático visible.

Las capturas originales se conservaron en [evidencias/RT-P07](evidencias/RT-P07/) con hashes SHA-256. Los archivos muestran el estado de la pantalla, pero no prueban de manera independiente la ausencia de toda interacción entre ambas capturas. El relato del usuario aporta esa condición.

## Interpretación y límite

El cliente mantiene la bóveda en memoria hasta la acción «Cerrar» (`cliente-gestor/src/App.tsx` y `src-tauri/src/vault/commands.rs`); en los archivos revisados no se identificó un temporizador de inactividad. El resultado confirma que la bóveda **permanecía abierta al cabo de 11 minutos en esta ejecución**. No demuestra una duración ilimitada ni que se pudiera consultar una credencial, porque esta bóveda estaba vacía.

Si otra persona accede a una sesión de Windows desbloqueada con una bóveda que contiene credenciales abiertas, podría usar la interfaz mientras siga abierta. Ese escenario y su gravedad dependen del control de acceso físico y del bloqueo de Windows; no se asigna aún una severidad CVSS.

## Recomendación

Definir un plazo de inactividad para la bóveda, cerrarla al vencerlo y exigir de nuevo los factores configurados para abrirla. Después de una corrección autorizada, repetir la prueba con una credencial ficticia y el plazo de bloqueo acordado.

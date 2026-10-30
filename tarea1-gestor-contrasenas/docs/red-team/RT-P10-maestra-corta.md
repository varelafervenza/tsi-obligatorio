# RT-P10 — Contraseña maestra de un carácter aceptada

Fecha local: **29/10/2026**, America/Montevideo. Prueba manual del cliente de escritorio en Windows principal, con una bóveda ficticia vacía. No se usaron credenciales reales ni se intentó descifrar el archivo fuera de la app.

## Objetivo y procedimiento

Comprobar si el gestor exige una longitud mínima para la contraseña maestra de una bóveda nueva. El usuario creó la bóveda de prueba `boveda-passcorta.sqlite` y declaró haber usado `a` como contraseña maestra. Cerró la bóveda y volvió a abrirla con `a`.

## Resultado y evidencia

- [04-nueva-boveda-un-caracter.png](evidencias/RT-P10/04-nueva-boveda-un-caracter.png): ruta de la nueva bóveda y un único carácter oculto en el campo «Contraseña maestra» antes de abrir.
- [05-boveda-abierta-con-maestra-corta.png](evidencias/RT-P10/05-boveda-abierta-con-maestra-corta.png): «Bóveda abierta» para la misma ruta, sin credenciales guardadas.
- El archivo existe y comienza con `BOV2`, el formato cifrado esperado. La imagen muestra la longitud de un carácter, pero no permite ver de manera independiente que el carácter era `a`; eso consta por declaración del usuario.

Las capturas `01`–`03` de esta carpeta corresponden a un **intento anterior descartado**: el usuario aclaró que había pulsado otra tecla al crear `boveda-contracorta.sqlite`. Sus rechazos de apertura no son evidencia de corrupción ni de esta debilidad. Se conservan con sus hashes para mantener la trazabilidad, pero no se usan para la conclusión. Los hashes de todas las capturas están en [SHA256SUMS.txt](evidencias/RT-P10/SHA256SUMS.txt).

## Interpretación y límite

La validación de la maestra en `cliente-gestor/src-tauri/src/vault/store.rs` rechaza únicamente una cadena vacía. La prueba manual confirma que la app acepta una maestra de un carácter y permite volver a abrir la bóveda con ella. Esto debilita la resistencia frente a intentos de adivinar la contraseña **si un tercero obtiene una copia del archivo cifrado**. No se demostró obtención del archivo por un tercero, descifrado fuera de la app ni acceso a contraseñas reales. No se asigna aún severidad CVSS.

## Recomendación

Exigir una maestra suficientemente larga al crear y cambiar la contraseña, explicar la importancia de una frase única y comprobar ambas rutas después de la corrección. La longitud mínima concreta debe definirse con la política del proyecto.

# RT-H04 — Token de sesión del panel sin vencimiento temporal

Fecha de documentación: **25/10/2026**, America/Montevideo. Estado: **identificado mediante revisión de código; demostración HTTP temporal pendiente**. No corresponde a la firma JWS de los eventos, que es un mecanismo diferente.

## Descripción y evidencia técnica

El token del panel es un valor opaco generado con `secrets.token_urlsafe(32)`. El servidor almacena su SHA-256 y autentica comparándolo con `User.token_hash`. En los archivos revisados no existe fecha de emisión/vencimiento ni comprobación de antigüedad o inactividad.

Fuentes del proyecto:

- `control-central/app/api/users.py`, `_emitir_token`: genera y reemplaza el hash almacenado; `login` lo emite después de comprobar las credenciales y el TOTP cuando está habilitado.
- `control-central/app/core/auth.py`, `usuario_actual`: acepta el token cuando su hash coincide con un usuario, sin condición temporal.
- `control-central/app/models/user.py`, `User`: almacena `token_hash`, sin campos de expiración de sesión.
- `control-central/app/main.py`: en el código revisado no se aplica middleware global de vencimiento de sesiones.

## Impacto y condiciones

Si un tercero obtiene un token válido y puede acceder a la API, puede reutilizar la sesión con los permisos de esa cuenta sin repetir contraseña ni MFA mientras el hash siga vigente. La falta de caducidad amplía esa ventana. No se demostró robo de tokens, acceso desde Internet ni privilegios de administrador. El token no permite por sí solo descifrar bóvedas del cliente.

**Un nuevo login exitoso del mismo usuario reemplaza el token y deja inválido el anterior.** También podría invalidarse cambiando/eliminando su hash mediante una operación administrativa. Por tanto, no se afirma que sea irrevocable ni que sobreviva necesariamente a cualquier cambio de estado. La fortaleza aleatoria del token y su almacenamiento mediante hash no son el hallazgo.

## Estado de validación y límites

Esta entrada documenta análisis estático, no una prueba de duración ejecutada. No se guardaron tokens, contraseñas ni respuestas HTTP como evidencia de este caso. No se inició el proyecto, no se hicieron logins ni se modificaron cuentas. No se verificó que el binario o contenedor instalado coincida con esta revisión del código.

La documentación de acceso describe autenticación y MFA, pero no se identificó un plazo de sesión acordado en las fuentes revisadas. Debe acordarse antes de definir el criterio temporal de aceptación. Que un token dure unos minutos no acredita por sí solo una vulnerabilidad.

## Verificación futura propuesta — no ejecutada

1. Usar una cuenta ficticia, registrar hora de login y conservar el token de forma privada.
2. Consultar una ruta protegida y guardar el resultado.
3. Sin volver a iniciar sesión, repetir después del plazo acordado. Registrar el intervalo real; una espera finita solo demuestra vigencia durante ese intervalo.
4. Hacer nuevo login y comprobar que el token previo es rechazado, como control de reemplazo.

No modificar el reloj para forzar resultados. Ocultar tokens en capturas y exportaciones destinadas al informe.

## Recomendación y seguimiento

Definir y aplicar un máximo de duración de sesión y, si corresponde, de inactividad. Guardar y validar las fechas en el servidor; incorporar cierre/revocación y reautenticación para acciones sensibles. El plazo concreto debe responder a la política acordada.

Severidad, CVSS/vector, CWE y MITRE: **pendientes**, sin clasificación automática como alta. Responsable, plazo de corrección, incidente y riesgo asociados: pendientes. Sin corrección ni retest.

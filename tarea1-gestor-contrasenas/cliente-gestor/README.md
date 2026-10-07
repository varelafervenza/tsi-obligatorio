# cliente-gestor — Gestor de contraseñas offline

App de escritorio (Tauri: Rust + frontend web) que corre 100% local. No requiere red para el uso cotidiano (RNF-01). Solo emite eventos firmados hacia `control-central` cuando hay conexión disponible (RF-08).

## Estructura

| Ruta | Responsabilidad |
|---|---|
| `src-tauri/src/crypto/` | KDF (Argon2id) y cifrado AEAD (XChaCha20-Poly1305) de la bóveda. RF-01, RF-16. |
| `src-tauri/src/vault/` | Apertura/cierre de bóveda, CRUD de credenciales, historial, import/export cifrado. RF-02, RF-03, RF-12. |
| `src-tauri/src/auth/` | MFA local: TOTP (RFC 6238) y WebAuthn/Windows Hello. RF-11 (elección de hash se define en control-central). |
| `src-tauri/src/events/` | Firma JWS de alta, modificación, borrado, cambio de maestra, intento fallido y vencimiento. RF-07, RF-08, RF-17. |
| `src-tauri/src/generator/` | Generador de contraseñas/passphrase + validación por regex definida por sistema. RF-04, RF-05. |
| `src/` | Frontend (React + TypeScript): UI de bóveda, definición de políticas por sistema, buscador/filtros. RF-15. |

## Estado

- [x] Esquema SQLite y CRUD: crear, abrir, cerrar, alta, consulta, modificación y borrado. `vault/store.rs`. En disco el archivo va cifrado (`BOV2`). El historial se usa para rechazar reuso; la pantalla no lo lista.
- [x] Argon2id con parámetros configurables (por defecto 19 MiB / 2 iteraciones). `crypto/kdf.rs`.
- [x] Cifrado XChaCha20-Poly1305 de cada entrada (nonce nuevo, tag verificado). `crypto/cipher.rs`.
- [x] TOTP local: la bóveda muestra un QR, se confirma con el código de la app y la próxima apertura lo exige. `auth/totp.rs` y `src/App.tsx`. WebAuthn/Windows Hello siguen sin implementar.
- [x] Cliente HTTP: firma JWS RS256 y POST al control central, con cola local si no hay red. `events/`.
- [x] Cambio de la maestra: reencripta secretos, notas e historial y emite `cambio_maestra`. `vault/store.rs`.
- [x] Pantalla mínima: crear/abrir bóveda y alta, consulta, edición y borrado. `src/App.tsx`.
- [x] Generador (contraseña y frase) y política por sistema: longitud, caracteres, regex, historial y vencimiento. `generator/` y `vault/store.rs`.
- [x] Buscador (sistema, usuario, categoría), filtro de favoritos y de vencidas, y marca de favorito. `src/App.tsx`.
- [x] Exportar e importar la bóveda en un archivo cifrado con contraseña de transporte (`.gex`). `vault/store.rs`.
- [x] Aviso de vencimiento local y evento `vencimiento_credencial` al abrir la bóveda (RF-17). `vault/commands.rs`.
- [x] Delay al fallar la maestra: 1 s, 2 s, 4 s y después 8 s (RF-16). Se pone en cero si la apertura es correcta.

## Cómo levantar

Hace falta Node y Rust (`rustc` / `cargo`). En Windows, si `cargo` no se reconoce, abrí una terminal nueva después de instalar Rust.

Desde `cliente-gestor`, cada comando en su propia línea:

```powershell
$env:NODE_OPTIONS = "--use-system-ca"
npm install
npm run tauri dev
```

Si las dos primeras quedan pegadas en una sola línea, Windows responde que el nombre de archivo no es válido. `NODE_OPTIONS` evita el error `UNABLE_TO_VERIFY_LEAF_SIGNATURE` contra el registry de npm en esta red. Vite queda en 5.4 (el plugin de React de este proyecto no acepta Vite 8). La ventana es el gestor. El archivo de la bóveda, si no cambiás la ruta, es `boveda-prueba.sqlite` en la carpeta de datos de la app.

Si `cargo` responde `Acceso denegado` al crear `src-tauri\target`, creá esa carpeta a mano y volvé a correr `npm run tauri dev`. Un aviso de "incremental compilation" en una ruta con `ñ` no frena el ejecutable.

Tests automáticos de cripto y bóveda:

```powershell
cd src-tauri
cargo test
```

## Cómo probar la ventana

1. Dejá la ruta del archivo. Poné una contraseña maestra y tocá **Crear bóveda**. Puede tardar cerca de un segundo.
2. En **Alta**, cargá sistema, usuario y contraseña. **Agregar**. Tiene que aparecer en la lista.
3. Elegí la credencial. **Ver** muestra la contraseña. Cambiala y **Guardar cambios**. Volvé a elegirla y confirmá el valor nuevo.
4. **Cerrar** y **Abrir bóveda** con la misma maestra. La credencial sigue ahí.
5. Cerrá y abrí con otra maestra. Tiene que rechazarla y no mostrar credenciales.
6. Abrí `boveda-prueba.sqlite` con un editor de texto o con DB Browser. El archivo empieza con `BOV2` y no se abre como una base SQLite. No tienen que aparecer la contraseña, el sistema ni el usuario.
7. Con una credencial seleccionada, **Borrar** la saca de la lista.

## Política y generador

Abrí **Política de este sistema** en el formulario. Elegí modo contraseña o frase, longitud, caracteres y, si hace falta, una expresión regular. **Generar** llena el campo de contraseña. **Guardar política** hace que las próximas altas de ese sistema respeten la regla, el historial y los días de validez. Si vence, la lista lo muestra al lado del usuario.

## Eventos al control central

Docker Desktop tiene que estar abierto (el ícono de la ballena en marcha). Si el motor está apagado, `docker compose` dice que no encuentra `dockerDesktopLinuxEngine`.

Desde `tarea1-gestor-contrasenas\infra`:

```powershell
docker compose up --build -d
```

La API queda en el puerto 8001 (8000 dentro del contenedor) y Mailpit en `http://localhost:8025`. Comprobación: `http://localhost:8001/healthz` tiene que responder `status: ok`. En esta red el build de la imagen necesita `--trusted-host` de pip; ya está en `control-central/Dockerfile`.

En la ventana:

1. Abrí **Control central**.
2. Dejá la URL `http://localhost:8001/api/events/`.
3. En la carpeta de claves poné la ruta absoluta a `tarea1-gestor-contrasenas\keys\agentes`.
4. **Guardar y copiar clave pública**. El aviso tiene que decir que copió `{agente-id}.pub.pem`. Docker monta esa carpeta en solo lectura: no hace falta reiniciar el contenedor.
5. Abrí la bóveda y agregá una credencial. El aviso tiene que decir que el evento se envió.
6. En Mailpit tiene que llegar el correo de alta. En `http://localhost:8001/api/events/` el último evento tiene que traer `firma_valida: true`.

Si Docker está apagado, el alta igual se guarda. El aviso dice que el evento quedó en cola. Al volver a haber red, el próximo alta, cambio o borrado reintenta la cola.

Una maestra incorrecta al abrir (el archivo ya existe) emite `intento_fallido_maestra` y la ventana espera 1 s la primera vez, 2 s la segunda, 4 s la tercera y 8 s de ahí en más. Abrir con la maestra correcta pone esa espera en cero. **Cambiar contraseña maestra**, con la bóveda abierta, reencripta las credenciales y emite `cambio_maestra`.

Si al abrir hay credenciales vencidas, la lista lo dice y sale un evento `vencimiento_credencial` por cada sistema vencido. Mailpit recibe ese correo igual que un alta.

Con la bóveda abierta, **TOTP de esta bóveda** y **Generar TOTP** muestran un QR. Escanealo con una app de autenticación y escribí el código de 6 dígitos en **Código para confirmar**. La próxima apertura pide ese código además de la maestra. La pantalla de inicio no lo calcula. Si la cámara no lee el QR, **No puedo escanear** muestra el secreto para cargarlo a mano. El secreto no sale hacia el control central.

## Buscador, favoritos y copia

Con la bóveda abierta, el cuadro **Buscar** filtra por sistema, usuario o categoría. El filtro **Favoritos** o **Vencidas** achica la lista. La estrella al lado de cada credencial la marca o la saca.

**Copia cifrada** exporta un `.gex` con una contraseña de transporte (no es la maestra). En otra bóveda, la misma ruta y la misma contraseña de transporte con **Importar** trae las credenciales. El archivo no tiene que contener el secreto en claro.

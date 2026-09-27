# cliente-gestor — Gestor de contraseñas offline

App de escritorio (Tauri: Rust + frontend web) que corre 100% local. No requiere red para el uso cotidiano (RNF-01). Solo emite eventos firmados hacia `control-central` cuando hay conexión disponible (RF-08).

## Estructura

| Ruta | Responsabilidad |
|---|---|
| `src-tauri/src/crypto/` | KDF (Argon2id) y cifrado AEAD (XChaCha20-Poly1305) de la bóveda. RF-01, RF-16. |
| `src-tauri/src/vault/` | Apertura/cierre de bóveda, CRUD de credenciales, historial, import/export cifrado. RF-02, RF-03, RF-12. |
| `src-tauri/src/auth/` | MFA local: TOTP (RFC 6238) y WebAuthn/Windows Hello. RF-11 (elección de hash se define en control-central). |
| `src-tauri/src/events/` | Construcción y firma JWS de eventos de auditoría (alta/mod/borrado/cambio de maestra). RF-07, RF-08. |
| `src-tauri/src/generator/` | Generador de contraseñas/passphrase + validación por regex definida por sistema. RF-04, RF-05. |
| `src/` | Frontend (React + TypeScript): UI de bóveda, definición de políticas por sistema, buscador/filtros. RF-15. |

## Pendiente (no implementado aún)

- [ ] Definir esquema SQLite/SQLCipher de la bóveda.
- [ ] Implementar Argon2id con parámetros configurables (memoria/iteraciones).
- [ ] Implementar cifrado XChaCha20-Poly1305 de cada entrada.
- [ ] Flujo de enroll TOTP + WebAuthn (Windows Hello como authenticator de plataforma).
- [ ] Cliente HTTP para POST de eventos firmados a `control-central` (con cola local si no hay red).
- [ ] UI: alta/consulta/edición/borrado, definición de regex por sistema, vencimientos, categorías, favoritos.

## Cómo correr (una vez implementado)

```bash
npm install
npm run tauri dev
```

"""Verificación de firma JWS de eventos y hashing de contraseñas de usuarios del panel.
TODO: verificación con clave pública por agente (no HMAC compartida global; ver
cliente-gestor/src-tauri/src/events/signer.rs) y selección Argon2id/bcrypt según
Settings.hash_algorithm.
"""

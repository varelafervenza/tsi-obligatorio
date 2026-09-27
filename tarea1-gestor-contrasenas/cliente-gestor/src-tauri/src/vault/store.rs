//! Acceso al archivo de bóveda (SQLite embebido). Cada fila de credencial se guarda
//! ya cifrada por `crypto::cipher`; este módulo nunca ve texto plano de contraseñas
//! ajenas a la operación en curso.
//! TODO: esquema (credenciales, historial, políticas por sistema, categorías).

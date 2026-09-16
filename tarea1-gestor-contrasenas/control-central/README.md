# control-central — API de auditoría, usuarios y dashboard

Servicio REST (FastAPI) de la organización. Recibe **eventos firmados** (nunca secretos),
los registra para el SIEM, dispara notificaciones por correo y expone datos para el
dashboard. Es **zero-knowledge**: nunca puede descifrar una bóveda de cliente.

## Estructura

| Ruta | Responsabilidad |
|---|---|
| `app/api/events.py` | Recibe y verifica eventos JWS del cliente (alta/mod/borrado/cambio de maestra). RF-08. |
| `app/api/users.py` | Usuarios del control central, roles, MFA (TOTP/WebAuthn/Windows Hello), elección de Argon2id/bcrypt. RF-11. |
| `app/api/dashboard.py` | Endpoints de KPIs/alertas/incidentes/estado de agentes para Grafana. RF-09, RF-14. |
| `app/core/security.py` | Verificación de firma JWS, hashing de contraseñas de usuarios del panel. |
| `app/core/mfa.py` | Lógica de MFA del control central. |
| `app/models/` | `Event`, `User`, `Incident` (SQLAlchemy). |
| `app/db/` | Sesión de PostgreSQL + migraciones (Alembic). |
| `app/siem/wazuh_forwarder.py` | Reenvío/normalización de eventos hacia Wazuh (o Security Onion). RF-10. |

## Pendiente (no implementado aún)

- [ ] Modelo de datos definitivo de `Event` (tipo, sistema, agente, timestamp, firma, resultado de verificación).
- [ ] Verificación JWS con clave pública por agente (no HMAC compartida global).
- [ ] Disparo de correo (Mailu/Postfix) en alta/mod/borrado/cambio de maestra. RF-07.
- [ ] Reglas de detección reenviadas a Wazuh: fuerza bruta de maestra, borrado masivo, cambio de maestra. RF-10.
- [ ] RBAC de usuarios del panel + enroll TOTP/WebAuthn/Windows Hello. RF-11.
- [ ] Retención de eventos ≥ 90 días (RNF-07) y purga programada.

## Cómo correr (una vez implementado)

```bash
python -m venv venv && source venv/bin/activate  # o venv\Scripts\activate en Windows
pip install -r requirements.txt
uvicorn app.main:app --reload
```

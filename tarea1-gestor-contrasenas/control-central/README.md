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

- [x] Modelo y persistencia de `audit_events` (`POST/GET /api/events/`).
- [x] Forwarder JSONL a `infra/logs/audit-events.jsonl` (Wazuh lo consume después).
- [x] Verificación JWS RS256 con clave pública por agente (`firma_valida` true/false).
- [x] Correo SMTP en alta/mod/borrado/cambio de maestra (Mailpit en H2; Mailu después).
- [ ] Reglas de detección en Wazuh (RF-10) sobre ese JSONL.
- [ ] RBAC de usuarios del panel + enroll TOTP/WebAuthn/Windows Hello. RF-11.
- [ ] Retención de eventos ≥ 90 días (RNF-07) y purga programada.

## Cómo correr

Stack del H2 (API + Postgres + Grafana + Mailpit; TheHive no arranca):

```bash
cd ../infra
docker compose up --build -d
curl http://localhost:8000/healthz
# esperado: {"status":"ok","database":"up"}

# Una vez (en la raíz de tarea1-gestor-contrasenas); la privada no se commitea:
#   pip install python-jose[cryptography]
#   python scripts/generar_par_agente.py
# Luego recrear el compose para montar keys/agentes/*.pub.pem
python ../scripts/generar_evento_prueba.py
# esperado: firma_valida true. Con --sin-firma queda false (TODO).
curl http://localhost:8000/api/events/
# Una línea JSON nueva en infra/logs/audit-events.jsonl (Get-Content).
# Correo de prueba: http://localhost:8025  (Mailpit; Mailu se apunta después).
```

Sin Docker (API local contra Postgres del compose; puerto 5432 publicado):

```bash
python -m venv venv && venv\Scripts\activate
pip install -r requirements.txt
uvicorn app.main:app --reload
```
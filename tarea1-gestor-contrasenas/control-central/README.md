# control-central — API de auditoría, usuarios y dashboard

Servicio REST (FastAPI) de la organización. Recibe **eventos firmados** (nunca secretos),
los registra para el SIEM, dispara notificaciones por correo y expone datos para el
dashboard. Es **zero-knowledge**: nunca puede descifrar una bóveda de cliente.

## Estructura

| Ruta | Responsabilidad |
|---|---|
| `app/api/events.py` | Recibe y verifica eventos JWS (alta, modificación, borrado, cambio de maestra, intento fallido y vencimiento). RF-08, RF-17. |
| `app/api/incidents.py` | Alta, listado y cambio de estado de incidentes a partir de un evento. RF-13. |
| `app/api/users.py` | Usuarios del panel, roles, TOTP y elección de Argon2id/bcrypt. RF-11. |
| `app/api/dashboard.py` | `GET /api/dashboard/kpis`: agentes, correo, volumen, MTTD, MTTR, cobertura y uptime. RF-09, RF-14. |
| `app/core/security.py` | Verificación de firma JWS por agente. |
| `app/core/mfa.py` | TOTP RFC 6238 del panel. WebAuthn y Windows Hello no tienen endpoint. |
| `app/models/` | `AuditEvent`, `User` (`panel_users`), `Incident`. |
| `app/db/` | Sesión de PostgreSQL. Las tablas se crean con `create_all` al arrancar. |
| `app/siem/wazuh_forwarder.py` | Una línea JSON por evento en `infra/logs/audit-events.jsonl`. RF-10. |

## Estado

- [x] Modelo y persistencia de `audit_events` (`POST/GET /api/events/`).
- [x] Verificación JWS RS256 con `keys/agentes/{agente_id}.pub.pem` (una clave por agente). Si falla, se persiste con `firma_valida=false`.
- [x] Forwarder JSONL a `infra/logs/audit-events.jsonl` (Wazuh lo consume después).
- [x] Correo SMTP en alta, modificación, borrado, cambio de maestra y vencimiento (Mailpit; Mailu después).
- [x] Incidentes `abierto` / `en_analisis` / `resuelto` ligados a un evento (`/api/incidents/`).
- [x] Panel `GET /api/dashboard/kpis` y tablero Grafana **Control central**.
- [x] Reglas escritas en `infra/wazuh/local_rules.xml`. Falta dispararlas con el manager levantado.
- [x] Usuarios del panel: primer alta sin token (queda admin), hash Argon2id o bcrypt, login y TOTP. WebAuthn y Windows Hello siguen sin endpoint.
- [x] Alertas locales con los mismos umbrales que `local_rules.xml` (`GET/PATCH /api/alerts/`). El manager de Wazuh sigue sin levantar.
- [x] Retención: al arrancar se borran eventos de más de 90 días que no sean origen de un incidente o una alerta (RNF-07).

## Cómo correr

Stack del H2 (API + Postgres + Grafana + Mailpit; TheHive no arranca):

```bash
# Primera vez, en infra/:
copy .env.example .env
copy control-central.env.example control-central.env

# Desde la raíz de tarea1-gestor-contrasenas:
docker compose up --build -d
# o: cd infra && docker compose up --build -d

curl http://localhost:8000/healthz
curl http://localhost:8000/api/dashboard/kpis
# Grafana: http://localhost:3000  (tablero Control central; claves en la copia local de .env)
# Mailpit: http://localhost:8025
```

Evento firmado, desde la raíz de `tarea1-gestor-contrasenas`. La privada no se versiona.

```bash
pip install python-jose[cryptography]
python scripts/generar_par_agente.py
python scripts/generar_evento_prueba.py
# firma_valida true. Con --sin-firma queda false.
curl http://localhost:8000/api/events/
```

El primer usuario del panel no lleva token y queda admin. El login devuelve el token para el enroll TOTP.

```bash
curl -X POST http://localhost:8000/api/users/ -H "Content-Type: application/json" -d "{\"email\":\"rsi@correo.local\",\"password\":\"clave-panel-1\",\"algoritmo_hash\":\"argon2id\"}"
curl -X POST http://localhost:8000/api/users/login -H "Content-Type: application/json" -d "{\"email\":\"rsi@correo.local\",\"password\":\"clave-panel-1\"}"
```

Sin Docker (API local contra Postgres del compose; puerto 5432 publicado):

```bash
python -m venv venv && venv\Scripts\activate
pip install -r requirements.txt
uvicorn app.main:app --reload
```
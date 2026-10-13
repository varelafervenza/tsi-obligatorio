"""Autenticación por token del panel, compartida entre routers.

Antes vivía solo en `app/api/users.py`. `incidents.py` y `alerts.py` no la usaban: cualquiera
que llegara a la API podía crear incidentes falsos, marcar un incidente real como "resuelto"
(apagando la alerta de un ataque en curso) o reclasificar cualquier alerta como falso positivo,
sin ningún token. Hallazgo del 12/10/2026, antes del tag `v1.0`.
"""
import hashlib

from fastapi import Depends, Header, HTTPException
from sqlalchemy import select
from sqlalchemy.orm import Session

from app.db.session import get_db
from app.models.user import User


def usuario_actual(
    authorization: str | None = Header(default=None),
    db: Session = Depends(get_db),
) -> User:
    if not authorization or not authorization.lower().startswith("bearer "):
        raise HTTPException(status_code=401, detail="Falta el token.")
    token = authorization.split(" ", 1)[1].strip()
    huella = hashlib.sha256(token.encode("utf-8")).hexdigest()
    usuario = db.scalar(select(User).where(User.token_hash == huella))
    if usuario is None:
        raise HTTPException(status_code=401, detail="Token inválido.")
    return usuario


def exigir_admin(usuario: User) -> None:
    if usuario.rol != "admin":
        raise HTTPException(status_code=403, detail="Solo un admin puede hacer esto.")

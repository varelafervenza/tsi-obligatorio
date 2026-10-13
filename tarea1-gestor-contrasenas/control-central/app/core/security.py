"""Verificación JWS de eventos: una clave pública por agente (no HMAC global)."""
from pathlib import Path

from jose import JWTError, jwt
from jose.exceptions import JOSEError

from app.core.config import settings

ALGORITMO = "RS256"
# INVARIANTE DE SEGURIDAD — no agregar otro algoritmo a esta constante ni a la lista de abajo.
# python-jose tiene una falla de confusión de algoritmo (CVE-2026-85394, V08 en
# 10-Gestion-Vulnerabilidades.md): si `jwt.decode` permite más de un algoritmo (por ejemplo
# RS256 y HS256 juntos), alguien con la clave PÚBLICA de un agente puede forjar un token HS256
# usándola como si fuera el secreto HMAC, sin la clave privada. Mientras `algorithms` tenga un
# solo elemento, `jose` rechaza cualquier token que declare un `alg` distinto, antes de intentar
# verificar la firma. Probado el 12/10/2026 con un token HS256 armado a mano: rechazado.


def _public_key_path(agente_id: str) -> Path:
    seguro = Path(agente_id).name
    return Path(settings.jwt_public_keys_dir) / f"{seguro}.pub.pem"


def verificar_jws(
    firma_jws: str,
    *,
    agente_id: str,
    tipo: str,
    sistema: str,
    timestamp_iso: str,
) -> bool:
    path = _public_key_path(agente_id)
    if not path.is_file():
        return False
    try:
        claims = jwt.decode(
            firma_jws,
            path.read_text(encoding="utf-8"),
            algorithms=[ALGORITMO],
        )
    except (JWTError, JOSEError, ValueError):
        return False
    return (
        claims.get("agente_id") == agente_id
        and claims.get("tipo") == tipo
        and claims.get("sistema") == sistema
        and claims.get("ts") == timestamp_iso
    )

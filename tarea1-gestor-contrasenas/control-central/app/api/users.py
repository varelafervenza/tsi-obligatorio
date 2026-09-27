"""Usuarios del control central: roles, MFA (TOTP/WebAuthn/Windows Hello) y
elección de algoritmo de hash (Argon2id/bcrypt). RF-11.
"""
from fastapi import APIRouter

router = APIRouter()


@router.get("/")
def listar_usuarios():
    # TODO: RBAC (admin, auditor, RSI); no exponer secretos de MFA.
    raise NotImplementedError

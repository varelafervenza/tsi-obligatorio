"""Usuarios del panel: roles, hash elegible y TOTP. RF-11.

El primer usuario se crea sin token y queda como admin. Los siguientes los crea un admin.
Los eventos del agente siguen entrando por JWS, sin esta sesión.
"""
import hashlib
import secrets
from typing import Literal

from fastapi import APIRouter, Depends, Header, HTTPException
from passlib.hash import argon2, bcrypt
from pydantic import BaseModel, ConfigDict, Field
from sqlalchemy import func, select
from sqlalchemy.exc import IntegrityError
from sqlalchemy.orm import Session

from app.core.config import settings
from app.core.mfa import generar_secreto, uri_otpauth, verificar_totp
from app.db.session import get_db
from app.models.user import User
from app.notify.mailer import enviar_aviso_mfa_seguro

router = APIRouter()

Rol = Literal["admin", "auditor", "rsi"]
AlgoritmoHash = Literal["argon2id", "bcrypt"]
MENSAJE_CREDENCIALES = "Credenciales inválidas."


class UsuarioIn(BaseModel):
    email: str = Field(min_length=3, max_length=256)
    password: str = Field(min_length=8, max_length=128)
    rol: Rol = "rsi"
    algoritmo_hash: AlgoritmoHash | None = None


class LoginIn(BaseModel):
    email: str
    password: str
    codigo: str | None = None


class CodigoIn(BaseModel):
    codigo: str = Field(min_length=6, max_length=6)


class UsuarioOut(BaseModel):
    model_config = ConfigDict(from_attributes=True)

    id: int
    email: str
    rol: str
    algoritmo_hash: str
    mfa_habilitado: bool
    mfa_tipo: str | None


class LoginOut(BaseModel):
    access_token: str
    token_type: str = "bearer"
    usuario: UsuarioOut


class EnrollOut(BaseModel):
    secreto: str
    otpauth_uri: str


def hashear(clave: str, algoritmo: str) -> str:
    if algoritmo == "bcrypt":
        return bcrypt.hash(clave)
    return argon2.using(type="ID", memory_cost=19456, time_cost=2, parallelism=1).hash(clave)


def verificar_clave(clave: str, huella: str) -> bool:
    try:
        if huella.startswith("$argon2"):
            return argon2.verify(clave, huella)
        if huella.startswith("$2"):
            return bcrypt.verify(clave, huella)
    except (ValueError, TypeError):
        return False
    return False


def _email(valor: str) -> str:
    limpio = valor.strip().lower()
    if "@" not in limpio or limpio.startswith("@") or limpio.endswith("@"):
        raise HTTPException(status_code=400, detail="El correo no es válido.")
    return limpio


def _algoritmo(pedido: str | None) -> str:
    valor = pedido or settings.hash_algorithm
    if valor not in ("argon2id", "bcrypt"):
        return "argon2id"
    return valor


def _usuario_actual(
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


def _exigir_admin(usuario: User) -> None:
    if usuario.rol != "admin":
        raise HTTPException(status_code=403, detail="Solo un admin puede hacer esto.")


def _emitir_token(usuario: User, db: Session) -> str:
    token = secrets.token_urlsafe(32)
    usuario.token_hash = hashlib.sha256(token.encode("utf-8")).hexdigest()
    db.commit()
    return token


@router.post("/", response_model=UsuarioOut, status_code=201)
def crear_usuario(
    payload: UsuarioIn,
    db: Session = Depends(get_db),
    authorization: str | None = Header(default=None),
):
    hay_usuarios = db.scalar(select(func.count()).select_from(User)) or 0
    if hay_usuarios:
        actor = _usuario_actual(authorization, db)
        _exigir_admin(actor)
        rol = payload.rol
    else:
        rol = "admin"
    email = _email(payload.email)
    algoritmo = _algoritmo(payload.algoritmo_hash)
    if algoritmo == "bcrypt" and len(payload.password.encode("utf-8")) > 72:
        raise HTTPException(
            status_code=400,
            detail="Con bcrypt la contraseña no puede pasar de 72 bytes.",
        )
    usuario = User(
        email=email,
        rol=rol,
        hash_password=hashear(payload.password, algoritmo),
        algoritmo_hash=algoritmo,
        mfa_habilitado=False,
        mfa_tipo=None,
        totp_secreto=None,
        token_hash=None,
    )
    db.add(usuario)
    try:
        db.commit()
    except IntegrityError:
        db.rollback()
        raise HTTPException(status_code=409, detail="Ese correo ya existe.") from None
    db.refresh(usuario)
    return usuario


@router.get("/", response_model=list[UsuarioOut])
def listar_usuarios(
    db: Session = Depends(get_db),
    _: User = Depends(_usuario_actual),
):
    return list(db.scalars(select(User).order_by(User.id)))


@router.post("/login", response_model=LoginOut)
def login(payload: LoginIn, db: Session = Depends(get_db)):
    email = payload.email.strip().lower()
    usuario = db.scalar(select(User).where(User.email == email))
    if usuario is None or not verificar_clave(payload.password, usuario.hash_password):
        raise HTTPException(status_code=401, detail=MENSAJE_CREDENCIALES)
    if usuario.mfa_habilitado:
        if not payload.codigo:
            raise HTTPException(status_code=401, detail="Falta el código MFA.")
        if not usuario.totp_secreto or not verificar_totp(usuario.totp_secreto, payload.codigo):
            raise HTTPException(status_code=401, detail="Código MFA inválido.")
    token = _emitir_token(usuario, db)
    db.refresh(usuario)
    return LoginOut(access_token=token, usuario=UsuarioOut.model_validate(usuario))


@router.post("/{usuario_id}/totp/enroll", response_model=EnrollOut)
def enroll_totp(
    usuario_id: int,
    db: Session = Depends(get_db),
    actor: User = Depends(_usuario_actual),
):
    if actor.rol != "admin" and actor.id != usuario_id:
        raise HTTPException(status_code=403, detail="No podés enrolar el MFA de otro usuario.")
    usuario = db.get(User, usuario_id)
    if usuario is None:
        raise HTTPException(status_code=404, detail="No existe el usuario.")
    secreto = generar_secreto()
    usuario.totp_secreto = secreto
    usuario.mfa_habilitado = False
    usuario.mfa_tipo = None
    db.commit()
    return EnrollOut(secreto=secreto, otpauth_uri=uri_otpauth(usuario.email, secreto))


@router.post("/{usuario_id}/totp/confirmar", response_model=UsuarioOut)
def confirmar_totp(
    usuario_id: int,
    payload: CodigoIn,
    db: Session = Depends(get_db),
    actor: User = Depends(_usuario_actual),
):
    if actor.rol != "admin" and actor.id != usuario_id:
        raise HTTPException(status_code=403, detail="No podés confirmar el MFA de otro usuario.")
    usuario = db.get(User, usuario_id)
    if usuario is None or not usuario.totp_secreto:
        raise HTTPException(status_code=404, detail="Ese usuario no tiene un TOTP pendiente.")
    if not verificar_totp(usuario.totp_secreto, payload.codigo):
        raise HTTPException(status_code=401, detail="Código MFA inválido.")
    usuario.mfa_habilitado = True
    usuario.mfa_tipo = "totp"
    db.commit()
    db.refresh(usuario)
    enviar_aviso_mfa_seguro(usuario.email, usuario.id)
    return usuario

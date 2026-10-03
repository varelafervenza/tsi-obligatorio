"""Usuario del panel del control central. RF-11.

El secreto TOTP y el hash de la contraseña no salen en los listados.
"""
from sqlalchemy import Boolean, String, Text
from sqlalchemy.orm import Mapped, mapped_column

from app.db.base import Base


class User(Base):
    __tablename__ = "panel_users"

    id: Mapped[int] = mapped_column(primary_key=True, autoincrement=True)
    email: Mapped[str] = mapped_column(String(256), unique=True, index=True)
    rol: Mapped[str] = mapped_column(String(16))
    hash_password: Mapped[str] = mapped_column(Text)
    algoritmo_hash: Mapped[str] = mapped_column(String(16))
    mfa_habilitado: Mapped[bool] = mapped_column(Boolean, default=False)
    mfa_tipo: Mapped[str | None] = mapped_column(String(16), nullable=True)
    totp_secreto: Mapped[str | None] = mapped_column(Text, nullable=True)
    token_hash: Mapped[str | None] = mapped_column(String(64), nullable=True, index=True)

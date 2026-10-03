"""Evento de auditoría: solo metadata. Nunca un secreto ni el valor de una contraseña."""
from datetime import datetime

from sqlalchemy import Boolean, DateTime, String, Text
from sqlalchemy.orm import Mapped, mapped_column

from app.db.base import Base

TIPOS_EVENTO = (
    "alta_credencial",
    "modificacion_credencial",
    "borrado_credencial",
    "cambio_maestra",
    "intento_fallido_maestra",
    "vencimiento_credencial",
)


class AuditEvent(Base):
    __tablename__ = "audit_events"

    id: Mapped[int] = mapped_column(primary_key=True, autoincrement=True)
    agente_id: Mapped[str] = mapped_column(String(128), index=True)
    tipo: Mapped[str] = mapped_column(String(64), index=True)
    sistema: Mapped[str] = mapped_column(String(256))
    occurred_at: Mapped[datetime] = mapped_column(DateTime(timezone=True))
    received_at: Mapped[datetime] = mapped_column(DateTime(timezone=True))
    firma_jws: Mapped[str] = mapped_column(Text)
    firma_valida: Mapped[bool | None] = mapped_column(Boolean, nullable=True)
    ip_origen: Mapped[str | None] = mapped_column(String(64), nullable=True)

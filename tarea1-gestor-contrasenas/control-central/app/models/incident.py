"""Incidente abierto a partir de un evento de auditoría. RF-13.

Estados: abierto, en_analisis (en análisis), resuelto.
"""
from datetime import datetime

from sqlalchemy import DateTime, ForeignKey, String, Text
from sqlalchemy.orm import Mapped, mapped_column

from app.db.base import Base


class Incident(Base):
    __tablename__ = "incidents"

    id: Mapped[int] = mapped_column(primary_key=True, autoincrement=True)
    evento_origen_id: Mapped[int] = mapped_column(ForeignKey("audit_events.id"), index=True)
    severidad: Mapped[str] = mapped_column(String(2))
    estado: Mapped[str] = mapped_column(String(16), index=True)
    descripcion: Mapped[str] = mapped_column(Text)
    asignado_a: Mapped[str | None] = mapped_column(String(128), nullable=True)
    creado_en: Mapped[datetime] = mapped_column(DateTime(timezone=True))
    resuelto_en: Mapped[datetime | None] = mapped_column(DateTime(timezone=True), nullable=True)

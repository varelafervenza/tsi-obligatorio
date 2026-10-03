"""Alerta de las mismas reglas que infra/wazuh/local_rules.xml. RF-10.

El manager de Wazuh las vuelve a evaluar sobre el JSONL. Acá quedan
registradas para el panel aunque el manager todavía no esté levantado.
"""
from datetime import datetime

from sqlalchemy import Boolean, DateTime, ForeignKey, String
from sqlalchemy.orm import Mapped, mapped_column

from app.db.base import Base


class Alert(Base):
    __tablename__ = "alerts"

    id: Mapped[int] = mapped_column(primary_key=True, autoincrement=True)
    regla: Mapped[str] = mapped_column(String(64), index=True)
    evento_id: Mapped[int] = mapped_column(ForeignKey("audit_events.id"), index=True)
    agente_id: Mapped[str] = mapped_column(String(128), index=True)
    creado_en: Mapped[datetime] = mapped_column(DateTime(timezone=True))
    es_falso_positivo: Mapped[bool | None] = mapped_column(Boolean, nullable=True)

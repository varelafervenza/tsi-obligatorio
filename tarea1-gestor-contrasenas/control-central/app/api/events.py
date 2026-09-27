"""Recepción de eventos de auditoría (RF-08).

Persiste metadata en Postgres y escribe una línea JSONL para el SIEM.
La verificación JWS y el correo quedan para commits siguientes.
"""
from datetime import datetime, timezone
from typing import Literal

from fastapi import APIRouter, Depends, Request
from pydantic import BaseModel, ConfigDict, Field
from sqlalchemy import select
from sqlalchemy.orm import Session

from app.db.session import get_db
from app.models.event import AuditEvent
from app.siem.wazuh_forwarder import reenviar_evento_seguro

router = APIRouter()

TipoEvento = Literal[
    "alta_credencial",
    "modificacion_credencial",
    "borrado_credencial",
    "cambio_maestra",
    "intento_fallido_maestra",
]


class EventoIn(BaseModel):
    tipo: TipoEvento
    sistema: str = Field(min_length=1, max_length=256)
    agente_id: str = Field(min_length=1, max_length=128)
    timestamp: datetime
    firma_jws: str = Field(min_length=1)


class EventoOut(BaseModel):
    model_config = ConfigDict(from_attributes=True)

    id: int
    tipo: str
    sistema: str
    agente_id: str
    occurred_at: datetime
    received_at: datetime
    firma_valida: bool | None
    ip_origen: str | None


@router.post("/", response_model=EventoOut, status_code=201)
def recibir_evento(payload: EventoIn, request: Request, db: Session = Depends(get_db)):
    # firma_valida=None: todavía no se verifica JWS (RF-08, siguiente parte).
    evento = AuditEvent(
        agente_id=payload.agente_id,
        tipo=payload.tipo,
        sistema=payload.sistema,
        occurred_at=payload.timestamp,
        received_at=datetime.now(timezone.utc),
        firma_jws=payload.firma_jws,
        firma_valida=None,
        ip_origen=request.client.host if request.client else None,
    )
    db.add(evento)
    db.commit()
    db.refresh(evento)
    reenviar_evento_seguro(evento)
    return evento


@router.get("/", response_model=list[EventoOut])
def listar_eventos(db: Session = Depends(get_db), limit: int = 20):
    stmt = select(AuditEvent).order_by(AuditEvent.id.desc()).limit(min(limit, 100))
    return list(db.scalars(stmt))

"""Recepción de eventos de auditoría (RF-08).

Persiste metadata en Postgres, escribe JSONL para el SIEM y notifica por correo
(RF-07). Verifica JWS con la clave pública del agente (RF-08).
"""
from datetime import datetime, timezone
from typing import Literal

from fastapi import APIRouter, Depends, Request
from pydantic import BaseModel, ConfigDict, Field
from sqlalchemy import select
from sqlalchemy.orm import Session

from app.core.security import verificar_jws
from app.db.session import get_db
from app.models.event import AuditEvent
from app.notify.mailer import enviar_notificacion_segura
from app.siem.reglas import evaluar_reglas
from app.siem.wazuh_forwarder import reenviar_evento_seguro

router = APIRouter()

TipoEvento = Literal[
    "alta_credencial",
    "modificacion_credencial",
    "borrado_credencial",
    "cambio_maestra",
    "intento_fallido_maestra",
    "vencimiento_credencial",
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
    ts = payload.timestamp.astimezone(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    firma_valida = verificar_jws(
        payload.firma_jws,
        agente_id=payload.agente_id,
        tipo=payload.tipo,
        sistema=payload.sistema,
        timestamp_iso=ts,
    )
    evento = AuditEvent(
        agente_id=payload.agente_id,
        tipo=payload.tipo,
        sistema=payload.sistema,
        occurred_at=payload.timestamp,
        received_at=datetime.now(timezone.utc),
        firma_jws=payload.firma_jws,
        firma_valida=firma_valida,
        ip_origen=request.client.host if request.client else None,
    )
    db.add(evento)
    db.commit()
    db.refresh(evento)
    reenviar_evento_seguro(evento)
    enviar_notificacion_segura(evento)
    evaluar_reglas(db, evento)
    return evento


@router.get("/", response_model=list[EventoOut])
def listar_eventos(db: Session = Depends(get_db), limit: int = 20):
    stmt = select(AuditEvent).order_by(AuditEvent.id.desc()).limit(min(limit, 100))
    return list(db.scalars(stmt))

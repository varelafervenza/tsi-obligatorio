"""Registro de incidentes a partir de un evento ya persistido. RF-13."""
from datetime import datetime, timezone
from typing import Literal

from fastapi import APIRouter, Depends, HTTPException
from pydantic import BaseModel, ConfigDict, Field
from sqlalchemy import select
from sqlalchemy.orm import Session

from app.db.session import get_db
from app.models.event import AuditEvent
from app.models.incident import Incident

router = APIRouter()

EstadoIncidente = Literal["abierto", "en_analisis", "resuelto"]
SeveridadIncidente = Literal["S0", "S1", "S2", "S3"]


class IncidenteIn(BaseModel):
    evento_origen_id: int = Field(ge=1)
    severidad: SeveridadIncidente
    descripcion: str = Field(min_length=1, max_length=2000)
    asignado_a: str | None = Field(default=None, max_length=128)


class IncidentePatch(BaseModel):
    estado: EstadoIncidente | None = None
    asignado_a: str | None = Field(default=None, max_length=128)


class IncidenteOut(BaseModel):
    model_config = ConfigDict(from_attributes=True)

    id: int
    evento_origen_id: int
    severidad: str
    estado: str
    descripcion: str
    asignado_a: str | None
    creado_en: datetime
    resuelto_en: datetime | None


def _aplicar_estado(incidente: Incident, estado: str) -> None:
    incidente.estado = estado
    if estado == "resuelto":
        if incidente.resuelto_en is None:
            incidente.resuelto_en = datetime.now(timezone.utc)
        return
    incidente.resuelto_en = None


@router.post("/", response_model=IncidenteOut, status_code=201)
def crear_incidente(payload: IncidenteIn, db: Session = Depends(get_db)):
    evento = db.get(AuditEvent, payload.evento_origen_id)
    if evento is None:
        raise HTTPException(status_code=404, detail="No existe el evento de origen.")
    descripcion = payload.descripcion.strip()
    if not descripcion:
        raise HTTPException(status_code=400, detail="La descripción está vacía.")
    incidente = Incident(
        evento_origen_id=evento.id,
        severidad=payload.severidad,
        estado="abierto",
        descripcion=descripcion,
        asignado_a=payload.asignado_a.strip() if payload.asignado_a else None,
        creado_en=datetime.now(timezone.utc),
        resuelto_en=None,
    )
    db.add(incidente)
    db.commit()
    db.refresh(incidente)
    return incidente


@router.get("/", response_model=list[IncidenteOut])
def listar_incidentes(db: Session = Depends(get_db), limit: int = 50):
    stmt = select(Incident).order_by(Incident.id.desc()).limit(min(limit, 100))
    return list(db.scalars(stmt))


@router.patch("/{incidente_id}", response_model=IncidenteOut)
def actualizar_incidente(
    incidente_id: int,
    payload: IncidentePatch,
    db: Session = Depends(get_db),
):
    incidente = db.get(Incident, incidente_id)
    if incidente is None:
        raise HTTPException(status_code=404, detail="No existe el incidente.")
    if payload.estado is not None:
        _aplicar_estado(incidente, payload.estado)
    if "asignado_a" in payload.model_fields_set:
        incidente.asignado_a = payload.asignado_a.strip() if payload.asignado_a else None
    db.commit()
    db.refresh(incidente)
    return incidente

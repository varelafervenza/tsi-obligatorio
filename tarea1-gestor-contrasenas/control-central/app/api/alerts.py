"""Listado y clasificación de alertas de las reglas locales. RF-10."""
from datetime import datetime

from fastapi import APIRouter, Depends, HTTPException
from pydantic import BaseModel, ConfigDict
from sqlalchemy import select
from sqlalchemy.orm import Session

from app.db.session import get_db
from app.models.alert import Alert

router = APIRouter()


class AlertaOut(BaseModel):
    model_config = ConfigDict(from_attributes=True)

    id: int
    regla: str
    evento_id: int
    agente_id: str
    creado_en: datetime
    es_falso_positivo: bool | None


class AlertaPatch(BaseModel):
    es_falso_positivo: bool


@router.get("/", response_model=list[AlertaOut])
def listar_alertas(db: Session = Depends(get_db), limit: int = 50):
    stmt = select(Alert).order_by(Alert.id.desc()).limit(min(limit, 100))
    return list(db.scalars(stmt))


@router.patch("/{alerta_id}", response_model=AlertaOut)
def clasificar_alerta(alerta_id: int, payload: AlertaPatch, db: Session = Depends(get_db)):
    alerta = db.get(Alert, alerta_id)
    if alerta is None:
        raise HTTPException(status_code=404, detail="No existe la alerta.")
    alerta.es_falso_positivo = payload.es_falso_positivo
    db.commit()
    db.refresh(alerta)
    return alerta

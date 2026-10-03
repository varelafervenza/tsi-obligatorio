"""Correlación local, mismos umbrales que local_rules.xml. RF-10.

- cambio de maestra: una alerta por evento
- fuerza bruta: 5 intentos fallidos del mismo agente en 2 minutos
- borrado masivo: 5 borrados del mismo agente en 2 minutos
"""
from __future__ import annotations

import logging
from datetime import datetime, timedelta, timezone

from sqlalchemy import func, select
from sqlalchemy.orm import Session

from app.models.alert import Alert
from app.models.event import AuditEvent

logger = logging.getLogger(__name__)

UMBRAL = 5
VENTANA = timedelta(seconds=120)
REGLA_POR_TIPO = {
    "intento_fallido_maestra": "fuerza_bruta_maestra",
    "borrado_credencial": "borrado_masivo",
}


def dispara(cantidad: int, ya_hay_alerta: bool, umbral: int = UMBRAL) -> bool:
    return cantidad >= umbral and not ya_hay_alerta


def evaluar_reglas(db: Session, evento: AuditEvent) -> None:
    try:
        if evento.tipo == "cambio_maestra":
            _guardar(db, "cambio_maestra", evento)
            return
        regla = REGLA_POR_TIPO.get(evento.tipo)
        if regla is None:
            return
        desde = _utc(evento.received_at) - VENTANA
        cantidad = int(
            db.scalar(
                select(func.count())
                .select_from(AuditEvent)
                .where(
                    AuditEvent.tipo == evento.tipo,
                    AuditEvent.agente_id == evento.agente_id,
                    AuditEvent.received_at >= desde,
                )
            )
            or 0
        )
        ya = (
            db.scalar(
                select(func.count())
                .select_from(Alert)
                .where(
                    Alert.regla == regla,
                    Alert.agente_id == evento.agente_id,
                    Alert.creado_en >= desde,
                )
            )
            or 0
        )
        if dispara(cantidad, ya > 0):
            _guardar(db, regla, evento)
    except Exception:
        logger.exception("No se pudo evaluar reglas para el evento %s", evento.id)


def _guardar(db: Session, regla: str, evento: AuditEvent) -> None:
    db.add(
        Alert(
            regla=regla,
            evento_id=evento.id,
            agente_id=evento.agente_id,
            creado_en=datetime.now(timezone.utc),
            es_falso_positivo=None,
        )
    )
    db.commit()


def _utc(valor: datetime) -> datetime:
    if valor.tzinfo is None:
        return valor.replace(tzinfo=timezone.utc)
    return valor.astimezone(timezone.utc)

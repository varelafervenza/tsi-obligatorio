"""Conserva eventos al menos 90 días y borra solo los más viejos. RNF-07.

No toca un evento que sea origen de un incidente o de una alerta.
"""
from datetime import datetime, timedelta, timezone

from sqlalchemy import delete, select
from sqlalchemy.orm import Session

from app.models.alert import Alert
from app.models.event import AuditEvent
from app.models.incident import Incident

RETENCION = timedelta(days=90)


def purgar_eventos_viejos(db: Session) -> int:
    corte = datetime.now(timezone.utc) - RETENCION
    resultado = db.execute(
        delete(AuditEvent).where(
            AuditEvent.received_at < corte,
            AuditEvent.id.not_in(select(Incident.evento_origen_id)),
            AuditEvent.id.not_in(select(Alert.evento_id)),
        )
    )
    db.commit()
    return int(resultado.rowcount or 0)

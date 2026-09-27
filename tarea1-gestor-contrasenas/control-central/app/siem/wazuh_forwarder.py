"""Escribe un JSON por línea (JSONL) para que Wazuh lo lea con localfile.

RF-10: el decoder/regla vendrá después; acá solo se fija el formato y se deja
el archivo. No se escribe firma JWS ni secretos (RNF-06).
"""
from __future__ import annotations

import json
import logging
from datetime import datetime
from pathlib import Path

from app.core.config import settings
from app.models.event import AuditEvent

logger = logging.getLogger(__name__)


def _iso(value: datetime) -> str:
    return value.isoformat()


def evento_a_linea(evento: AuditEvent) -> dict:
    return {
        "programa": "control-central",
        "event_id": evento.id,
        "tipo": evento.tipo,
        "sistema": evento.sistema,
        "agente_id": evento.agente_id,
        "occurred_at": _iso(evento.occurred_at),
        "received_at": _iso(evento.received_at),
        "ip_origen": evento.ip_origen,
        "firma_valida": evento.firma_valida,
    }


def reenviar_evento(evento: AuditEvent) -> None:
    path = Path(settings.siem_log_path)
    path.parent.mkdir(parents=True, exist_ok=True)
    linea = json.dumps(evento_a_linea(evento), ensure_ascii=False)
    with path.open("a", encoding="utf-8") as fh:
        fh.write(linea + "\n")


def reenviar_evento_seguro(evento: AuditEvent) -> None:
    """No tumba el POST si el disco falla: Postgres ya es la fuente de verdad."""
    try:
        reenviar_evento(evento)
    except OSError:
        logger.exception("No se pudo escribir el log SIEM en %s", settings.siem_log_path)

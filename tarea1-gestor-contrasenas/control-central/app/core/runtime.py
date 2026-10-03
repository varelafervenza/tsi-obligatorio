"""Marca el arranque del proceso para el uptime del panel (RF-14, sección 6.4)."""
from datetime import datetime, timezone

_arranque: datetime | None = None


def marcar_arranque() -> None:
    global _arranque
    _arranque = datetime.now(timezone.utc)


def segundos_activo() -> int:
    inicio = _arranque or datetime.now(timezone.utc)
    return max(0, int((datetime.now(timezone.utc) - inicio).total_seconds()))

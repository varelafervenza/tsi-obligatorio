from contextlib import asynccontextmanager

from fastapi import FastAPI
from fastapi.responses import JSONResponse
from sqlalchemy.exc import SQLAlchemyError

import logging

from app.api import alerts, dashboard, events, incidents, users
from app.core.runtime import marcar_arranque
from app.db.base import Base
from app.db.retention import purgar_eventos_viejos
from app.db.session import SessionLocal, engine, ping_db
from app.models.alert import Alert  # noqa: F401 — registra el modelo en Base
from app.models.event import AuditEvent  # noqa: F401 — registra el modelo en Base
from app.models.incident import Incident  # noqa: F401 — registra el modelo en Base
from app.models.user import User  # noqa: F401 — registra el modelo en Base

logger = logging.getLogger(__name__)


@asynccontextmanager
async def lifespan(_app: FastAPI):
    marcar_arranque()
    Base.metadata.create_all(bind=engine)
    try:
        with SessionLocal() as db:
            purgar_eventos_viejos(db)
    except SQLAlchemyError:
        logger.exception("No se pudo aplicar la retención de 90 días")
    yield


app = FastAPI(title="Control Central - Gestor de Contraseñas", lifespan=lifespan)

app.include_router(events.router, prefix="/api/events", tags=["events"])
app.include_router(alerts.router, prefix="/api/alerts", tags=["alerts"])
app.include_router(incidents.router, prefix="/api/incidents", tags=["incidents"])
app.include_router(users.router, prefix="/api/users", tags=["users"])
app.include_router(dashboard.router, prefix="/api/dashboard", tags=["dashboard"])


@app.get("/healthz")
def healthz():
    try:
        ping_db()
    except SQLAlchemyError:
        return JSONResponse(
            status_code=503,
            content={"status": "degraded", "database": "down"},
        )
    return {"status": "ok", "database": "up"}

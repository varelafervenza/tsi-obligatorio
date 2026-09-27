from contextlib import asynccontextmanager

from fastapi import FastAPI
from fastapi.responses import JSONResponse
from sqlalchemy.exc import SQLAlchemyError

from app.api import dashboard, events, users
from app.db.base import Base
from app.db.session import engine, ping_db
from app.models.event import AuditEvent  # noqa: F401 — registra el modelo en Base


@asynccontextmanager
async def lifespan(_app: FastAPI):
    Base.metadata.create_all(bind=engine)
    yield


app = FastAPI(title="Control Central - Gestor de Contraseñas", lifespan=lifespan)

app.include_router(events.router, prefix="/api/events", tags=["events"])
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

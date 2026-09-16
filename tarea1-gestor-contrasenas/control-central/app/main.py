from fastapi import FastAPI

from app.api import dashboard, events, users

app = FastAPI(title="Control Central - Gestor de Contraseñas")

app.include_router(events.router, prefix="/api/events", tags=["events"])
app.include_router(users.router, prefix="/api/users", tags=["users"])
app.include_router(dashboard.router, prefix="/api/dashboard", tags=["dashboard"])


@app.get("/healthz")
def healthz():
    return {"status": "ok"}

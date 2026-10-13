"""Las rutas de incidentes y alertas piden token, igual que las de usuarios (RF-11).

Hallazgo del 12/10/2026: antes, cualquiera sin token podía crear incidentes falsos, marcar un
incidente real como "resuelto" (apagando la alerta de un ataque en curso) o reclasificar
cualquier alerta como falso positivo. Ver `app/core/auth.py`.
"""
import hashlib
from datetime import datetime, timezone

import pytest
from fastapi.testclient import TestClient

from app.db.base import Base
from app.db.session import SessionLocal, engine, get_db
from app.main import app
from app.models.alert import Alert
from app.models.event import AuditEvent
from app.models.user import User

TOKEN_DE_PRUEBA = "token-de-prueba-no-es-un-secreto-real"


@pytest.fixture
def db():
    assert "test" in engine.url.database, "Las pruebas solo corren contra una base de test"
    Base.metadata.drop_all(bind=engine)
    Base.metadata.create_all(bind=engine)
    session = SessionLocal()
    yield session
    session.close()
    Base.metadata.drop_all(bind=engine)


@pytest.fixture
def cliente(db):
    app.dependency_overrides[get_db] = lambda: db
    yield TestClient(app)
    app.dependency_overrides.clear()


@pytest.fixture
def encabezado_con_token(db):
    usuario = User(
        email="rsi@correo.local",
        rol="admin",
        hash_password="no-se-usa-en-esta-prueba",
        algoritmo_hash="argon2id",
        token_hash=hashlib.sha256(TOKEN_DE_PRUEBA.encode("utf-8")).hexdigest(),
    )
    db.add(usuario)
    db.commit()
    return {"Authorization": f"Bearer {TOKEN_DE_PRUEBA}"}


def crear_evento(db) -> int:
    evento = AuditEvent(
        agente_id="agente-prueba",
        tipo="cambio_maestra",
        sistema="sistema-prueba",
        occurred_at=datetime(2026, 10, 12, tzinfo=timezone.utc),
        received_at=datetime(2026, 10, 12, tzinfo=timezone.utc),
        firma_jws="TODO",
        firma_valida=True,
        ip_origen="127.0.0.1",
    )
    db.add(evento)
    db.commit()
    db.refresh(evento)
    return evento.id


def crear_alerta(db, evento_id: int) -> int:
    alerta = Alert(
        regla="cambio_maestra",
        evento_id=evento_id,
        agente_id="agente-prueba",
        creado_en=datetime(2026, 10, 12, tzinfo=timezone.utc),
        es_falso_positivo=None,
    )
    db.add(alerta)
    db.commit()
    db.refresh(alerta)
    return alerta.id


def test_crear_incidente_sin_token_da_401(db, cliente):
    evento_id = crear_evento(db)
    respuesta = cliente.post(
        "/api/incidents/",
        json={"evento_origen_id": evento_id, "severidad": "S1", "descripcion": "ataque falso"},
    )
    assert respuesta.status_code == 401


def test_listar_incidentes_sin_token_da_401(cliente):
    assert cliente.get("/api/incidents/").status_code == 401


def test_patch_incidente_sin_token_da_401(cliente):
    # Ni siquiera hace falta que el incidente exista: el token se exige antes de buscarlo.
    assert cliente.patch("/api/incidents/1", json={"estado": "resuelto"}).status_code == 401


def test_listar_alertas_sin_token_da_401(cliente):
    assert cliente.get("/api/alerts/").status_code == 401


def test_clasificar_alerta_sin_token_da_401(cliente):
    assert cliente.patch("/api/alerts/1", json={"es_falso_positivo": True}).status_code == 401


def test_crear_incidente_con_token_funciona(db, cliente, encabezado_con_token):
    evento_id = crear_evento(db)
    respuesta = cliente.post(
        "/api/incidents/",
        json={"evento_origen_id": evento_id, "severidad": "S1", "descripcion": "incidente real"},
        headers=encabezado_con_token,
    )
    assert respuesta.status_code == 201


def test_clasificar_alerta_con_token_funciona(db, cliente, encabezado_con_token):
    evento_id = crear_evento(db)
    alerta_id = crear_alerta(db, evento_id)
    respuesta = cliente.patch(
        f"/api/alerts/{alerta_id}",
        json={"es_falso_positivo": False},
        headers=encabezado_con_token,
    )
    assert respuesta.status_code == 200
    assert respuesta.json()["es_falso_positivo"] is False

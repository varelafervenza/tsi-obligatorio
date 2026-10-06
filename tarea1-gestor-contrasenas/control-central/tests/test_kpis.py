"""Pruebas de los KPIs del panel (MTTD, MTTR y cobertura). Sección 6.4 de LETRA.md.

Corren contra PostgreSQL, con una base cuyo nombre contenga "test". Las tablas se borran
y se crean en cada prueba, así que nunca se usa la base de laboratorio.
"""
from datetime import datetime, timedelta, timezone

import pytest
from fastapi.testclient import TestClient

from app.db.base import Base
from app.db.session import SessionLocal, engine, get_db
from app.main import app
from app.models.alert import Alert
from app.models.event import AuditEvent
from app.models.incident import Incident

INICIO = datetime(2026, 10, 5, 20, 0, 0, tzinfo=timezone.utc)


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


def crear_evento(db, ocurrido_en, tipo="alta_credencial"):
    evento = AuditEvent(
        agente_id="agente-test",
        tipo=tipo,
        sistema="sistema-test",
        occurred_at=ocurrido_en,
        received_at=ocurrido_en,
        firma_jws="prueba",
        firma_valida=True,
        ip_origen="127.0.0.1",
    )
    db.add(evento)
    db.commit()
    db.refresh(evento)
    return evento


def crear_alerta(db, evento, creado_en):
    db.add(
        Alert(
            regla="fuerza_bruta_maestra",
            evento_id=evento.id,
            agente_id=evento.agente_id,
            creado_en=creado_en,
            es_falso_positivo=None,
        )
    )
    db.commit()


def crear_incidente(db, evento, creado_en, estado="abierto", resuelto_en=None):
    db.add(
        Incident(
            evento_origen_id=evento.id,
            severidad="S2",
            estado=estado,
            descripcion="prueba",
            asignado_a=None,
            creado_en=creado_en,
            resuelto_en=resuelto_en,
        )
    )
    db.commit()


def kpis(cliente):
    respuesta = cliente.get("/api/dashboard/kpis")
    assert respuesta.status_code == 200
    return respuesta.json()["kpis"]


def test_mttd_va_desde_el_evento_hasta_la_primera_alerta(db, cliente):
    evento = crear_evento(db, INICIO)
    crear_alerta(db, evento, INICIO + timedelta(seconds=2))
    crear_alerta(db, evento, INICIO + timedelta(seconds=5))
    # La creación manual del incidente no entra en el MTTD.
    crear_incidente(db, evento, INICIO + timedelta(hours=3))

    mttd = kpis(cliente)["mttd"]

    assert mttd == {"segundos": 2.0, "muestras": 1}


def test_mttd_ignora_incidentes_sin_alerta(db, cliente):
    evento = crear_evento(db, INICIO)
    crear_incidente(db, evento, INICIO + timedelta(hours=1))

    assert kpis(cliente)["mttd"] == {"segundos": None, "muestras": 0}


def test_mttr_va_desde_la_alerta_hasta_la_resolucion(db, cliente):
    evento = crear_evento(db, INICIO)
    crear_alerta(db, evento, INICIO)
    crear_incidente(
        db,
        evento,
        INICIO + timedelta(hours=3),
        estado="resuelto",
        resuelto_en=INICIO + timedelta(seconds=1000),
    )

    mttr = kpis(cliente)["mttr"]

    assert mttr == {"segundos": 1000.0, "muestras": 1}


def test_mttr_no_cuenta_incidentes_abiertos(db, cliente):
    evento = crear_evento(db, INICIO)
    crear_alerta(db, evento, INICIO)
    crear_incidente(db, evento, INICIO + timedelta(hours=1))

    assert kpis(cliente)["mttr"] == {"segundos": None, "muestras": 0}


def test_cobertura_cuenta_los_tipos_requeridos_vistos(db, cliente):
    crear_evento(db, INICIO, tipo="alta_credencial")
    crear_evento(db, INICIO, tipo="borrado_credencial")

    cobertura = kpis(cliente)["cobertura_eventos"]

    assert cobertura["porcentaje"] == 50.0
    assert sorted(cobertura["tipos_vistos"]) == ["alta_credencial", "borrado_credencial"]

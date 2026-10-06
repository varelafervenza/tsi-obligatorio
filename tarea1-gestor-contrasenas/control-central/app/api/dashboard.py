"""Panel del control central. RF-09, RF-14 y KPIs de la sección 6.4 de LETRA.md.

MTTD: promedio de la primera alerta del evento de origen (creado_en) menos occurred_at de ese evento.
MTTR: promedio de resuelto_en menos la primera alerta del evento de origen, solo casos resueltos con alerta.
Cobertura: cuántos de los cuatro tipos de RF-07 ya llegaron al menos una vez.
La tasa de falsos positivos queda vacía hasta que el SIEM emita alertas (RF-10).
"""
import socket
from datetime import datetime, timedelta, timezone

from fastapi import APIRouter, Depends
from pydantic import BaseModel
from sqlalchemy import func, select
from sqlalchemy.orm import Session

from app.core.config import settings
from app.core.runtime import segundos_activo
from app.db.session import get_db
from app.models.alert import Alert
from app.models.event import AuditEvent
from app.models.incident import Incident

router = APIRouter()

TIPOS_COBERTURA = (
    "alta_credencial",
    "modificacion_credencial",
    "borrado_credencial",
    "cambio_maestra",
)
VENTANA_AGENTE = timedelta(hours=24)


class AgenteEstado(BaseModel):
    agente_id: str
    ultima_recepcion: datetime | None
    eventos: int
    activo: bool


class CorreoEstado(BaseModel):
    estado: str
    host: str
    port: int


class Funcionamiento(BaseModel):
    agentes: list[AgenteEstado]
    agentes_activos: int
    ultima_sincronizacion: datetime | None
    correo: CorreoEstado
    base_de_datos: str


class Duracion(BaseModel):
    segundos: float | None
    muestras: int


class Cobertura(BaseModel):
    tipos_requeridos: list[str]
    tipos_vistos: list[str]
    porcentaje: float


class FalsosPositivos(BaseModel):
    tasa: float | None
    alertas_siem: int
    alertas_reglas: int
    nota: str


class Kpis(BaseModel):
    mttd: Duracion
    mttr: Duracion
    cobertura_eventos: Cobertura
    falsos_positivos: FalsosPositivos
    uptime_segundos: int


class Panel(BaseModel):
    generado_en: datetime
    funcionamiento: Funcionamiento
    volumen_por_tipo: dict[str, int]
    alertas: dict[str, int]
    incidentes: dict[str, int]
    kpis: Kpis


def _utc(valor: datetime) -> datetime:
    if valor.tzinfo is None:
        return valor.replace(tzinfo=timezone.utc)
    return valor.astimezone(timezone.utc)


def _promedio_segundos(pares: list[tuple[datetime, datetime]]) -> float | None:
    if not pares:
        return None
    total = 0.0
    for fin, inicio in pares:
        total += max(0.0, (_utc(fin) - _utc(inicio)).total_seconds())
    return round(total / len(pares), 1)


def _estado_correo() -> CorreoEstado:
    try:
        with socket.create_connection((settings.smtp_host, settings.smtp_port), timeout=2):
            estado = "up"
    except OSError:
        estado = "down"
    return CorreoEstado(estado=estado, host=settings.smtp_host, port=settings.smtp_port)


@router.get("/kpis", response_model=Panel)
def obtener_kpis(db: Session = Depends(get_db)):
    ahora = datetime.now(timezone.utc)
    volumen = {
        tipo: int(cantidad)
        for tipo, cantidad in db.execute(
            select(AuditEvent.tipo, func.count()).group_by(AuditEvent.tipo)
        )
    }
    agentes = []
    for agente_id, ultima, cantidad in db.execute(
        select(
            AuditEvent.agente_id,
            func.max(AuditEvent.received_at),
            func.count(),
        )
        .group_by(AuditEvent.agente_id)
        .order_by(func.max(AuditEvent.received_at).desc())
    ):
        ultima_utc = _utc(ultima) if ultima is not None else None
        agentes.append(
            AgenteEstado(
                agente_id=agente_id,
                ultima_recepcion=ultima_utc,
                eventos=int(cantidad),
                activo=ultima_utc is not None and ahora - ultima_utc <= VENTANA_AGENTE,
            )
        )

    incidentes = {"abierto": 0, "en_analisis": 0, "resuelto": 0}
    for estado, cantidad in db.execute(
        select(Incident.estado, func.count()).group_by(Incident.estado)
    ):
        if estado in incidentes:
            incidentes[estado] = int(cantidad)

    detecciones = [
        (detectado_en, occurred_at)
        for detectado_en, occurred_at in db.execute(
            select(func.min(Alert.creado_en), AuditEvent.occurred_at)
            .select_from(Incident)
            .join(AuditEvent, AuditEvent.id == Incident.evento_origen_id)
            .join(Alert, Alert.evento_id == Incident.evento_origen_id)
            .group_by(Incident.id, AuditEvent.occurred_at)
        )
        if detectado_en is not None and occurred_at is not None
    ]
    respuestas = [
        (resuelto_en, detectado_en)
        for resuelto_en, detectado_en in db.execute(
            select(Incident.resuelto_en, func.min(Alert.creado_en))
            .select_from(Incident)
            .join(Alert, Alert.evento_id == Incident.evento_origen_id)
            .where(
                Incident.estado == "resuelto",
                Incident.resuelto_en.is_not(None),
            )
            .group_by(Incident.id, Incident.resuelto_en)
        )
        if resuelto_en is not None and detectado_en is not None
    ]
    vistos = [tipo for tipo in TIPOS_COBERTURA if volumen.get(tipo, 0) > 0]
    firma_invalida = int(
        db.scalar(select(func.count()).select_from(AuditEvent).where(AuditEvent.firma_valida.is_(False)))
        or 0
    )
    alertas_reglas = int(db.scalar(select(func.count()).select_from(Alert)) or 0)
    clasificadas = int(
        db.scalar(
            select(func.count()).select_from(Alert).where(Alert.es_falso_positivo.is_not(None))
        )
        or 0
    )
    falsos = int(
        db.scalar(
            select(func.count()).select_from(Alert).where(Alert.es_falso_positivo.is_(True))
        )
        or 0
    )
    if clasificadas == 0:
        tasa_falsos = None
        nota_falsos = (
            f"{alertas_reglas} alertas de las reglas locales, ninguna clasificada. "
            "Wazuh todavía no emite las suyas."
        )
    else:
        tasa_falsos = round(100 * falsos / clasificadas, 1)
        nota_falsos = "Tasa sobre alertas locales ya clasificadas. El manager de Wazuh sigue aparte."

    return Panel(
        generado_en=ahora,
        funcionamiento=Funcionamiento(
            agentes=agentes,
            agentes_activos=sum(1 for agente in agentes if agente.activo),
            ultima_sincronizacion=agentes[0].ultima_recepcion if agentes else None,
            correo=_estado_correo(),
            base_de_datos="up",
        ),
        volumen_por_tipo=volumen,
        alertas={
            "firma_invalida": firma_invalida,
            "intento_fallido_maestra": volumen.get("intento_fallido_maestra", 0),
        },
        incidentes=incidentes,
        kpis=Kpis(
            mttd=Duracion(segundos=_promedio_segundos(detecciones), muestras=len(detecciones)),
            mttr=Duracion(segundos=_promedio_segundos(respuestas), muestras=len(respuestas)),
            cobertura_eventos=Cobertura(
                tipos_requeridos=list(TIPOS_COBERTURA),
                tipos_vistos=vistos,
                porcentaje=round(100 * len(vistos) / len(TIPOS_COBERTURA), 1),
            ),
            falsos_positivos=FalsosPositivos(
                tasa=tasa_falsos,
                alertas_siem=0,
                alertas_reglas=alertas_reglas,
                nota=nota_falsos,
            ),
            uptime_segundos=segundos_activo(),
        ),
    )

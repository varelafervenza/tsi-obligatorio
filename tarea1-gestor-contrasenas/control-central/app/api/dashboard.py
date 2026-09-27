"""Datos para el dashboard (Grafana): alertas, incidentes, estado de agentes,
volumen de eventos, KPIs (MTTD, MTTR, cobertura, falsos positivos, uptime).
RF-09, RF-13, RF-14.
"""
from fastapi import APIRouter

router = APIRouter()


@router.get("/kpis")
def obtener_kpis():
    # TODO: calcular KPIs de la sección 6.4 de LETRA.md.
    raise NotImplementedError

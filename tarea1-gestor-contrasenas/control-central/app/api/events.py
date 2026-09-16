"""Recepción de eventos de auditoría firmados (JWS) desde cliente-gestor.

RF-08: alta, modificación, borrado y cambio de contraseña maestra.
TODO: verificar firma con clave pública por agente, persistir en Postgres,
reenviar a SIEM (app/siem/wazuh_forwarder.py) y disparar correo si corresponde.
"""
from fastapi import APIRouter

router = APIRouter()


@router.post("/")
def recibir_evento():
    # TODO: validar payload JWS, verificar firma, persistir, reenviar a SIEM/mail.
    raise NotImplementedError

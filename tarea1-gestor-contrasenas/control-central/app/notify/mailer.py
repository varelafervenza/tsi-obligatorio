"""Notificaciones SMTP ante alta/mod/borrado/cambio de maestra (RF-07).

H2 usa Mailpit (bandeja local). Mailu entra después con las mismas variables
SMTP_*. El cuerpo solo lleva metadata: nunca secretos ni firma JWS (RNF-06).
"""
from __future__ import annotations

import logging
import smtplib
from email.message import EmailMessage

from app.core.config import settings
from app.models.event import AuditEvent

logger = logging.getLogger(__name__)

TIPOS_CON_CORREO = frozenset(
    {
        "alta_credencial",
        "modificacion_credencial",
        "borrado_credencial",
        "cambio_maestra",
        "vencimiento_credencial",
    }
)

ASUNTO = {
    "alta_credencial": "Alta de credencial",
    "modificacion_credencial": "Modificación de credencial",
    "borrado_credencial": "Borrado de credencial",
    "cambio_maestra": "CRÍTICO — Cambio de contraseña maestra",
    "vencimiento_credencial": "Vencimiento de contraseña",
}


def _cuerpo(evento: AuditEvent) -> str:
    return (
        "Evento de auditoría del gestor de contraseñas.\n\n"
        f"id: {evento.id}\n"
        f"tipo: {evento.tipo}\n"
        f"sistema: {evento.sistema}\n"
        f"agente_id: {evento.agente_id}\n"
        f"occurred_at: {evento.occurred_at.isoformat()}\n"
        f"received_at: {evento.received_at.isoformat()}\n"
        f"ip_origen: {evento.ip_origen or '-'}\n"
        f"firma_valida: {evento.firma_valida}\n\n"
        "Este correo no incluye secretos ni la firma JWS.\n"
    )


def enviar_notificacion(evento: AuditEvent) -> None:
    if evento.tipo not in TIPOS_CON_CORREO:
        return
    msg = EmailMessage()
    msg["From"] = settings.smtp_from
    msg["To"] = settings.smtp_to
    msg["Subject"] = ASUNTO[evento.tipo]
    msg.set_content(_cuerpo(evento))

    with smtplib.SMTP(settings.smtp_host, settings.smtp_port, timeout=10) as smtp:
        if settings.smtp_use_tls:
            smtp.starttls()
        if settings.smtp_user:
            smtp.login(settings.smtp_user, settings.smtp_password)
        smtp.send_message(msg)


def enviar_aviso_mfa_seguro(email_usuario: str, usuario_id: int) -> None:
    """Avisa al titular y al RSI que un factor TOTP quedó activo (R06). No tumba la operación si SMTP falla."""
    msg = EmailMessage()
    msg["From"] = settings.smtp_from
    msg["To"] = f"{email_usuario}, {settings.smtp_to}"
    msg["Subject"] = "Se activó un factor MFA (TOTP)"
    msg.set_content(
        "Se activó un factor TOTP para el usuario del panel.\n\n"
        f"usuario_id: {usuario_id}\n"
        f"email: {email_usuario}\n\n"
        "Si no fuiste vos quien lo activó, avisá al RSI de inmediato.\n"
        "Este correo no incluye secretos ni códigos.\n"
    )
    try:
        with smtplib.SMTP(settings.smtp_host, settings.smtp_port, timeout=10) as smtp:
            if settings.smtp_use_tls:
                smtp.starttls()
            if settings.smtp_user:
                smtp.login(settings.smtp_user, settings.smtp_password)
            smtp.send_message(msg)
    except OSError:
        logger.exception("No se pudo enviar el aviso MFA del usuario %s", usuario_id)


def enviar_notificacion_segura(evento: AuditEvent) -> None:
    """No tumba el POST si SMTP falla."""
    try:
        enviar_notificacion(evento)
    except OSError:
        logger.exception(
            "No se pudo enviar correo de evento %s a %s:%s",
            evento.id,
            settings.smtp_host,
            settings.smtp_port,
        )

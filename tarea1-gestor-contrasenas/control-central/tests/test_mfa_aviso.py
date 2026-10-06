"""Aviso por correo al activar un factor TOTP (R06 de docs/03-Analisis-Riesgos.md)."""
import smtplib

from app.notify import mailer


class SmtpFalso:
    enviados = []

    def __init__(self, *_args, **_kwargs):
        pass

    def __enter__(self):
        return self

    def __exit__(self, *_args):
        return False

    def send_message(self, msg):
        SmtpFalso.enviados.append(msg)


def test_aviso_mfa_llega_al_titular_y_al_rsi(monkeypatch):
    SmtpFalso.enviados = []
    monkeypatch.setattr(mailer.smtplib, "SMTP", SmtpFalso)

    mailer.enviar_aviso_mfa_seguro("usuario@correo.local", 7)

    assert len(SmtpFalso.enviados) == 1
    msg = SmtpFalso.enviados[0]
    assert "usuario@correo.local" in msg["To"]
    assert mailer.settings.smtp_to in msg["To"]
    assert msg["Subject"] == "Se activó un factor MFA (TOTP)"


def test_aviso_mfa_no_incluye_secretos(monkeypatch):
    SmtpFalso.enviados = []
    monkeypatch.setattr(mailer.smtplib, "SMTP", SmtpFalso)

    mailer.enviar_aviso_mfa_seguro("usuario@correo.local", 7)

    cuerpo = SmtpFalso.enviados[0].get_content()
    assert "usuario_id: 7" in cuerpo
    assert "secreto" not in cuerpo.lower().replace("no incluye secretos", "")
    assert "otpauth" not in cuerpo


def test_si_smtp_falla_no_tumba_la_operacion(monkeypatch):
    def smtp_caido(*_args, **_kwargs):
        raise smtplib.SMTPConnectError(421, "servicio no disponible")

    monkeypatch.setattr(mailer.smtplib, "SMTP", smtp_caido)

    # No debe lanzar excepción: la activación del factor ya se guardó.
    mailer.enviar_aviso_mfa_seguro("usuario@correo.local", 7)

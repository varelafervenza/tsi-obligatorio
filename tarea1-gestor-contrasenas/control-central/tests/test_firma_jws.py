"""Verificación de la firma JWS de los eventos (RF-08, RS-02 de la arquitectura).

Usa un par de claves RSA generado en una carpeta temporal. No toca las claves reales de keys/agentes.
"""
from datetime import datetime, timezone

import pytest
from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.asymmetric import rsa
from jose import jwt

from app.core import security
from app.core.config import settings

AGENTE = "agente-prueba"
TIPO = "alta_credencial"
SISTEMA = "sistema-prueba"
TS = "2026-10-06T18:00:00Z"


def _par_de_claves():
    privada = rsa.generate_private_key(public_exponent=65537, key_size=2048)
    pem_privada = privada.private_bytes(
        serialization.Encoding.PEM,
        serialization.PrivateFormat.PKCS8,
        serialization.NoEncryption(),
    ).decode()
    pem_publica = privada.public_key().public_bytes(
        serialization.Encoding.PEM,
        serialization.PublicFormat.SubjectPublicKeyInfo,
    ).decode()
    return pem_privada, pem_publica


def _firmar(pem_privada, **cambios):
    datos = {"agente_id": AGENTE, "tipo": TIPO, "sistema": SISTEMA, "ts": TS}
    datos.update(cambios)
    return jwt.encode(datos, pem_privada, algorithm="RS256")


@pytest.fixture
def claves(tmp_path, monkeypatch):
    pem_privada, pem_publica = _par_de_claves()
    (tmp_path / f"{AGENTE}.pub.pem").write_text(pem_publica, encoding="utf-8")
    monkeypatch.setattr(settings, "jwt_public_keys_dir", str(tmp_path))
    return pem_privada


def verificar(firma, **cambios):
    datos = {"agente_id": AGENTE, "tipo": TIPO, "sistema": SISTEMA, "timestamp_iso": TS}
    datos.update(cambios)
    return security.verificar_jws(firma, **datos)


def test_firma_valida_se_acepta(claves):
    assert verificar(_firmar(claves)) is True


def test_firma_de_otra_clave_se_rechaza(claves):
    otra_privada, _ = _par_de_claves()
    assert verificar(_firmar(otra_privada)) is False


def test_tipo_distinto_se_rechaza(claves):
    # Una firma no sirve para otro tipo de evento.
    assert verificar(_firmar(claves), tipo="borrado_credencial") is False


def test_sistema_distinto_se_rechaza(claves):
    assert verificar(_firmar(claves), sistema="otro-sistema") is False


def test_timestamp_distinto_se_rechaza(claves):
    # Una firma no sirve para otro momento: evita reenviar un evento viejo con otra hora.
    assert verificar(_firmar(claves), timestamp_iso="2026-10-06T19:00:00Z") is False


def test_agente_distinto_en_la_firma_se_rechaza(claves):
    assert verificar(_firmar(claves, agente_id="otro-agente")) is False


def test_texto_todo_se_rechaza(claves):
    # Lo que manda scripts/generar_evento_prueba.py con --sin-firma.
    assert verificar("TODO") is False


def test_agente_sin_clave_publica_se_rechaza(claves):
    firma = _firmar(claves, agente_id="agente-inexistente")
    assert verificar(firma, agente_id="agente-inexistente") is False

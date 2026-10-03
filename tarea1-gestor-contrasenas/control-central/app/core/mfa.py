"""TOTP (RFC 6238) para el panel del control central. RF-11.

WebAuthn y Windows Hello no tienen endpoint: el factor que se puede demostrar es TOTP.
"""
from __future__ import annotations

import base64
import hashlib
import hmac
import secrets
import struct
import time
from urllib.parse import quote

PASO_SEGUNDOS = 30
DIGITOS = 6
VENTANA = 1


def generar_secreto() -> str:
    return base64.b32encode(secrets.token_bytes(20)).decode("ascii").rstrip("=")


def codigo_totp(secreto: str, momento: float | None = None) -> str:
    clave = _decodificar(secreto)
    contador = int((time.time() if momento is None else momento) // PASO_SEGUNDOS)
    digest = hmac.new(clave, struct.pack(">Q", contador), hashlib.sha1).digest()
    offset = digest[-1] & 0x0F
    entero = struct.unpack(">I", digest[offset : offset + 4])[0] & 0x7FFFFFFF
    return str(entero % (10**DIGITOS)).zfill(DIGITOS)


def verificar_totp(secreto: str, codigo: str, momento: float | None = None) -> bool:
    limpio = codigo.strip()
    if len(limpio) != DIGITOS or not limpio.isdigit():
        return False
    base = time.time() if momento is None else momento
    for delta in range(-VENTANA, VENTANA + 1):
        esperado = codigo_totp(secreto, base + delta * PASO_SEGUNDOS)
        if hmac.compare_digest(esperado, limpio):
            return True
    return False


def uri_otpauth(email: str, secreto: str) -> str:
    etiqueta = quote(f"ControlCentral:{email}")
    emisor = quote("ControlCentral")
    return (
        f"otpauth://totp/{etiqueta}?secret={secreto}&issuer={emisor}"
        f"&digits={DIGITOS}&period={PASO_SEGUNDOS}"
    )


def _decodificar(secreto: str) -> bytes:
    relleno = "=" * ((8 - len(secreto) % 8) % 8)
    return base64.b32decode(secreto.upper() + relleno, casefold=True)

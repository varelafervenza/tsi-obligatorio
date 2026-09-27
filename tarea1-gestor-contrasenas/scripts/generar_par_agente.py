"""Genera el par RSA de un agente (privada gitignorada, pública para el central)."""
from __future__ import annotations

import argparse
from pathlib import Path

from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.asymmetric import rsa

ROOT = Path(__file__).resolve().parents[1]
DESTINO = ROOT / "keys" / "agentes"


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--agente", default="agente-dev-01")
    args = parser.parse_args()
    DESTINO.mkdir(parents=True, exist_ok=True)

    clave = rsa.generate_private_key(public_exponent=65537, key_size=2048)
    privada = DESTINO / f"{args.agente}.pem"
    publica = DESTINO / f"{args.agente}.pub.pem"
    privada.write_bytes(
        clave.private_bytes(
            encoding=serialization.Encoding.PEM,
            format=serialization.PrivateFormat.PKCS8,
            encryption_algorithm=serialization.NoEncryption(),
        )
    )
    publica.write_bytes(
        clave.public_key().public_bytes(
            encoding=serialization.Encoding.PEM,
            format=serialization.PublicFormat.SubjectPublicKeyInfo,
        )
    )
    print(f"privada: {privada}  (no commitear)")
    print(f"publica: {publica}")


if __name__ == "__main__":
    main()

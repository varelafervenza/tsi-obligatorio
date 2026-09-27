"""Firma un evento JWS (RS256) y lo POST-ea a control-central."""
from __future__ import annotations

import argparse
import json
from datetime import datetime, timezone
from pathlib import Path
from urllib.error import HTTPError, URLError
from urllib.request import Request, urlopen

from jose import jwt

ROOT = Path(__file__).resolve().parents[1]
DESTINO = ROOT / "keys" / "agentes"
ALGORITMO = "RS256"


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--url", default="http://localhost:8000/api/events/")
    parser.add_argument("--agente", default="agente-dev-01")
    parser.add_argument("--tipo", default="alta_credencial")
    parser.add_argument("--sistema", default="sistema-prueba")
    parser.add_argument("--sin-firma", action="store_true", help="manda firma_jws=TODO")
    args = parser.parse_args()

    timestamp = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    if args.sin_firma:
        firma = "TODO"
    else:
        privada = DESTINO / f"{args.agente}.pem"
        if not privada.is_file():
            raise SystemExit(
                f"No está {privada}. Corré: python scripts/generar_par_agente.py"
            )
        firma = jwt.encode(
            {
                "agente_id": args.agente,
                "tipo": args.tipo,
                "sistema": args.sistema,
                "ts": timestamp,
            },
            privada.read_text(encoding="utf-8"),
            algorithm=ALGORITMO,
        )

    cuerpo = {
        "tipo": args.tipo,
        "sistema": args.sistema,
        "agente_id": args.agente,
        "timestamp": timestamp,
        "firma_jws": firma,
    }
    req = Request(
        args.url,
        data=json.dumps(cuerpo).encode("utf-8"),
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    try:
        with urlopen(req, timeout=10) as resp:
            print(resp.status, resp.read().decode("utf-8"))
    except HTTPError as exc:
        print(exc.code, exc.read().decode("utf-8"))
        raise SystemExit(1) from exc
    except URLError as exc:
        raise SystemExit(f"No se pudo conectar a {args.url}: {exc}") from exc


if __name__ == "__main__":
    main()

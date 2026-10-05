"""Simula los casos de uso CU-01, CU-02 y CU-03 (docs/07-Monitoreo-Logs-SIEM.md).

Manda eventos firmados con la clave del agente, como lo haría el cliente:
- CU-01: 5 intentos fallidos de maestra en menos de 2 minutos.
- CU-02: 5 borrados de credencial en menos de 2 minutos.
- CU-03: 1 cambio de contraseña maestra.
"""
from __future__ import annotations

import argparse
import json
import time
from datetime import datetime, timezone
from pathlib import Path
from urllib.error import HTTPError, URLError
from urllib.request import Request, urlopen

from jose import jwt

ROOT = Path(__file__).resolve().parents[1]
DESTINO = ROOT / "keys" / "agentes"
ALGORITMO = "RS256"


def enviar(url: str, agente: str, privada: str, tipo: str, sistema: str) -> None:
    timestamp = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    firma = jwt.encode(
        {"agente_id": agente, "tipo": tipo, "sistema": sistema, "ts": timestamp},
        privada,
        algorithm=ALGORITMO,
    )
    cuerpo = {
        "tipo": tipo,
        "sistema": sistema,
        "agente_id": agente,
        "timestamp": timestamp,
        "firma_jws": firma,
    }
    req = Request(
        url,
        data=json.dumps(cuerpo).encode("utf-8"),
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    try:
        with urlopen(req, timeout=10) as resp:
            print(f"{tipo:<26} {sistema:<18} -> {resp.status}")
    except HTTPError as exc:
        print(f"{tipo:<26} {sistema:<18} -> {exc.code} {exc.read().decode('utf-8')}")
        raise SystemExit(1) from exc
    except URLError as exc:
        raise SystemExit(f"No se pudo conectar a {url}: {exc}") from exc


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--url", default="http://localhost:8001/api/events/")
    parser.add_argument("--agente", default="agente-dev-01")
    parser.add_argument("--pausa", type=float, default=0.5, help="segundos entre eventos")
    args = parser.parse_args()

    privada_path = DESTINO / f"{args.agente}.pem"
    if not privada_path.is_file():
        raise SystemExit(f"No está {privada_path}. Corré: python scripts/generar_par_agente.py")
    privada = privada_path.read_text(encoding="utf-8")

    print("CU-01: intentos fallidos de maestra")
    for _ in range(5):
        enviar(args.url, args.agente, privada, "intento_fallido_maestra", "boveda")
        time.sleep(args.pausa)

    print("CU-02: borrado masivo de credenciales")
    for n in range(1, 6):
        enviar(args.url, args.agente, privada, "borrado_credencial", f"sistema-borrado-{n}")
        time.sleep(args.pausa)

    print("CU-03: cambio de contraseña maestra")
    enviar(args.url, args.agente, privada, "cambio_maestra", "boveda")


if __name__ == "__main__":
    main()

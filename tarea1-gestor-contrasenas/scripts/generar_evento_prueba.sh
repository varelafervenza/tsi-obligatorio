#!/usr/bin/env bash
# Firma y envía un evento de prueba (JWS RS256 por agente).
set -euo pipefail
cd "$(dirname "$0")/.."
python scripts/generar_evento_prueba.py "$@"

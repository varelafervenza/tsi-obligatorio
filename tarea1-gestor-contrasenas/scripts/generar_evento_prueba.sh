#!/usr/bin/env bash
# Envía un evento de prueba al control-central. Útil para probar el pipeline
# evento -> SIEM -> correo -> dashboard antes de tener el cliente-gestor terminado.
# TODO: reemplazar el payload por uno firmado (JWS) real una vez implementado
# cliente-gestor/src-tauri/src/events/signer.rs.
set -euo pipefail

CONTROL_CENTRAL_URL="${CONTROL_CENTRAL_URL:-http://localhost:8000}"

curl -sS -X POST "$CONTROL_CENTRAL_URL/api/events/" \
  -H "Content-Type: application/json" \
  -d '{
        "tipo": "alta_credencial",
        "sistema": "sistema-prueba",
        "agente_id": "agente-dev-01",
        "timestamp": "'"$(date -u +%Y-%m-%dT%H:%M:%SZ)"'",
        "firma_jws": "TODO"
      }'

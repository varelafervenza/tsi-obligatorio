
# 01
# 26/09/26 - Alta de credencial desde terminal -
# Al dar de alta una credencial se genera un registro correctamente.
# En terminal, desde /infra:

$body = @{
  tipo = "alta_credencial"
  sistema = "sistema-prueba"
  agente_id = "agente-dev-01"
  timestamp = (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ssZ")
  firma_jws = "TODO"
} | ConvertTo-Json
Invoke-RestMethod -Method Post -Uri http://localhost:8000/api/events/ -ContentType "application/json" -Body $body
Invoke-RestMethod http://localhost:8000/api/events/


Response:
id           : 1
tipo         : alta_credencial
sistema      : sistema-prueba
agente_id    : agente-dev-01
occurred_at  : 27/9/2026 2:00:01
received_at  : 27/9/2026 2:00:02
firma_valida :
ip_origen    : *.*.*.*


# 02
# - Verificación del forwarder JSONL en el host -
# En terminal, desde /infra:

Get-Content .\logs\audit-events.jsonl

Respuesta:
{"programa": "control-central", "event_id": 2, "tipo": "alta_credencial", "sistema": "sistema-prueba", "agente_id": "agente-dev-01", "occurred_at": "2026-09-27T02:26:32+00:00", "received_at": "2026-09-27T02:26:32.241867+00:00", "ip_origen": "172.20.0.1", "firma_valida": null}



# 03
# - Verificación de notificacion por SMTP -
# En terminal, desde /infra:

start http://localhost:8025

# Al ejecutar Alta de credencial desde terminal se envia un mail avisando el evento sucedido
# Se puede chequear en mailpit

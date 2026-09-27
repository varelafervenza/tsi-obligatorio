
# 01
# 26/09/26 - Alta de credencial desde terminal -
# Al dar de alta una credencial se genera un registro correctamente.

En terminal, desde /infra:

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











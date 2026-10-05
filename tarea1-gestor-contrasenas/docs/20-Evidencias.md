
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

# 04
# 05/10/26 - Demo funcional del control central (puerto 8001) -
# Stack levantado con `docker compose up` desde /infra. Evento de prueba generado con
# `python scripts/generar_evento_prueba.py` desde la raíz de la tarea.

## 04-01 — Documentación interactiva de la API
- Archivo: `docs/evidencias/01-api-docs.png`
- Muestra: `http://localhost:8001/docs` con todas las rutas: events, alerts, incidents, users,
  dashboard y healthz.
- Qué demuestra: la API está viva y expone los endpoints que usa la demo.

## 04-02 — Evento firmado aceptado
- Archivo: `docs/evidencias/02-evento-firmado.png`
- Comando: `python scripts/generar_evento_prueba.py`
- Respuesta: `201` con `"firma_valida":true` (evento id 3, agente `agente-dev-01`).
- Qué demuestra: el control central verifica la firma JWS con la clave pública del agente y
  persiste el evento.

## 04-03 — Notificaciones por correo
- Archivo: `docs/evidencias/03-mailpit.png`
- Muestra: Mailpit en `http://localhost:8025` con tres correos "Alta de credencial" (ids 1, 2 y 3)
  para `rsi@correo.local`.
- Qué demuestra: RF-07 en el prototipo (Mailpit como sustituto de Mailu, ver la limitación de
  alcance del correo en `00-arquitectura-c4.md`).

## 04-04 — Tablero de Grafana
- Archivo: `docs/evidencias/04-grafana.png`
- Muestra: el tablero "Control central" con Eventos = 3, Agentes = 1, Incidentes abiertos = 0,
  "Última sincronización: hace 7 minutos" y el volumen por tipo (alta_credencial = 3).
- Qué demuestra: RF-09 y RF-14. El panel de incidentes dice "No data" porque todavía no hay
  incidentes. El panel de sincronización se corrigió en el commit aa247e2 (ver bitácora).

# 05
# 05/10/26 - Prueba del cliente de escritorio contra el control central -
# Desde la app instalada en Windows (con Smart App Control desactivado en el equipo de prueba).
# Bóveda de prueba creada, dos altas de credencial, carpeta de claves del central configurada.

## 05-01 — Alta antes de instalar la clave del agente
- Evento 4, sistema `sistema-app`, agente `agente-f3815a26`, `firma_valida: false`.
- Qué demuestra: el central rechaza una firma que no puede verificar, porque la clave pública del
  agente todavía no está en `keys/agentes/`.

## 05-02 — Alta con la clave instalada
- Evento 5, sistema `sistema-app-2`, agente `agente-f3815a26`, `firma_valida: true`.
- Qué demuestra: el cliente real firma el evento con su clave privada y el central lo verifica
  con la clave pública que la app copió con el botón "Guardar" de la sección Control central.

## 05-03 — Correos y tablero
- Mailpit: 5 correos de alta en total (tres de la demo anterior y dos de la app).
- Grafana: Eventos = 5, Agentes = 2, "Última sincronización: hace un minuto".
- Capturas guardadas en `docs/evidencias/`:
  - `05-01-eventos-firma.png`: `http://localhost:8001/api/events/` con el evento 5 (`firma_valida: true`)
    y el evento 4 (`firma_valida: false`).
  - `05-02-app-control-central.png`: la app en la sección Control central, con la URL de eventos,
    la carpeta de claves configurada y el mensaje "Evento enviado al control central".
  - `05-03-app-alta.png`: la app con la bóveda abierta y la lista de credenciales con `sistema-app`
    y `sistema-app-2`.
  - `05-03-mailpit.png`: Mailpit con 5 correos de alta, incluidos los ids 4 y 5 de la app.
  - `05-04-grafana.png`: tablero con Eventos = 5, Agentes = 2 y "Última sincronización: hace 18 minutos".

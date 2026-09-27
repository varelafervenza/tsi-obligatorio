# Wazuh (SIEM/HIDS)

No se incluye un `docker-compose.yml` inline acá porque el stack oficial de Wazuh
(manager + indexer + dashboard, con certificados TLS generados) es extenso y cambia
de versión en versión. Usar el generador oficial:

```bash
git clone https://github.com/wazuh/wazuh-docker.git -b v4.9.0
cd wazuh-docker/single-node
docker compose up -d
```

Después:
1. Conectar el manager de Wazuh a la red `blue-team-net` de `infra/docker-compose.yml`
   (o exponer el puerto del manager y apuntar los agentes ahí).
2. Instalar el **agente Wazuh** en la VM del cliente-gestor (o del control-central si
   se quiere monitorear también ese host) para FIM sobre el archivo de bóveda.
3. Reglas custom a crear en `/var/ossec/etc/rules/local_rules.xml` del manager (RF-10):
   - Fuerza bruta de contraseña maestra (varios intentos fallidos en ventana corta).
   - Borrado masivo de credenciales.
   - Cambio de contraseña maestra (regla de severidad alta, dispara notificación).
4. Documentar las reglas y capturas de alertas en `docs/07-Monitoreo-Logs-SIEM.md`.

## Formato de log (ya emitido por control-central)

Cada `POST /api/events/` agrega **una línea JSON** en
`infra/logs/audit-events.jsonl` (dentro del contenedor:
`/var/log/control-central/audit-events.jsonl`). Campos: `programa`, `event_id`,
`tipo`, `sistema`, `agente_id`, `occurred_at`, `received_at`, `ip_origen`,
`firma_valida`. Sin secretos ni `firma_jws`.

Cuando exista el manager, configurar `localfile` apuntando a ese path (o copiar
el JSONL al agente). Las 3 reglas RF-10 se escriben después, leyendo `tipo`:
`intento_fallido_maestra`, `borrado_credencial`, `cambio_maestra`.

## Pendiente

- [x] Formato de log del forwarder (`wazuh_forwarder.py` → JSONL).
- [ ] Crear y probar las 3 reglas custom de RF-10.
- [ ] Configurar retención ≥ 90 días (RNF-07).

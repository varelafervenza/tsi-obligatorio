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
3. Copiar `local_rules.xml` a `/var/ossec/etc/rules/local_rules.xml` del manager (RF-10)
   y el fragmento `localfile-audit.xml` al `ossec.conf` que lee el JSONL.
4. Documentar las capturas de alertas en `docs/07-Monitoreo-Logs-SIEM.md`.

## Formato de log (ya emitido por control-central)

Cada `POST /api/events/` agrega **una línea JSON** en
`infra/logs/audit-events.jsonl` (dentro del contenedor:
`/var/log/control-central/audit-events.jsonl`). Campos: `programa`, `event_id`,
`tipo`, `sistema`, `agente_id`, `occurred_at`, `received_at`, `ip_origen`,
`firma_valida`. Sin secretos ni `firma_jws`.

Cuando exista el manager, pegar `localfile-audit.xml` en el `ossec.conf` que lee
ese JSONL. Las reglas ya están en `local_rules.xml` y matchean `tipo`:
`intento_fallido_maestra`, `borrado_credencial`, `cambio_maestra`.

## Pendiente

- [x] Formato de log del forwarder (`wazuh_forwarder.py` → JSONL).
- [x] Reglas escritas en `local_rules.xml` (fuerza bruta, borrado masivo, cambio de maestra). Falta dispararlas con el manager levantado.
- [ ] Configurar retención ≥ 90 días (RNF-07).

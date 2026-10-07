# Wazuh (SIEM/HIDS)

## Cómo levantarlo

Desde `infra/`, con Docker Desktop abierto. Es el mismo compose del control central; no hace
falta clonar `wazuh-docker`.

```powershell
copy .env.example .env
copy control-central.env.example control-central.env
docker compose up --build -d
```

La primera vez descarga `wazuh/wazuh-manager:4.14.8` y `wazuh/wazuh-agent:4.14.8`. El agente
no arranca hasta que el manager está healthy (cerca de un minuto).

```powershell
docker exec infra-wazuh-manager-1 /var/ossec/bin/agent_control -l
```

Esperado: `agente-control-central` **Active**. Después, una línea nueva en
`logs/audit-events.jsonl` con `"tipo": "cambio_maestra"` tiene que generar la regla 100120.
Fuerza bruta (100101) y borrado masivo (100111) piden 5 eventos del mismo `agente_id` en
2 minutos. El panel las suma en `GET /api/dashboard/kpis` → `alertas_siem`.

```powershell
docker exec infra-wazuh-manager-1 tail -n 5 /var/ossec/logs/alerts/alerts.json
```

| Pieza | Imagen | Rol |
|---|---|---|
| `wazuh-manager` | `wazuh/wazuh-manager:4.14.8` | Aplica `local_rules.xml` y escribe `alerts.json` |
| `wazuh-agent` | `wazuh/wazuh-agent:4.14.8` | Lee `infra/logs/audit-events.jsonl` y lo manda al manager |

La 4.9 que figuraba antes no publica imagen de agente. Manager y agente van en la misma
4.14.8 para que el enrolamiento cierre.

## Qué no está

El indexer y el dashboard de Wazuh no se levantan: el indexer pide cerca de 1 GB de heap y
esta Docker ya comparte memoria con el resto del stack. Sin indexer, Filebeat sale y la
imagen oficial apaga el manager. `manager/01-laboratorio.sh` deja Filebeat en espera para
que el manager siga vivo. Las alertas quedan en el volumen `wazuh-alerts`
(`/var/ossec/logs/alerts/alerts.json`). El panel las cuenta en `alertas_siem` de
`GET /api/dashboard/kpis` (reglas 100100, 100101, 100110, 100111 y 100120).

Tampoco hay agente en el Windows del usuario. El FIM de la bóveda local sigue sin
desplegar. El agente de este compose vigila el JSONL de auditoría, no el archivo de la bóveda.

Sin puertos publicados en el host (07/10): el manager y el agente comparten `blue-team-net`
y se hablan por el nombre del servicio. Antes se publicaban 1514, 1515 y 55000 (API del
manager, con el usuario y la contraseña por defecto, `wazuh`/`wazuh`); nada del laboratorio
los usaba desde el host, así que se quitaron (R08 en `docs/03-Analisis-Riesgos.md`). Si se
agrega un agente Wazuh real en el Windows del usuario, hay que volver a publicar 1514 y 1515
en `infra/docker-compose.yml`.

## Retención

- [ ] Retención del JSONL y de `alerts.json` en 90 días o más (RNF-07). La de PostgreSQL ya corre al arrancar el central.

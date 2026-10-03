# Grafana provisioning

Datasources (Postgres, y Elasticsearch/Wazuh si se usa Kibana en paralelo) y dashboards
provisionados como código, para que el dashboard de KPIs (RF-09, sección 6.4 de
LETRA.md) se reconstruya con `docker compose up` sin pasos manuales.

## Estado

- [x] `datasources/postgres.yml` apunta al Postgres del compose (`postgres:5432`).
- [x] Tablero `dashboards/control-central.json`: eventos, incidentes abiertos, agentes, última sincronización, volumen y listado.
- [ ] MTTD, MTTR, cobertura y uptime se leen en `GET /api/dashboard/kpis`, no en el tablero. La tasa de falsos positivos queda vacía hasta que Wazuh emita alertas.

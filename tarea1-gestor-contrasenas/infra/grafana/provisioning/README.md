# Grafana provisioning

Datasources (Postgres, y Elasticsearch/Wazuh si se usa Kibana en paralelo) y dashboards
provisionados como código, para que el dashboard de KPIs (RF-09, sección 6.4 de
LETRA.md) se reconstruya con `docker compose up` sin pasos manuales.

## Pendiente

- [ ] `datasources/postgres.yml` apuntando a `control-central`/postgres.
- [ ] Dashboard con: MTTD, MTTR, cobertura de eventos, tasa de falsos positivos, uptime.

# TheHive (gestión de casos/incidentes)

TheHive 5 requiere Cassandra + Elasticsearch propios (no corre standalone). Evaluar
si el tiempo del curso alcanza para este componente completo o si conviene resolver
RF-13 (registro de incidentes con estado abierto/en análisis/resuelto) directamente
en `control-central` (tabla `Incident` + endpoints) y dejar TheHive como mejora.

## Si se decide usar TheHive

```bash
git clone https://github.com/StrangeBee/TheHive.git -b 5.2 thehive-official
cd thehive-official/docker
docker compose up -d
```

Conectar a `blue-team-net` y a Wazuh Active Response / Alertmanager para creación
automática de casos ante alertas críticas (cambio de maestra, borrado masivo).

## Pendiente

- [ ] Decidir TheHive vs. tabla `Incident` propia (documentar la decisión en el análisis
      de riesgos / arquitectura).

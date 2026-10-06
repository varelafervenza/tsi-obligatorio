#!/usr/bin/env bash
# Prueba de restauración: restaura el último backup en una base separada y compara los conteos
# contra la base original. No toca la base de producción del laboratorio.
set -euo pipefail

RAIZ="$(cd "$(dirname "$0")/.." && pwd)"
DESTINO="$RAIZ/infra/backups"
CONTENEDOR="${CONTENEDOR_BD:-infra-postgres-1}"
BASE="${BASE_BD:-control_central}"
BASE_PRUEBA="${BASE_PRUEBA:-control_central_restore}"
USUARIO="${USUARIO_BD:-gestor}"

dump="${1:-$(ls -1t "$DESTINO"/*.dump | head -1)}"
echo "restaurando: $dump"

echo "verificando sha256:"
sha256sum -c "$dump.sha256"

docker exec "$CONTENEDOR" psql -U "$USUARIO" -d postgres -qc "DROP DATABASE IF EXISTS $BASE_PRUEBA;"
docker exec "$CONTENEDOR" psql -U "$USUARIO" -d postgres -qc "CREATE DATABASE $BASE_PRUEBA;"

inicio=$(date +%s)
docker exec -i "$CONTENEDOR" pg_restore -U "$USUARIO" -d "$BASE_PRUEBA" --no-owner < "$dump"
fin=$(date +%s)
echo "tiempo de restauracion: $((fin - inicio)) s"

echo "conteos (original vs restaurada):"
for tabla in audit_events alerts incidents panel_users; do
  original=$(docker exec "$CONTENEDOR" psql -U "$USUARIO" -d "$BASE" -tAc "select count(*) from $tabla")
  restaurada=$(docker exec "$CONTENEDOR" psql -U "$USUARIO" -d "$BASE_PRUEBA" -tAc "select count(*) from $tabla")
  estado=$([ "$original" = "$restaurada" ] && echo OK || echo DIFIERE)
  echo "  $tabla: original=$original restaurada=$restaurada $estado"
done

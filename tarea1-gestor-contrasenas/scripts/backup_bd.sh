#!/usr/bin/env bash
# Backup lógico de PostgreSQL del control central (docs/06-Plan-Continuidad.md).
# Genera un dump comprimido en infra/backups/ con fecha y su SHA-256.
set -euo pipefail

RAIZ="$(cd "$(dirname "$0")/.." && pwd)"
DESTINO="$RAIZ/infra/backups"
CONTENEDOR="${CONTENEDOR_BD:-infra-postgres-1}"
BASE="${BASE_BD:-control_central}"
USUARIO="${USUARIO_BD:-gestor}"

mkdir -p "$DESTINO"
archivo="$DESTINO/${BASE}-$(date -u +%Y%m%dT%H%M%SZ).dump"

docker exec "$CONTENEDOR" pg_dump -U "$USUARIO" -d "$BASE" -Fc > "$archivo"
sha256sum "$archivo" > "$archivo.sha256"

echo "backup: $archivo"
echo "tamano: $(wc -c < "$archivo") bytes"
echo "sha256: $(cut -d' ' -f1 "$archivo.sha256")"

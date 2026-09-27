#!/usr/bin/env bash
# Calcula el hash SHA-256 del árbol del repo en el tag v1.0, para que el Red Team
# pueda verificar que la entrega congelada (07/10) no se modificó (sección 8.2 de LETRA.md).
set -euo pipefail

TAG="${1:-v1.0}"

git rev-parse "$TAG" >/dev/null 2>&1 || { echo "El tag $TAG no existe" >&2; exit 1; }

echo "Commit del tag $TAG: $(git rev-parse "$TAG")"
echo "Hash SHA-256 del árbol:"
git archive "$TAG" | sha256sum

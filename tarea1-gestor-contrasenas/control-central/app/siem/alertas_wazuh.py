"""Cuenta alertas de Wazuh en alerts.json. RF-10.

Solo entran las reglas de infra/wazuh/local_rules.xml. Si el archivo no está
(tests, o el manager todavía no arrancó), el conteo es 0.
"""
from __future__ import annotations

import json
from pathlib import Path

REGLAS = frozenset({"100100", "100101", "100110", "100111", "100120"})


def contar_alertas(path: Path) -> int:
    if not path.is_file():
        return 0
    total = 0
    with path.open(encoding="utf-8", errors="replace") as archivo:
        for linea in archivo:
            linea = linea.strip()
            if not linea:
                continue
            try:
                alerta = json.loads(linea)
            except json.JSONDecodeError:
                continue
            regla = str((alerta.get("rule") or {}).get("id", ""))
            if regla in REGLAS:
                total += 1
    return total

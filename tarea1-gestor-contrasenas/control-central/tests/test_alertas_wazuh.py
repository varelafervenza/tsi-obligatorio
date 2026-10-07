import json
from pathlib import Path

from app.siem.alertas_wazuh import contar_alertas


def _linea(regla: str) -> str:
    return json.dumps({"rule": {"id": regla, "level": 10}})


def test_cuenta_solo_las_reglas_del_gestor(tmp_path: Path):
    archivo = tmp_path / "alerts.json"
    archivo.write_text(
        "\n".join(
            [
                _linea("100120"),
                _linea("5710"),
                "no es json",
                _linea("100101"),
            ]
        ),
        encoding="utf-8",
    )

    assert contar_alertas(archivo) == 2


def test_sin_archivo_el_conteo_es_cero(tmp_path: Path):
    assert contar_alertas(tmp_path / "no-existe.json") == 0

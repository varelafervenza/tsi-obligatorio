"""Actualiza los Excel MCU (#14 evidencias, #15 bitácora) a partir de los docs.

Uso, desde la raíz de la tarea (tarea1-gestor-contrasenas/):

    python -m pip install openpyxl
    python scripts/actualizar_excels_mcu.py

Reescribe 04-bitacora-planilla.xlsx con todas las entradas de 99-bitacora-trabajo.md
y reemplaza celdas de evidencia/demostración que todavía dicen "pendiente" en
01-controles-mcu5-perfil-avanzado.xlsx, según la función de la hoja.
"""
from __future__ import annotations

import re
from pathlib import Path

from openpyxl import load_workbook
from openpyxl.styles import Alignment, Font

RAIZ = Path(__file__).resolve().parents[1]
DOCS = RAIZ / "docs"
XLSX_CONTROLES = DOCS / "mcu5" / "excel" / "01-controles-mcu5-perfil-avanzado.xlsx"
XLSX_BITACORA = DOCS / "mcu5" / "excel" / "04-bitacora-planilla.xlsx"
MD_BITACORA = DOCS / "99-bitacora-trabajo.md"

# Evidencia por función MCU. No inventa controles cerrados: si sigue abierto, lo dice.
EVIDENCIA_POR_FUNCION = {
    "gobernar": (
        "docs/01-Politica-Seguridad.md; docs/03-Analisis-Riesgos.md; "
        "docs/mcu5/excel/03-matriz-raci-mcu5.xlsx; docs/99-bitacora-trabajo.md. "
        "Falta firma del RSI (pendiente #12)."
    ),
    "identificar": (
        "docs/02-Registro-Activos.md; docs/03-Analisis-Riesgos.md; "
        "docs/10-Gestion-Vulnerabilidades.md; docs/evidencias/10-*.txt. "
        "V03 remediada (FastAPI 0.142.3). Escaneo de red/Trivy pendientes."
    ),
    "proteger": (
        "docs/09-Gestion-Accesos.md; cliente-gestor (Argon2id, XChaCha20, TOTP); "
        "docs/evidencias/05-03-app-alta.png; guia-de-pruebas.md C-09. "
        "Hello/WebAuthn y TLS: limitación documentada."
    ),
    "detectar": (
        "docs/07-Monitoreo-Logs-SIEM.md; infra/wazuh/local_rules.xml; "
        "docs/evidencias/07-simulacion-casos-uso.txt; Grafana :3000; "
        "Wazuh 4.14.8 reglas 100120/100101/100111. Sin indexer."
    ),
    "responder": (
        "docs/04-Gestion-Incidentes.md; docs/12-Notificacion-Incidentes.md; "
        "POST/PATCH /api/incidents/; incidente 1 en docs/evidencias/06-restauracion-bd-con-datos.txt. "
        "Falta firma RSI y contactos oficiales."
    ),
    "recuperar": (
        "docs/06-Plan-Continuidad.md; scripts/backup_bd.sh; "
        "docs/evidencias/06-restauracion-bd.txt y 06-restauracion-bd-con-datos.txt. "
        "Backup diario y copia fuera del equipo pendientes."
    ),
}

DEMOSTRAR_POR_FUNCION = {
    "gobernar": "Abrir 01, 03 y la RACI; recorrer roles. La firma del RSI se muestra si ya está.",
    "identificar": "Abrir 02 y 03; mostrar pip-audit/bandit en docs/evidencias/10-*.",
    "proteger": "Abrir la app, TOTP, alta firmada (C-04). Declarar Hello/TLS como limitación.",
    "detectar": "Mostrar /api/alerts/, Grafana y docker exec ... tail alerts.json (regla 100120).",
    "responder": "GET /api/incidents/ y el incidente 1 en 12 y en la restauración.",
    "recuperar": "Repetir scripts/restaurar_bd_prueba.sh y mostrar el SHA-256 del dump.",
}


def _norm(valor: object) -> str:
    return re.sub(r"\s+", " ", str(valor or "")).strip().lower()


def _funcion_de_hoja(nombre: str) -> str | None:
    n = _norm(nombre)
    for clave in EVIDENCIA_POR_FUNCION:
        if clave in n:
            return clave
    return None


def _funcion_de_texto(*textos: object) -> str | None:
    n = " ".join(_norm(t) for t in textos)
    for clave in EVIDENCIA_POR_FUNCION:
        if clave in n:
            return clave
    if any(p in n for p in ("politica", "raci", "rol", "gobierno")):
        return "gobernar"
    if any(p in n for p in ("activ", "riesgo", "vulnerab", "inventario")):
        return "identificar"
    if any(p in n for p in ("totp", "rbac", "acceso", "mfa", "jws", "cifr", "boveda", "argon", "clave")):
        return "proteger"
    if any(p in n for p in ("siem", "wazuh", "alerta", "log", "grafana", "detect")):
        return "detectar"
    if any(p in n for p in ("incident", "notif", "respond")):
        return "responder"
    if any(p in n for p in ("backup", "continuid", "restaur", "recuper")):
        return "recuperar"
    return None


def _es_pendiente(texto: object) -> bool:
    return "pendiente" in _norm(texto)


def _textos_para(funcion: str, control: object, evidencia_actual: object, demo_actual: object) -> tuple[str, str]:
    n = _norm(control) + " " + _norm(evidencia_actual) + " " + _norm(demo_actual)
    ev = EVIDENCIA_POR_FUNCION[funcion]
    demo = DEMOSTRAR_POR_FUNCION[funcion]
    if "rsi" in n and ("aprob" in n or "firma" in n or "aceptac" in n):
        ev = "docs/01-Politica-Seguridad.md y docs/03-Analisis-Riesgos.md (borradores). Falta la firma del RSI (#12)."
        demo = "Mostrar el documento. La aprobación formal del RSI sigue pendiente (Andrés)."
    elif "soa" in n or "aplicabilidad" in n or "11-soa" in n:
        ev = "docs/11-SoA-Plan-Tratamiento.md (93 controles, 07/10). Falta firma del RSI."
        demo = "Abrir la SoA y recorrer un control Implementado y uno N.A."
    elif "vulnerab" in n or "sast" in n or "openvas" in n or "10-gestion" in n:
        ev = "docs/10-Gestion-Vulnerabilidades.md; evidencias 10-pip-audit*.txt, 10-bandit.txt, 10-npm-audit.txt. V03 remediada. Red/Trivy/cargo audit pendientes."
        demo = "Mostrar el registro V01-V04 y los txt de evidencias. Declarar que nmap/Trivy no se corrieron."
    elif "rbac" in n or "users.py" in n or "alta/baja" in n:
        ev = "control-central/app/api/users.py; guia-de-pruebas.md B-14; usuario rsi@correo.local. Roles admin/auditor/rsi."
        demo = "POST /api/users/ y login con TOTP (B-14). Ya no es un TODO."
    elif "totp" in n or "enroll" in n or "multifactor" in n or "mfa" in n:
        ev = "docs/09-Gestion-Accesos.md; guia C-09 (bóveda) y B-14 (panel). Hello/WebAuthn fuera de alcance."
        demo = "Generar TOTP en la app y confirmar; login del panel con código de 6 dígitos."
    elif "jws" in n or "agente" in n and "clave" in n:
        ev = "cliente-gestor/src-tauri/src/events/; keys/agentes/; evidencias 05-01-eventos-firma.png y 05-03-app-modificacion.png."
        demo = "Alta/mod/borrado desde la app con firma_valida true (C-04)."
    return ev, demo


def actualizar_controles() -> int:
    wb = load_workbook(XLSX_CONTROLES)
    cambiadas = 0
    for hoja in wb.worksheets:
        encabezado = {_norm(c.value): idx for idx, c in enumerate(next(hoja.iter_rows(min_row=1, max_row=1)), start=1)}
        col_fun = next((i for k, i in encabezado.items() if "funci" in k), None)
        col_ctrl = next((i for k, i in encabezado.items() if "resultado" in k or "control" in k), 2)
        col_ev = next((i for k, i in encabezado.items() if "evidencia" in k), None)
        col_demo = next((i for k, i in encabezado.items() if "demuestra" in k or "demostr" in k), None)
        if not col_ev and not col_demo:
            continue
        for fila in hoja.iter_rows(min_row=2):
            fun = _funcion_de_texto(
                fila[col_fun - 1].value if col_fun else "",
                fila[col_ctrl - 1].value if col_ctrl else "",
            ) or _funcion_de_hoja(hoja.title)
            if not fun:
                continue
            ev_celda = fila[col_ev - 1] if col_ev else None
            demo_celda = fila[col_demo - 1] if col_demo else None
            hay = (ev_celda and _es_pendiente(ev_celda.value)) or (
                demo_celda and _es_pendiente(demo_celda.value)
            )
            if not hay:
                continue
            ev, demo = _textos_para(
                fun,
                fila[col_ctrl - 1].value if col_ctrl else "",
                ev_celda.value if ev_celda else "",
                demo_celda.value if demo_celda else "",
            )
            if ev_celda and _es_pendiente(ev_celda.value):
                ev_celda.value = ev
                ev_celda.alignment = Alignment(wrap_text=True, vertical="top")
            if demo_celda and _es_pendiente(demo_celda.value):
                demo_celda.value = demo
                demo_celda.alignment = Alignment(wrap_text=True, vertical="top")
            cambiadas += 1
    wb.save(XLSX_CONTROLES)
    return cambiadas


def parsear_bitacora(texto: str) -> list[dict[str, str]]:
    bloques = re.split(r"\n---\n", texto)
    entradas: list[dict[str, str]] = []
    actual: dict[str, str] = {}
    for bloque in bloques:
        meta = {
            "fecha": _campo(bloque, r"^Fecha:\s*(.+)$"),
            "responsable": _campo(bloque, r"^Responsable:\s*(.+)$"),
            "hora": _campo(bloque, r"^Hora \(UTC\):\s*(.+)$"),
        }
        if meta["fecha"] or meta["responsable"]:
            actual = meta
        titulo = re.search(r"^## Actividad:\s*(.+)$", bloque, re.M)
        if not titulo:
            continue
        entradas.append(
            {
                "fecha": actual.get("fecha", "N/D"),
                "hora": actual.get("hora", "N/D"),
                "fase": _campo(bloque, r"\*\*Fase\*\*:\s*(.+)"),
                "responsable": actual.get("responsable", "N/D"),
                "tarea": titulo.group(1).strip() + " — " + _campo(bloque, r"\*\*Tarea realizada\*\*:\s*(.+)"),
                "herramienta": _campo(bloque, r"\*\*Herramienta / comando\*\*:\s*(.+)"),
                "resultado": _campo(bloque, r"\*\*Resultado\*\*:\s*(.+)"),
                "evidencia": _campo(bloque, r"\*\*Evidencia anexa\*\*:\s*(.+)"),
                "incidencia": "S" if "ninguna" not in _norm(_campo(bloque, r"\*\*Incidencia / hallazgo\*\*:\s*(.+)")) else "N",
                "observaciones": _campo(bloque, r"\*\*Observaciones\*\*:\s*(.+)"),
            }
        )
    return entradas


def _campo(bloque: str, patron: str) -> str:
    m = re.search(patron, bloque, re.M)
    if not m:
        return "N/D"
    resto = m.group(1).strip()
    # El valor puede seguir en líneas indentadas.
    lineas = [resto]
    despues = bloque[m.end() :]
    for linea in despues.splitlines():
        if linea.startswith("- **") or linea.startswith("## ") or linea.startswith("---"):
            break
        if linea.startswith("  ") or linea.startswith("\t"):
            lineas.append(linea.strip())
        elif linea.strip() == "":
            break
    return " ".join(lineas).strip() or "N/D"


def actualizar_bitacora() -> int:
    entradas = parsear_bitacora(MD_BITACORA.read_text(encoding="utf-8"))
    wb = load_workbook(XLSX_BITACORA)
    hoja = wb.active
    # Conserva la fila 1 (encabezados del template). Borra el resto.
    if hoja.max_row > 1:
        hoja.delete_rows(2, hoja.max_row - 1)
    encabezados = [_norm(c.value) for c in hoja[1]]
    # Orden canónico si el template coincide; si no, escribe en las primeras 10 columnas.
    valores_por_clave = [
        ("fecha", "fecha"),
        ("hora", "hora"),
        ("fase", "fase"),
        ("responsable", "responsable"),
        ("tarea", "tarea"),
        ("herramienta", "herramienta"),
        ("resultado", "resultado"),
        ("evidencia", "evidencia"),
        ("incidencia", "incidencia"),
        ("observaciones", "observaciones"),
    ]
    for entrada in entradas:
        fila = []
        if any(encabezados):
            for header in encabezados:
                clave = next((k for k, pista in valores_por_clave if pista in header), None)
                fila.append(entrada.get(clave, "") if clave else "")
        else:
            fila = [entrada[k] for k, _ in valores_por_clave]
        hoja.append(fila)
        for celda in hoja[hoja.max_row]:
            celda.alignment = Alignment(wrap_text=True, vertical="top")
            celda.font = Font(name="Calibri", size=10)
    wb.save(XLSX_BITACORA)
    return len(entradas)


def diagnosticar_controles() -> None:
    wb = load_workbook(XLSX_CONTROLES, data_only=True)
    print(f"archivo: {XLSX_CONTROLES}")
    print(f"hojas: {wb.sheetnames}")
    for hoja in wb.worksheets:
        funcion = _funcion_de_hoja(hoja.title)
        encabezados = [(idx, c.value) for idx, c in enumerate(next(hoja.iter_rows(min_row=1, max_row=1)), start=1)]
        print(f"\n=== {hoja.title!r} funcion={funcion} max_row={hoja.max_row} ===")
        print("fila1:", encabezados)
        pendientes = 0
        for r in range(1, min(hoja.max_row, 80) + 1):
            for c in range(1, min(hoja.max_column, 20) + 1):
                val = hoja.cell(r, c).value
                if val and "pendiente" in _norm(val):
                    pendientes += 1
                    if pendientes <= 8:
                        print(f"  pendiente r{r}c{c}: {str(val)[:160]}")
        print(f"  celdas con 'pendiente' (primeras 80 filas): {pendientes}")


def main() -> None:
    import sys

    if "--diagnostico" in sys.argv:
        diagnosticar_controles()
        return
    n_bit = actualizar_bitacora()
    n_ctrl = actualizar_controles()
    print(f"bitacora: {n_bit} entradas en {XLSX_BITACORA.name}")
    print(f"controles: {n_ctrl} filas de evidencia actualizadas en {XLSX_CONTROLES.name}")


if __name__ == "__main__":
    main()

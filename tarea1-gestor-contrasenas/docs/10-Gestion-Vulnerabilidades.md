# Gestión de Vulnerabilidades — Gestor de Contraseñas con Control Centralizado

> Basado en `plantilla/isaca/10-gestion-vulnerabilidades.md`. Función **Identificar / Proteger** del MCU 5.0.
> Este documento registra sólo los escaneos que realmente se corrieron, con su fecha y su salida en
> `docs/evidencias/`. Lo que no se corrió aparece como pendiente.

---

## Encabezado de mapeo normativo

| Marco | Ítem | Detalle / aporte |
|---|---|---|
| **MCU 5.0 (función)** | Identificar + Proteger | ID-03 (vulnerabilidades), PR-01 (remediación). |
| **COBIT 2019** | APO12, DSS05 | Gestión de vulnerabilidades y parches. |
| **ISO/IEC 27001:2022** | A.8.8 | Gestión de vulnerabilidades técnicas. |
| **BCU — GSI** | Gestión de vulnerabilidades y parches | Remediación de vulnerabilidades críticas. |
| **URCDP — Ley 18.331, Art. 9** | Seguridad de datos personales | La remediación previene incidentes con datos personales. |

---

## Control del documento

| Campo | Valor |
|---|---|
| Código | SI-VUL-10 |
| Versión | 0.2 (borrador) |
| Responsable | Blue Team (Horacio Duarte, Pablo Morales, Andrés Varela) |
| Fecha | 11/10/2026 |

---

## 1. Herramientas utilizadas

| Herramienta | Qué analiza | Corrida | Evidencia | Estado |
|---|---|---|---|---|
| `pip-audit` | Dependencias de Python de `control-central` | 05/10/2026 (antes y después de actualizar) | `10-pip-audit.txt`, `10-pip-audit-despues.txt` | Corrido |
| `npm audit --omit=dev` | Dependencias de producción de `cliente-gestor` | 05/10/2026 | `10-npm-audit.txt` | Corrido |
| `bandit` (SAST) | Código Python de `control-central/app` | 05/10/2026 | `10-bandit.txt` | Corrido |
| `cargo audit` | Dependencias de Rust de `cliente-gestor` | — | — | **Pendiente** |
| nuclei v3.11.1 (7078 plantillas) | HTTP a :8001, :3000, :8025 vía `host.docker.internal` | 11/10/2026 | `10-nuclei.txt` | Corrido. :8001 casi sin cobertura (timeout). OpenVAS no se corrió |
| Trivy / semgrep | Imágenes Docker y SAST adicional | — | — | **Pendiente** |

---

## 2. Registro de vulnerabilidades

`pip-audit` no informa CVSS. Los puntajes de V03 y V04 se tomaron de OSV/NVD el 11/10/2026
(detalle y URLs en `docs/evidencias/10-cvss-v03-v04.txt`). V01 y V02 siguen sin puntaje
oficial cargado: no son el pendiente #24.

| ID | Activo | Hallazgo | Herramienta | CVSS v3 | Estado | Fecha detección | Fecha remediación | Responsable |
|---|---|---|---|---|---|---|---|---|
| V01 | `control-central` (`python-jose` 3.3.0) | PYSEC-2024-232, PYSEC-2024-233, PYSEC-2025-185 | pip-audit | Pendiente | **Remediado**: se subió a 3.4.0 | 05/10/2026 | 05/10/2026 | Pablo Morales |
| V02 | `control-central` (`python-multipart` 0.0.9) | Siete avisos PYSEC-2026-1851 a 3040 | pip-audit | Pendiente | **Remediado**: se subió a 0.0.31 | 05/10/2026 | 05/10/2026 | Pablo Morales |
| V03 | `control-central` (hallazgo: `starlette` 0.38.6 vía FastAPI 0.115.0) | Siete avisos PYSEC-2026. El 07/10 se subió a FastAPI 0.142.3 y starlette 1.7.0 | pip-audit | **7.5 (Alto)** — peor v3.1 del grupo | **Remediado.** 18 tests pasan. Un `pip-audit` nuevo no pudo consultar PyPI en esta máquina (el certificado no verifica) | 05/10/2026 | 07/10/2026 | Pablo Morales |
| V04 | `control-central` (`ecdsa` 0.19.2, dependencia de `python-jose`) | PYSEC-2026-1325 (CVE-2024-23342) | pip-audit | **7.4 (Alto)** `AV:N/AC:H/PR:N/UI:N/S:U/C:H/I:H/A:N` | **Riesgo aceptado** (ver sección 5) | 05/10/2026 | — | Pablo Morales |
| V05 | Grafana 11.2.0 (`:3000`) | CVE-2025-4123 (XSS / open redirect). Fix ≥ 11.2.9+security-01 | nuclei | **6.1 (Media)** NVD; vendor 7.6 | **Aceptado** en laboratorio (sección 5) | 11/10/2026 | — | Pablo Morales |
| V06 | Mailpit v1.21 (`:8025`) | CVE-2026-21859 (SSRF en `/proxy`). Fix 1.28.1 | nuclei | **5.3 (Media)** NVD; CNA 5.8 | **Aceptado** en laboratorio (sección 5) | 11/10/2026 | — | Pablo Morales |
| V07 | Grafana `:3000/metrics` | Expone versión 11.2.0 (y métricas). Sin CVE | nuclei | Baja (info) | **Aceptado** | 11/10/2026 | — | Pablo Morales |

Desglose de V03 (`starlette` 0.38.6). La fila usa el peor CVSS v3.1 (7.5). CVE-2024-47874 no tiene
base v3 en NVD; OSV publica CVSS v4 8.7 y un v3 0.0 (vector con impacto nulo, no se usa).

| PYSEC | Alias | CVSS v3.1 | Fuente |
|---|---|---|---|
| PYSEC-2026-1943 | CVE-2024-47874 | N/A en NVD; OSV v4 8.7 (DoS multipart) | [NVD](https://nvd.nist.gov/vuln/detail/CVE-2024-47874), [OSV](https://osv.dev/vulnerability/PYSEC-2026-1943) |
| PYSEC-2026-1941 | CVE-2025-54121 | 5.3 Media | [OSV](https://osv.dev/vulnerability/PYSEC-2026-1941) |
| PYSEC-2026-161 | CVE-2026-48710 | 6.5 Media | [NVD](https://nvd.nist.gov/vuln/detail/CVE-2026-48710) |
| PYSEC-2026-2281 | CVE-2026-48818 | 7.5 Alta (StaticFiles UNC en Windows; el contenedor Linux no aplica) | [OSV](https://osv.dev/vulnerability/PYSEC-2026-2281) |
| PYSEC-2026-2280 | CVE-2026-48817 | 5.3 Media | [OSV](https://osv.dev/vulnerability/PYSEC-2026-2280) |
| PYSEC-2026-249 | CVE-2026-54283 | 7.5 Alta | [OSV](https://osv.dev/vulnerability/PYSEC-2026-249) |
| PYSEC-2026-248 | CVE-2026-54282 | 5.3 Media | [OSV](https://osv.dev/vulnerability/PYSEC-2026-248) |

---

## 3. Priorización de remediación

| Gravedad | CVSS | SLA de remediación |
|---|---|---|
| Crítica | 9.0–10.0 | 72 h |
| Alta | 7.0–8.9 | 7 días |
| Media | 4.0–6.9 | 30 días |
| Baja | 0.1–3.9 | 90 días |

V03 es **Alta** (peor CVSS v3.1 = 7.5). SLA: 7 días. Quedó remediada el 07/10 (2 días después
de detectarla). V04 es **Alta** (7.4); no hay parche publicado y se acepta (sección 5). El SLA
de parcheo no corre mientras el tratamiento sea aceptación.

---

## 4. Ventana de parcheo

- Los parches de dependencias se prueban primero en el entorno de Docker local: `docker compose build`,
  `healthz`, verificación de firma válida e inválida, y la simulación de CU-01 a CU-03.
- Después de la entrega congelada (`v1.0`), cualquier cambio sigue el mismo procedimiento antes de
  volver a etiquetar.

---

## 5. Tratamiento de riesgos y falsos positivos

| ID | Hallazgo | Justificación | Aprobado por | Fecha |
|---|---|---|---|---|
| V04 | PYSEC-2026-1325 / CVE-2024-23342 (`ecdsa`, CVSS 7.4) | Ataque Minerva sobre P-256. No tiene corrección publicada (el proyecto declara los side-channels fuera de alcance). `ecdsa` llega a través de `python-jose`, pero nuestra firma de eventos usa **RS256** (RSA), no ECDSA, así que el aviso no afecta el camino de firma que usamos. Se acepta mientras no haya corrección. | Equipo Blue (pendiente de confirmación del RSI) | 05/10/2026 |
| V05 | CVE-2025-4123 (Grafana 11.2.0) | Grafana es solo del laboratorio (`localhost:3000`), sin anónimo de internet ni Image Renderer. Subir a 11.2.9+security-01 puede romper el provisionado. Se acepta hasta después de la defensa. | Equipo Blue | 11/10/2026 |
| V06 | CVE-2026-21859 (Mailpit `/proxy`) | Mailpit no sale a internet (justificado en C4). El SSRF queda en la red Docker. No se sube a 1.28.1 antes de la defensa. | Equipo Blue | 11/10/2026 |
| V07 | `/metrics` de Grafana | Filtra la versión. Mismo alcance local. | Equipo Blue | 11/10/2026 |

---

## 6. Resultado del análisis de código (SAST)

`bandit` sobre `control-central/app` no encontró problemas (ver `docs/evidencias/10-bandit.txt`).
`npm audit --omit=dev` sobre `cliente-gestor` no encontró vulnerabilidades (ver `docs/evidencias/10-npm-audit.txt`).

`nuclei` (11/10/2026, `docs/evidencias/10-nuclei.txt`): 4 matches (V05–V07). Varios `i/o timeout` contra
`host.docker.internal`; la API en `:8001` quedó casi sin barrer. Los puertos 80/4040/9780/43800
dieron *connection refused*: no están publicados. OpenVAS no se corrió; la letra admite nuclei.

---

## Guía de llenado (Blue Team)

| Fase | Cómo completar | Con qué evidencia |
|---|---|---|
| Implementación | Escaneo de dependencias y SAST, remediación de lo que tiene corrección | `10-pip-audit*.txt`, `10-bandit.txt`, `10-npm-audit.txt` |
| Nuclei | Escaneo HTTP de :8001/:3000/:8025 | `10-nuclei.txt` |
| Red Team | Ejecuta su propio escaneo y pentest; sus hallazgos entran como nuevos IDs V | Informe Red Team |

---

## Check de aceptación

- [x] Escaneo de dependencias de Python y de Node con evidencia.
- [x] SAST sobre el código del backend con evidencia.
- [x] Registro con ID, activo, herramienta y estado.
- [x] CVSS y SLA de V03 (7.5 Alto) y V04 (7.4 Alto). V01 y V02 siguen sin puntaje.
- [x] V03 cerrada con FastAPI 0.142.3 y starlette 1.7.0 (07/10/2026).
- [x] Escaneo web con nuclei (11/10). OpenVAS no se corrió.
- [ ] SAST incorporado al ciclo de desarrollo (bandit corrido a mano, no en CI).

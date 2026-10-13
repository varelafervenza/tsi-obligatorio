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
| V08 | `control-central` (`python-jose` 3.5.0) | CVE-2026-85394 / GHSA-3qf3-8w2g-rqmx: confusión de algoritmo RS256↔HS256. Afecta hasta la 3.5.0; sin fix de la librería | pip-audit | Sin CVSS publicado (OSV/NVD, al 12/10/2026) | **Mitigado por diseño, sin fix disponible.** `verificar_jws` fija `algorithms=["RS256"]` (un solo elemento); probado el 12/10 con un token HS256 forjado a mano usando la clave pública: rechazado, en vivo y en los tests. Ver `app/core/security.py` y `tests/test_firma_jws.py` | 12/10/2026 | — (mitigado) | Andrés Varela |
| V09 | `control-central` (`pyasn1` 0.4.8, transitiva de `python-jose`/`ecdsa`) | 4 avisos de denegación de servicio por ASN.1 recursivo/cuadrático (PYSEC-2026-2263, 3455, 3456, 3457) | pip-audit | No evaluado (DoS, baja explotabilidad: solo parsea claves que el propio servidor guarda, no datos de una petición) | **Remediado**: `python-jose` 3.4.0 fija `pyasn1<0.5.0` (conflicto real al reconstruir la imagen); se subió `python-jose` a 3.5.0 para poder fijar `pyasn1>=0.6.4`. `pip-audit` sobre el `requirements.txt` final: sin avisos de `pyasn1` | 12/10/2026 | 12/10/2026 | Andrés Varela |
| V10 | `control-central` (`app/api/incidents.py`, `app/api/alerts.py`) | Ninguna ruta pedía token, ni las que escriben: cualquiera podía crear incidentes falsos, cerrar un incidente real (apagando la alerta de un ataque en curso) o marcar cualquier alerta como falso positivo | Revisión manual de código | No evaluado (falla de diseño, no una dependencia) | **Remediado**: las 5 rutas piden el mismo token de panel que `users.py` (`app/core/auth.py`) | 12/10/2026 | 12/10/2026 | Andrés Varela |

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

**Desglose de V08** (confusión de algoritmo en `python-jose`). La falla real: si `jwt.decode` acepta
más de un algoritmo (por ejemplo `["RS256", "HS256"]`), alguien con la clave pública de un agente —que
es pública por diseño, se copia a `keys/agentes/` a propósito— puede construir un JWT con cabecera
`alg: HS256` y firmarlo con HMAC-SHA256 usando el texto de esa clave pública como "secreto", sin tocar
la clave privada. Prueba hecha el 12/10/2026 (`tests/test_firma_jws.py`,
`test_token_hs256_forjado_con_la_clave_publica_se_rechaza`): se armó ese token a mano —no con
`jwt.encode`, que python-jose bloquea del lado de quien firma— y se lo pasó a `verificar_jws`.
Rechazado con `"The specified alg value is not allowed"`, porque `ALGORITMO` en `security.py` es una
lista de un solo elemento. Si alguna vez se agrega un segundo algoritmo a esa lista, la falla se
reabre: por eso el comentario de invariante en el código.

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

**Revisión del 12/10/2026, antes del tag `v1.0`** (`docs/evidencias/10-revision-12-10.txt`):
`bandit` y `npm audit` se repitieron, sin cambios (0 problemas). `pip-audit` encontró V08 y V09
(no estaban en la corrida del 05/10). `gitleaks` sobre el historial completo, sin hallazgos nuevos.
Revisión manual: sin SQL concatenado (todo parametrizado con SQLAlchemy), sin
`dangerouslySetInnerHTML`/`innerHTML`/`eval` en el cliente web, sin `Command::new` en el Rust de
producción, sin CORS configurado (por defecto, seguro). El hallazgo real fue V10, por lectura de
código, no por una herramienta: ninguna ruta de `incidents.py` ni `alerts.py` pedía token. `cargo
audit` no se corrió (no está instalado; instalarlo compila un árbol grande con el linker inestable
del equipo, ya documentado en `CLAUDE.md`).

**Después de abrir Docker:** se reconstruyó la imagen con `python-jose` 3.5.0 y `pyasn1` 0.6.4.
Los 26 tests pasan (18 de antes + 8 nuevos: 1 de la invariante HS256, 7 de autenticación). En vivo,
sin ningún token: `POST /api/incidents/` y `PATCH /api/alerts/1` responden `401`. `pip-audit` sobre
el `requirements.txt` final: solo V08 (sin fix) y V04 (riesgo aceptado); `pyasn1` ya no aparece.

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

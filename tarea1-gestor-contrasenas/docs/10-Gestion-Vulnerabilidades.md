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
| Versión | 0.1 (borrador) |
| Responsable | Blue Team (Horacio Duarte, Pablo Morales, Andrés Varela) |
| Fecha | 05/10/2026 |

---

## 1. Herramientas utilizadas

| Herramienta | Qué analiza | Corrida | Evidencia | Estado |
|---|---|---|---|---|
| `pip-audit` | Dependencias de Python de `control-central` | 05/10/2026 (antes y después de actualizar) | `10-pip-audit.txt`, `10-pip-audit-despues.txt` | Corrido |
| `npm audit --omit=dev` | Dependencias de producción de `cliente-gestor` | 05/10/2026 | `10-npm-audit.txt` | Corrido |
| `bandit` (SAST) | Código Python de `control-central/app` | 05/10/2026 | `10-bandit.txt` | Corrido |
| `cargo audit` | Dependencias de Rust de `cliente-gestor` | — | — | **Pendiente** |
| OpenVAS / nmap / nuclei | Escaneo de red, puertos y web | — | — | **Pendiente** |
| Trivy / semgrep | Imágenes Docker y SAST adicional | — | — | **Pendiente** |

---

## 2. Registro de vulnerabilidades

El CVSS no lo informa `pip-audit`; queda pendiente de buscarlo en la base de NVD para cada ID.

| ID | Activo | Hallazgo | Herramienta | CVSS v3 | Estado | Fecha detección | Fecha remediación | Responsable |
|---|---|---|---|---|---|---|---|---|
| V01 | `control-central` (`python-jose` 3.3.0) | PYSEC-2024-232, PYSEC-2024-233, PYSEC-2025-185 | pip-audit | Pendiente | **Remediado**: se subió a 3.4.0 | 05/10/2026 | 05/10/2026 | Pablo Morales |
| V02 | `control-central` (`python-multipart` 0.0.9) | Siete avisos PYSEC-2026-1851 a 3040 | pip-audit | Pendiente | **Remediado**: se subió a 0.0.31 | 05/10/2026 | 05/10/2026 | Pablo Morales |
| V03 | `control-central` (`starlette` 0.38.6, vía FastAPI 0.115.0) | Siete avisos: PYSEC-2026-161, 248, 249, 1941, 1943, 2280, 2281. El arreglo más alto que pedía el audit del 05/10 era starlette 1.3.1 | pip-audit | Pendiente | **Remediado:** FastAPI 0.142.3 y starlette 1.7.0. 18 tests pasan. Un `pip-audit` nuevo no pudo consultar PyPI en esta máquina (el certificado no verifica) | 05/10/2026 | 07/10/2026 | Pablo Morales |
| V04 | `control-central` (`ecdsa` 0.19.2, dependencia de `python-jose`) | PYSEC-2026-1325 | pip-audit | Pendiente | **Riesgo aceptado** (ver sección 5) | 05/10/2026 | — | Pablo Morales |

---

## 3. Priorización de remediación

| Gravedad | CVSS | SLA de remediación |
|---|---|---|
| Crítica | 9.0–10.0 | 72 h |
| Alta | 7.0–8.9 | 7 días |
| Media | 4.0–6.9 | 30 días |
| Baja | 0.1–3.9 | 90 días |

V03 se cerró el 07/10 al subir FastAPI y starlette por encima de las versiones que pedía el audit. El CVSS de esos avisos sigue sin buscarse en NVD. V04 sigue aceptada.

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
| V04 | PYSEC-2026-1325 (`ecdsa`) | No tiene corrección publicada. `ecdsa` llega a través de `python-jose`, pero nuestra firma de eventos usa **RS256** (RSA), no ECDSA, así que el aviso no afecta el camino de firma que usamos. Se acepta mientras no haya corrección. | Equipo Blue (pendiente de confirmación del RSI) | 05/10/2026 |

---

## 6. Resultado del análisis de código (SAST)

`bandit` sobre `control-central/app` no encontró problemas (ver `docs/evidencias/10-bandit.txt`).
`npm audit --omit=dev` sobre `cliente-gestor` no encontró vulnerabilidades (ver `docs/evidencias/10-npm-audit.txt`).

---

## Guía de llenado (Blue Team)

| Fase | Cómo completar | Con qué evidencia |
|---|---|---|
| Implementación | Escaneo de dependencias y SAST, remediación de lo que tiene corrección | `10-pip-audit*.txt`, `10-bandit.txt`, `10-npm-audit.txt` |
| Pendiente | Escaneo de red y web (OpenVAS, nmap, nuclei) y de imágenes Docker (Trivy) | Por correr |
| Red Team | Ejecuta su propio escaneo y pentest; sus hallazgos entran como nuevos IDs V | Informe Red Team |

---

## Check de aceptación

- [x] Escaneo de dependencias de Python y de Node con evidencia.
- [x] SAST sobre el código del backend con evidencia.
- [x] Registro con ID, activo, herramienta y estado.
- [ ] CVSS y SLA asignados a cada hallazgo (pendiente: V03 y V04).
- [x] V03 cerrada con FastAPI 0.142.3 y starlette 1.7.0 (07/10/2026).
- [ ] Escaneo de red y web (pendiente).
- [ ] SAST incorporado al ciclo de desarrollo (bandit corrido a mano, no en CI).

# Pendientes para la entrega — Tarea 1

> Solo lo que pide `LETRA.md`. Se actualiza en el **mismo commit** que cierra cada punto.
> Estado al **13/10/2026**. Defensa: **14/10/2026**.
>
> Si no está en la letra, no entra acá. Mailpit, Wazuh sin indexer y Docker en un host
> ya se hablaron con el profesor: no hay consulta pendiente al foro.

## Abierto (letra)

Sin pendientes abiertos de la letra.

## Cerrado (letra)

| # | Qué | Dónde | Cierre |
|---|---|---|---|
| 1 | Tag `v1.0` y hash SHA-256 del árbol | 6.3 H4; 8.2 | 13/10. Tag sobre `a0907d6` (objeto `9272c1a`). Release con el instalador publicado y descargable sin login (verificado con `curl -I -L` anónimo). Hash: `5ffe5ac7cac1cde62cd5ed4085d17e59259986c1285a566f576823996b787a78` (`scripts/verificar_integridad_tag.sh v1.0`). `v0.0-test` se deja publicado aparte, sin tocar |
| 3 | Cobertura 100 % alta / mod / borrado desde la app | 6.4 | 07/10 |
| 14 | Excel de controles MCU con evidencia | 8.1; 6.6 | 10/10 |
| 15 | Excel de bitácora al día | 6.5; 8.1 | 10/10 |
| 19 | SIEM con reglas y alerta real (Wazuh 4.14.8) | RF-10; 6.2 #6 | 07/10 |
| 20 | Notificaciones por correo (Mailpit; Mailu no se despliega) | RF-07 | 07/10. Justificación en `00-arquitectura-c4.md` |
| 22 | Uptime, RNF-02, RNF-05 (carga básica) y rollback RNF-10 | 6.4; sección 4 | 11/10 |
| 23 | nuclei sobre :8001/:3000/:8025 (OpenVAS no) | 8.1 plantilla 10 | 11/10 |
| 29 | Tasa de falsos positivos del SIEM | 6.4 | 11/10: 0,0 % (1 alerta `cambio_maestra` clasificada como válida) |
| 24 | CVSS de V03 y V04 | plantilla 10 + 7.4 (severidad) | 11/10 |
| 28 | V03 remediada (FastAPI 0.142.3 / starlette 1.7.0) | plantilla 10 | 07/10 |
| 2 | Video de la demo, 5 minutos o menos | 6.2 entregable 13; 8.2 | 12/10. Grabado por Horacio Duarte, en Drive (`docs/20-Evidencias.md`, sección 08). Permiso del enlace confirmado el 14/10 |

Backup y restauración de bóveda/BD ya están (06/10). La letra pide copias exportadas y prueba de restauración (6.2 #8; 8.1), no una tarea programada de Windows ni copia fuera del equipo.

## Sacado de esta lista (no está en la letra)

- Consulta al foro o mail a la cátedra (#21). Las dudas del grupo ya se hablaron con el profesor.
- Instalar el hook en el clone de Horacio (#17).
- Probar la guía con otra persona u otra máquina (#18). La letra pide demo en la auditoría (6.6), no ese ensayo.
- Tratamientos extra del análisis de riesgos (medidor de maestra, IDOR, CSP, bind a `127.0.0.1`, etc.).
- Completar horas N/D de la bitácora (#16).
- Firma del RSI, custodia de `keys/agentes/` y teléfonos oficiales de BCU/URCDP: los pide la plantilla ISACA o el equipo, no la letra.
- Trivy y `cargo audit`: la matriz 8.1 nombra OpenVAS/nuclei/SAST.

## Decisiones resueltas (cola de eventos, 07/10)

Ver `docs/00-arquitectura-c4.md` y la fila R02 de `docs/03-Analisis-Riesgos.md`.

| Idea | Decisión | Detalle |
|---|---|---|
| Extender el plazo de la firma | **Implementado, a 4 horas** | `events::VENCIMIENTO_FIRMA_SEGUNDOS` |
| Reintentar al abrir y al cerrar | **Implementado** | `events::reintentar_cola` |
| Botón de reintento en el panel | **Implementado** | `vault::commands::reintentar_eventos` |
| Aviso al cerrar / formulario al cerrar | **Descartado** | — |

## Opcional (no letra)

- Tests de retención de 90 días e incidentes.
- Dockerfile de tests del cliente; job de CI en Ubuntu.
- Página propia de incidentes y usuarios.

## Historial de cierres

| Fecha | Commit | Qué se cerró |
|---|---|---|
| 06/10 | `54f2ffb`, `ec8e6f3` | Backup, restauración con datos, guía de pruebas |
| 06/10 | `6cc98f4` | Declaración de aplicabilidad (93 controles) |
| 06/10 | `dd5dab2` | Notificación simulada del incidente 1 (12) |
| 06/10 | `3768fc1` | Informe final del Blue Team (28) |
| 06/10 | `49a99be` | Cobertura al 100 % con `modificacion_credencial` (script) |
| 06/10 | `6ba28d2` | MTTD desde la primera alerta del evento de origen |
| 06/10 | `c52b7c0` | MTTR desde la alerta; severidad S2 del incidente 1 |
| 06/10 | `cc52884` | Pruebas automáticas de MTTD, MTTR y cobertura |
| 06/10 | `fa4b30b` | Análisis de riesgos versión 2 (03) |
| 06/10 | `aaecc19` | Hook de `gitleaks`, aviso MFA (R06) y pruebas de firma JWS |
| 07/10 | — | Cobertura alta/mod/borrado desde la app |
| 07/10 | — | R13: PostgreSQL ya no se publica en el host (`5432`) |
| 10/10 | — | Excel MCU #14 y #15 |
| 10/10 | — | Nombre canónico `04-Gestion-Incidentes.md` (#13) |
| 11/10 | — | CVSS de V03 (7.5) y V04 (7.4) (#24) |
| 11/10 | — | #22: uptime 28 min, apertura 2,63/0,81/0,40 s, carga básica, rollback en `06` |
| 11/10 | — | nuclei #23: V05 Grafana, V06 Mailpit, V07 /metrics |
| 11/10 | — | #29: tasa de falsos positivos 0,0 % (alerta 1, cambio_maestra) |
| 12/10 | — | Firma del RSI en 01, 03, 06, 11, 12 y 28, y decisión de custodia de claves (sin respaldo, `01` sección 5.1). No lo pedía la letra; se hizo igual por consistencia con la auditoría |
| 12/10 | — | #2: video de la demo, grabado por Horacio Duarte y subido a Drive |
| 13/10 | — | #1: tag `v1.0` sobre `a0907d6`, Release con instalador y hash SHA-256 del árbol |

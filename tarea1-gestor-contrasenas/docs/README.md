# docs — Carpeta de trabajo del equipo (Tarea 1)

Esta carpeta es el **espacio de trabajo** del equipo. Aquí se van copiando y completando las plantillas de `../plantilla/isaca/` conforme a la matriz de documentación de `../LETRA.md` (sección 8).

> `../plantilla/` se deja siempre en blanco (es el material de referencia del curso, compartido
> con las otras tareas); las copias completadas de la Tarea 1 viven acá, en `docs/`. Los 4 Excel
> de apoyo del MCU 5.0 están en `docs/mcu5/excel/` (misma estructura que `plantilla/mcu5/excel/`).

## Documentos a generar (se completa a medida que se avanza)

Estado al **10/10/2026**. La firma del RSI y el tag `v1.0` siguen abiertos (`docs/00-pendientes-entrega.md`).

| Archivo esperado | Origen (plantilla) | Estado |
|---|---|---|
| `00-arquitectura-4mas1.md` | `plantilla/plantilla-arquitectura-4más1.md` | ☑ borrador 0.1 |
| `00-arquitectura-c4.md` | `plantilla/plantilla-arquitectura-C4.md` | ☑ borrador (Wazuh en compose, Mailpit justificado, cola 4 h) |
| `diagrama-arquitectura.png` | diagrama usado en la LETRA | ☑ en esta carpeta |
| `99-bitacora-trabajo.md` | `plantilla/isaca/99-bitacora-trabajo.md` | ☑ en curso (espejada en el Excel, 79 entradas al 10/10) |
| `01-Politica-Seguridad.md` | `plantilla/isaca/01-politica-seguridad.md` | ☑ borrador (TOTP/RBAC al día; falta firma del RSI) |
| `02-Registro-Activos.md` | `plantilla/isaca/02-registro-activos.md` | ☑ borrador (+ Excel `docs/mcu5/excel/02-registro-activos-mcu5.xlsx`) |
| `03-Analisis-Riesgos.md` | `plantilla/isaca/03-analisis-riesgos.md` | ☑ borrador 2.0 (13 riesgos; falta firma del RSI) |
| `04-Gestion-Incidentes.md` | `plantilla/isaca/04-gestion-incidentes.md` | ☑ borrador (nombre canónico; #13 cerrado) |
| `06-Plan-Continuidad.md` | `plantilla/isaca/06-plan-continuidad.md` | ☑ borrador (backup/restore probados; Wazuh ya está en el compose) |
| `07-Monitoreo-Logs-SIEM.md` | `plantilla/isaca/07-monitoreo-logs.md` | ☑ borrador (reglas de API y Wazuh con alertas 100120/100101/100111) |
| `09-Gestion-Accesos.md` | `plantilla/isaca/09-gestion-accesos.md` | ☑ TOTP en cliente y panel; Hello/WebAuthn fuera de alcance |
| `10-Gestion-Vulnerabilidades.md` | `plantilla/isaca/10-gestion-vulnerabilidades.md` | ☑ V01–V03 remediadas; V04 aceptada; red/Trivy/`cargo audit` pendientes |
| `11-SoA-Plan-Tratamiento.md` | `plantilla/isaca/11-soa-plan-tratamiento.md` | ☑ 93 controles; falta firma del RSI |
| `12-Notificacion-Incidentes.md` | `plantilla/isaca/12-notificacion-incidentes.md` | ☑ incidente 1 simulado; falta firma RSI y contactos oficiales |
| `98-Glosario.md` | `plantilla/isaca/GLOSARIO.md` (complementario) | ☑ siglas técnicas de la tarea |
| _(Excel)_ `mcu5/excel/03-matriz-raci-mcu5.xlsx` | `plantilla/mcu5/excel/` | ☑ completada |
| _(Excel)_ `mcu5/excel/01-controles-mcu5-perfil-avanzado.xlsx` | `plantilla/mcu5/excel/` | ☑ 47/47 con Aplica; 25 filas de evidencia actualizadas el 10/10 (#14 cerrado) |
| _(Excel)_ `mcu5/excel/04-bitacora-planilla.xlsx` | `plantilla/mcu5/excel/` | ☑ 79 entradas espejadas desde `99-bitacora-trabajo.md` (#15 cerrado) |
| `28-Informe-Blue-Team.md` | nueva | ☑ borrador (cobertura 100 % desde la app; V03 cerrada; falta firma RSI) |
| `evidencias/` | capturas, logs | ☑ demo, Mailpit, Grafana, cobertura, Wazuh/cola, backups; falta el video (#2) |

> La lista de lo que falta para el 14/10 está en `00-pendientes-entrega.md`.

# Carpeta de Plantillas para Documentación de Seguridad

Esta carpeta centraliza las **plantillas oficiales** que los equipos deben utilizar obligatoriamente para documentar la seguridad de sus soluciones, junto con los **marcos normativos de referencia**.

## Contenido

| Carpeta | Contenido |
|---|---|
| [`isaca/`](./isaca/) | Plantillas de documentación de seguridad basadas en ISACA / COBIT 2019. Cada plantilla indica en su encabezado a qué función MCU 5.0, proceso COBIT, control ISO 27001 y requisito BCU aporta. Incluye la **plantilla de bitácora** (`99-bitacora-trabajo.md`). |
| [`bcu/`](./bcu/) | Marco de referencia del Banco Central del Uruguay: Guía de Seguridad de la Información, requerimientos mínimos y Comunicación 2026/098 (reporte trimestral de nivel de madurez MCU 5.0). |
| [`mcu5/`](./mcu5/) | Marco de Ciberseguridad del Uruguay 5.0 (AGESIC): estructura, funciones del ciclo de vida, categorías y perfiles comunitarios Básico / Estándar / Avanzado. |
| [`mcu5/excel/`](./mcu5/excel/) | **Excel de apoyo al perfil AVANZADO** (obligatorios): controles por función con evidencia, registro de activos, matriz RACI y bitácora (XLSX + README). |
| `plantilla-arquitectura-4más1.md` | Plantilla de arquitectura **vistas 4+1** (Kruchten). |
| `plantilla-arquitectura-C4.md` | Plantilla de arquitectura **C4** (Contexto/Contenedores/Componentes/Código). |
| `informe-red-team.md` | Plantilla del **informe de evaluación del Red Team** (obligatoria en la Parte 2). |

## Cómo usar estas plantillas

1. Cada grupo de trabajo **copia la plantilla a su carpeta de trabajo** (`tareas/tareaN/docs/`) y la completa con la información real de su solución.
2. **No se modifica** el encabezado de mapeo de la plantilla: es la evidencia de a qué requisito reglamentario aporta cada entregable.
3. Cada letra de tarea (`tareas/tareaN/`) indica *qué plantilla llenar, en qué fase, con qué evidencias y para qué requisito*.
4. Las plantillas están disponibles en **Markdown (.md)** para editar y en **DOCX (.docx)** para entregar.
5. Todo documento entregado debe llevar su **hoja de control de versiones** (revisión, fecha, responsable, cambios) que incluye cada plantilla.
6. **Perfil MCU 5.0 objetivo: AVANZADO** en las tres tareas: completar los Excel de `mcu5/excel/` (controles/activos/RACI/bitácora) y subirlos al repositorio.
7. La arquitectura se documenta con las plantillas **4+1** y **C4** (`plantilla-arquitectura-4más1.md`, `plantilla-arquitectura-C4.md`) y se presenta en vivo junto con la demo funcional.
8. El **informe de evaluación del Red Team** se entrega con la plantilla `informe-red-team.md` (pre-entrega 28/10, final 09/11/2026).
9. La **bitácora diaria** es obligatoria en ambas partes (`isaca/99-bitacora-trabajo.md`).

## Conjunto mínimo de entregables de seguridad por tarea

Independientemente de la tarea, todo grupo debe entregar como mínimo la siguiente documentación (según corresponda al alcance):

| # | Documento | Plantilla |
|---|---|---|
| 1 | Política de seguridad de la información (o política de uso) | `isaca/01-politica-seguridad.md` |
| 2 | Inventario y clasificación de activos | `isaca/02-registro-activos.md` |
| 3 | Análisis y tratamiento de riesgos | `isaca/03-analisis-riesgos.md` |
| 4 | Registro y respuesta a incidentes | `isaca/04-gestion-incidentes.md` |
| 5 | Monitoreo, logs y registro de eventos | `isaca/07-monitoreo-logs.md` |
| 6 | Gestión de identidades y accesos | `isaca/09-gestion-accesos.md` |
| 7 | Gestión de vulnerabilidades | `isaca/10-gestion-vulnerabilidades.md` |
| 8 | Declaración de aplicabilidad + plan de tratamiento | `isaca/11-soa-plan-tratamiento.md` |
| 9 | Bitácora de trabajo diaria | `isaca/99-bitacora-trabajo.md` |
| 10 | Arquitectura (4+1 y C4) | `plantilla-arquitectura-4más1.md` y `plantilla-arquitectura-C4.md` |
| 11 | Informe Red Team | `informe-red-team.md` |
| 12 | Excel MCU 5.0 (controles/activos/RACI/bitácora) | `mcu5/excel/*.xlsx` |

El detalle de *cuándo* y *con qué* llenar cada una está en la matriz de documentación de cada letra de tarea.
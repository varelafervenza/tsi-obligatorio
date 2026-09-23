# Inventario y Clasificación de Activos — Gestor de Contraseñas con Control Centralizado

> Basado en `plantilla/isaca/02-registro-activos.md`. Los mismos activos están cargados en
> `plantilla/mcu5/excel/02-registro-activos-mcu5.xlsx` (hoja `Activos`), que es el formato
> exigido para MCU 5.0 (ID-01). Este `.md` agrega el contexto (clasificación, matriz crítica,
> responsabilidades) que el Excel no cubre.

---

## Encabezado de mapeo normativo

| Marco | Ítem | Detalle / aporte |
|---|---|---|
| **MCU 5.0** | **Identificar** (ID-01, ID-02), **Proteger** (PR-01) | Catálogo de activos y priorización de críticos. |
| **COBIT 2019** | APO03, APO12 | Gestión de activos de TI y de riesgo. |
| **ISO/IEC 27001:2022** | A.5.9, A.5.10, A.5.12 | Inventario, propiedad, clasificación. |
| **BCU — GSI** | Inventario de activos críticos | Activos que soportan operaciones de negocio. |
| **URCDP — Ley 18.331, Art. 9** | Inventario de datos personales | Activos marcados con `Dato personal = S`. |

---

## Control del documento

| Campo | Valor |
|---|---|
| Código | SI-ACT-02 |
| Versión | 0.1 (borrador — hito H1, 21/09/2026) |
| Responsable | Blue Team (Horacio Duarte, Pablo Morales, Andrés Varela) |
| Fecha | 22/09/2026 |

### Historial de versiones

| Versión | Fecha | Autor | Cambios |
|---|---|---|---|
| 0.1 | 22/09/2026 | Blue Team | Primera versión, 14 activos alineados a `docs/00-arquitectura-4mas1.md` y `docs/00-arquitectura-c4.md`. |

## 1. Objetivo

Mantener un inventario completo, actualizado y clasificado de los activos de información que
soportan el gestor de contraseñas con control centralizado, para priorizar su protección, medir
el riesgo (`03-analisis-riesgos`) y demostrar cumplimiento en la auditoría del 14/10/2026.

## 2. Metodología

- Identificación: se relevó desde `docs/00-arquitectura-c4.md` (niveles 1-2) y el esqueleto real
  del repositorio (`cliente-gestor/`, `control-central/`, `infra/`), no desde cero.
- Clasificación según confidencialidad e impacto (tabla siguiente).
- Dueños designados por activo (usuario final, RSI o Blue Team según corresponda).
- Revisión: semanal mientras dure la implementación (hasta el 07/10/2026); luego mensual.

## 3. Clasificación

| Nivel | Confidencialidad | Ejemplo en esta solución | Impacto si se compromete |
|---|---|---|---|
| Público | Sin restricción | Código fuente del esqueleto (sin secretos) | Nulo |
| Interno | Uso interno | Dashboard Grafana, agentes Wazuh, servidor de correo | Menor |
| Confidencial | Acceso restringido | Servidor control-central, PostgreSQL, servidor Wazuh, segmentación de red | Alto |
| Secreto | Altamente restringido | Bóveda del usuario, llave privada de firma de eventos, base de usuarios/RBAC del panel | Muy alto |

## 4. Inventario de activos

> Fuente de verdad operativa: `plantilla/mcu5/excel/02-registro-activos-mcu5.xlsx` (hoja
> `Activos`). Tabla espejo abajo para lectura rápida sin abrir el Excel.

| ID | Nombre del activo | Tipo | Ubicación | Dueño | Clasificación | Crítico | Dato personal | Función MCU |
|---|---|---|---|---|---|---|---|---|
| A01 | Cliente-gestor (app de escritorio) | HW/SW | Estación del usuario | Usuario final | Confidencial | Alta | N | Proteger |
| A02 | Bóveda cifrada del usuario | Dato | Estación del usuario | Usuario final | Secreto | Muy alta | S | Proteger |
| A03 | Clave privada de firma de eventos | Dato | Keystore local (estación usuario) | RSI | Secreto | Alta | N | Proteger/Detectar |
| A04 | Servidor control-central | HW/SW | VLAN Servidores | RSI | Confidencial | Alta | N | Proteger/Detectar |
| A05 | PostgreSQL (control-central) | Dato | Servidor control-central | RSI | Confidencial | Alta | S | Detectar |
| A06 | Claves públicas de agentes | Dato | Servidor control-central | RSI | Interno | Media | N | Proteger |
| A07 | Base de usuarios del panel (RBAC) | Dato | PostgreSQL control-central | RSI | Secreto | Alta | S | Proteger |
| A08 | Servidor Wazuh (manager+indexer+dashboard) | HW/SW | VLAN Seguridad | RSI | Confidencial | Alta | N | Detectar |
| A09 | Agentes Wazuh (en clientes) | SW | Estaciones de usuario | RSI | Interno | Media | N | Detectar |
| A10 | Servidor de correo Mailu | HW/SW | VLAN Servicios | RSI | Interno | Media | S | Responder |
| A11 | Dashboard Grafana | HW/SW | Servidor control-central | RSI | Interno | Media | N | Detectar/Gobernar |
| A12 | Gestión de incidentes (TheHive o tabla interna) | HW/SW | Servidor control-central | RSI | Interno | Media | N | Responder |
| A13 | Repositorio Git (código fuente) | SW/Dato | GitHub `varelafervenza/tsi-obligatorio` | Blue Team | Interno | Alta | N | Gobernar |
| A14 | Segmentación de red del laboratorio | HW | Perímetro del laboratorio | RSI | Confidencial | Alta | N | Proteger |

> Pendiente: reemplazar VLAN/IPs genéricas por las reales una vez levantadas las VMs del
> laboratorio (Anexo A de `LETRA.md`) y reflejarlas también en `docs/00-arquitectura-4mas1.md`
> (vista física).

## 5. Matriz crítica (para riesgo e impacto)

| Proceso de negocio | Activos que lo soportan | Categoría más alta | RTO deseado |
|---|---|---|---|
| Acceso a sistemas (uso diario del gestor) | A01, A02, A03 | Secreto | < 2 s reapertura local; sin RTO de red (RNF-01) |
| Auditoría y trazabilidad de eventos | A04, A05, A06, A13 | Confidencial | 4 h (RNF-03, disponibilidad control central) |
| Gestión de accesos del panel | A04, A07 | Secreto | 4 h |
| Monitoreo y detección | A08, A09, A11 | Confidencial | 4 h |
| Respuesta a incidentes y notificación | A10, A12 | Interno | Notificación inmediata ante cambio de maestra (RF-07) |
| Continuidad de red del laboratorio | A14 | Confidencial | Según plan de continuidad (`06-plan-continuidad`) |

## 6. Responsabilidades

| Rol | Responsabilidad |
|---|---|
| Usuario final | Dueño de A01/A02: cuidar su estación, no compartir la contraseña maestra. |
| RSI | Aprobar clasificación y matriz de criticidad; dueño de A03-A12 y A14. |
| Blue Team | Mantener el inventario actualizado a medida que se despliega (`docker ps`, IPs reales); dueño de A13. |
| Administrador de infraestructura | Copias de seguridad de A04, A05, A08, A10 y prueba de restauración (`06-plan-continuidad`). |

---

## Guía de llenado (Blue Team)

| Fase | Cómo completar | Con qué evidencia |
|---|---|---|
| Diseño (actual) | Inventario preliminar desde `docs/00-arquitectura-c4.md` y el esqueleto del repo | Este documento + Excel `02-registro-activos-mcu5.xlsx` |
| Implementación (H2-H3) | Enriquecer con lo realmente desplegado: IPs, hostnames, versiones exactas de cada imagen Docker | `docker ps`, `docker compose config`, capturas en `docs/evidencias/` |
| Pre-entrega (H4, 07/10) | Congelar versión final junto con el resto de la documentación | Commit + `git tag v1.0` |
| Red Team | No se modifica; se usa como referencia de activos expuestos | Informe Red Team |

## Check de aceptación

- [x] Todos los componentes de la solución (según la arquitectura 4+1/C4) tienen un ID de activo.
- [x] Cada activo tiene dueño y clasificación.
- [x] La matriz crítica identifica los procesos de negocio.
- [x] Cubre datos personales (URCDP: A02, A05, A07, A10) y activos críticos (BCU: A01-A05, A08, A14).
- [ ] IPs/VLANs reales cargadas (pendiente hasta levantar el laboratorio).

# Plantilla de Modelo de Arquitectura — C4 (Context, Containers, Components, Code)

> **Modelo C4** (Simon Brown): documenta la arquitectura en **cuatro niveles de zoom** (Contexto, Contenedores, Componentes, Código). Es ideal para comunicar a audiencias técnicas y no técnicas, y es el modelo recomendado junto con 4+1.
> Debe usarse para describir la solución **antes** de la demo funcional en la auditoría.

---

## Encabezado de mapeo normativo

| Marco | Ítem de referencia |
|---|---|
| **MCU 5.0** | Evidencia para **Identificar** (ID-01), **Gobernar** (GV-01) y **Proteger** (PR-01). |
| **BCU (GSI)** | Inventario y arquitectura de acceso/red. |
| **ISO/IEC 27001:2022** | A.5.9 (Inventario), A.5.29 (continuidad), A.8.16 (monitoreo). |
| **COBIT 2019** | APO03, BAI02, BAI06. |
| **Plantillas asociadas** | `02-registro-activos`, `07-monitoreo-logs`, `06-plan-continuidad`. |

---

## Control del documento

| Campo | Valor |
|---|---|
| Nombre | Modelo de arquitectura C4 — `[Solución]` |
| Código | ARQ-C4-`[NN]` |
| Versión | 1.0 |
| Autor/equipo | `[Equipo Blue Team]` |
| Fecha | `[DD/MM/AAAA]` |
| Aprobación | `[Docente / RSI]` |

---

## Nivel 1 — Contexto (Contexto)

**Qué contiene:**
- El sistema dibujado como **una sola caja** en el centro.
- Los **actores externos** que interactúan: usuarios, RSI, SIEM, correo, Wazo, internet.
- Las **relaciones principales**: quién usa dónde y qué.
- Límites de confianza y flujos de datos (sin detalles internos).

| Actor/Sistema externo | Relación con el sistema | Datos intercambiados |
|---|---|---|
| `[Usuario final]` | `[usa]` | `[credenciales cifradas]` |
| `[SIEM]` | `[recibe]` | `[eventos de auditoría]` |
| `[Servidor mail]` | `[recibe notificaciones]` | `[alertas]` |

## Nivel 2 — Contenedores (Contenedores)

**Qué contiene:**
- Los **contenedores** (aplicación web, servidor de aplicación, BD, dashboard, agente, SIEM).
- Tecnologías y protocolos entre contenedores.
- Despliegue (VM/contenedor) por cada caja.

| Contenedor | Tecnología | Responsabilidad | Despliegue | Protocolo |
|---|---|---|---|---|
| `[Frontend]` | `[React]` | `[UI]` | `[Docker]` | HTTPS |
| `[API]` | `[Django]` | `[lógica]` | `[VM]` | REST |
| `[BD]` | `[PostgreSQL]` | `[persistencia]` | `[VM]` | SQL/TLS |

## Nivel 3 — Componentes (Componentes)

**Qué contiene:**
- La **descomposición interna de cada contenedor** en componentes (módulos, servicios, librerías).
- Responsabilidad y dependencias de cada componente.
- Data-stores y colas internos.

| Contenedor | Componente | Responsabilidad | Depende de |
|---|---|---|---|
| `[API]` | `[AuthMgr]` | `[autenticación MFA]` | `[Keycloak, Redis]` |
| `[API]` | `[EventEmitter]` | `[genera eventos firmados]` | `[AuthMgr]` |

## Nivel 4 — Código (Código)

**Qué contiene:**
- Diagramas de clases/estructuras clave de los componentes más críticos (opcional según relevancia).
- En las tareas del curso se espera únicamente para los módulos de seguridad (autenticación, cifrado, generación de eventos).

| Componente | Patrones | Archivos/Clases relevantes | Dónde se documenta |
|---|---|---|---|
| `[AuthMgr]` | `[Strategy/Factory]` | `[auth/*.py]` | `docs/09-gestion-accesos` |

---

## Uso por tarea

| Tarea | C4 a entregar | Vista que más se enfatiza |
|---|---|---|
| T1 Gestor de contraseñas | Niveles 1 y 2 + N3 para autenticación/cifrado | Procesos (flujo de eventos/notificaciones) |
| T2 RSI | Niveles 1 y 2 + N3 para exportadores y RBAC | Procesos (generación de documentos) |
| T3 Red de defensa | Niveles 1 y 2 (cada sensor/contenedor) + vista física de red | Física/despliegue + escenarios de detección |

---

## Guía de auditoría (orden de presentación)

| Paso | Qué presentar | En qué orden |
|---|---|---|
| 1 | **Nivel 1 Contexto** (una caja del sistema + actores) | Primero |
| 2 | **Nivel 2 Contenedores** con tecnologías | Segundo |
| 3 | **Nivel 3 Componentes** de los módulos críticos | Tercero |
| 4 | Demo en vivo del sistema funcionando | Cuarto |
| 5 | Vínculo de cada caja a controles MCU/BCU/ISO | Cierre |

## Check de aceptación

- [ ] 4 niveles completos (código solo para módulos de seguridad).
- [ ] Tecnologías reales (las desplegadas).
- [ ] Relaciones con los actores y con SIEM/correo/Wazo correctas.
- [ ] Acompañado de capturas en `docs/evidencias`.
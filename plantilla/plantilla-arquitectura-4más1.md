# Plantilla de Modelo de Arquitectura — Vistas 4+1 (Kruchten)

> **Modelo 4+1** (Philippe Kruchten): documenta la arquitectura de software/sistema mediante **cuatro vistas** más un **conjunto de escenarios** (el "+1") que las valida y las cohesiona.
> Para el curso, esta plantilla es **obligatoria** en la letra de todas las tareas para describir la arquitectura de la solución, y debe presentarse **antes** de la demo funcional en la auditoría.

---

## Encabezado de mapeo normativo

| Marco | Ítem de referencia |
|---|---|
| **MCU 5.0** | Aporta evidencia a **Identificar** (ID-01 inventario de activos), **Gobernar** (GV-01) y **Proteger** (PR-01). |
| **BCU (GSI)** | Sustenta requerimientos de inventario, arquitecturas de acceso y continuidad. |
| **ISO/IEC 27001:2022** | A.5.9 Inventario, A.5.29 Continuidad, A.8.16 Monitoreo. |
| **COBIT 2019** | APO03 (Activos), BAI02 (Definir arquitectura), BAI06 (Cambios). |
| **Plantillas asociadas** | `02-registro-activos`, `07-monitoreo-logs`, `06-plan-continuidad`. |

---

## Control del documento

| Campo | Valor |
|---|---|
| Nombre | Modelo de arquitectura 4+1 — `[Solución]` |
| Código | ARQ-4+1-`[NN]` |
| Versión | 1.0 |
| Autor/equipo | `[Equipo Blue Team]` |
| Fecha | `[DD/MM/AAAA]` |
| Aprobación | `[Docente / RSI]` |

---

## 1. Vista Lógica (Lógica)

Representa la descomposición **funcional del sistema en módulos, servicios y sus responsabilidades** (independiente de dónde se ejecute).

**Qué contiene:**
- Diagrama de módulos/componentes funcionales (p. ej.: gestor cliente, control central, SIEM, dashboards, exportador).
- Responsabilidad principal de cada módulo.
- Interfaces principales entre módulos (API, colas, eventos).
- Patrones (MVC, microservicios, event-driven).

| Módulo | Responsabilidad | Interfaces que expone/consume | Dependencias |
|---|---|---|---|
| `[Módulo]` | `[Descripción]` | `[REST, gRPC, cola...]` | `[...]` |

## 2. Vista de Procesos (Procesos)

Modela la **concurrencia, secuencias clave y flujos de negocio** en ejecución (cómo se comporta en runtime).

**Qué contiene:**
- Diagramas de secuencia de los casos de uso críticos (`[RV-01…]`): p. ej., autenticación MFA, creación de contraseña/evento→notificación, respuesta a incidente.
- Hilos/procesos/asincronía relevantes (workers, colas).
- Tiempos de respuesta de referencia.

| Secuencia | Participantes | Flujo (pasos) | Tiempo esperado |
|---|---|---|---|
| `[Caso de uso]` | `[Módulos]` | `[1→2→3]` | `[ms/s]` |

## 3. Vista de Desarrollo (Desarrollo)

Describe la **organización estática del código y la infraestructura del proyecto** (componentes, librerías, capas, repositorios).

**Qué contiene:**
- Diagrama de paquetes/capas (origen de dependencias, sin ciclos).
- Tecnologías y versiones de librerías.
- Repositorio y ramas; integración con CI.
- Métricas estáticas (cobertura de dependencias, líneas).

| Capa/Paquete | Tecnología | Responsabilidad | Dependencias |
|---|---|---|---|
| `[Capa]` | `[Lenguaje/stack]` | `[...]` | `[...]` |

## 4. Vista Física / Despliegue (Física)

Ubica la **distribución de los componentes en el hardware/VM/red**: dónde vive cada cosa y sus parámetros de despliegue.

**Qué contiene:**
- Diagrama de nodos (VMs/equipos/contenedores), cargas y puertos.
- Conexiones de red entre nodos (protocolo, puerto, cifrado).
- IPs/VLANs, firewall, almacenamiento.
- Copias de seguridad y puntos de monitoreo (SPAN/TAP/agentes).

| Nodo | Rol | Componentes que ejecuta | IP/VLAN | Recursos | Puertos |
|---|---|---|---|---|---|
| `[VM1]` | `[...]` | `[...]` | `[...]` | `[...]` | `[...]` |

## 5. Escenarios (el "+1")

**Casos de uso significativos que validan la arquitectura**: recomendable incluir 4-6 escenarios que atraviesen todas las vistas (un débito por cada requisito funcional crítico o exigencia no funcional dura).

| ID escenario | Descripción | Vistas que toca | Observación/validación |
|---|---|---|---|
| ESC-01 | `[Caso de uso]` | Lógica, Procesos, Física | `[Cómo se valida]` |
| ESC-02 | `[Caso de uso]` | `[...]` | `[...]` |

---

## Guía para completar en auditoría

| Paso | Qué presentar | En qué orden |
|---|---|---|
| 1 | Diagrama de arquitectura general (físico) | Primero |
| 2 | Vista lógica (módulos) | Segundo |
| 3 | Vista de procesos (secuencias críticas) | Tercero |
| 4 | Demostración en vivo del sistema | Cuarto |
| 5 | Indicar cómo cada vista sustenta controles MCU/BCU/ISO | Cierre |

## Check de aceptación

- [ ] Las 4 vistas + escenarios están completas.
- [ ] El diagrama físico coincide con lo realmente desplegado.
- [ ] Cada control de auditoría puede anclarse a una vista.
- [ ] Acompañado en repositorio con capturas reales.
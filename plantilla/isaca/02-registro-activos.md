# Plantilla ISACA/COBIT 2019 — Inventario y Clasificación de Activos de Información

---

## Encabezado de mapeo normativo

| Marco | Ítem | Detalle / aporte |
|---|---|---|
| **MCU 5.0 (función)** | **Identificar** | Aporta a la función **Identificar** (ID-01 Catálogo de activos; ID-02 Perfil de riesgo). |
| **MCU 5.0 (categorías)** | ID-01, ID-02, PR-01 | Inventario físico/lógico; priorización de activos críticos. |
| **COBIT 2019** | APO03 (Gestión de activos de TI), APO12 (Riesgo) | Insumo para riesgo y cambios. |
| **ISO/IEC 27001:2022** | A.5.9 Inventario de activos, A.5.10 Propiedad, A.5.12 Clasificación | Controles A.5.9, A.5.10, A.5.12, A.8.1. |
| **ISO/IEC 27002** | 5.9, 5.10, 5.12 | Clasificación y etiquetado. |
| **BCU — Guía de Seguridad de la Información** | Requisito sobre inventario/activos | Identificar activos críticos que soportan operaciones del negocio. |
| **URCDP — Ley 18.331** | Art. 9 | Inventario de bases de datos que contienen datos personales. |
| **NIST CSF (referencial)** | ID.AM | Identification & asset management. |

---

## Control del documento

| Campo | Valor |
|---|---|
| Código | SI-ACT-02 |
| Versión | 1.0 |
| Responsable | `[Nombre]` |
| Fecha | `[DD/MM/AAAA]` |

### Historial de versiones

| Versión | Fecha | Autor | Cambios |
|---|---|---|---|
| 1.0 | | | |

---

## 1. Objetivo

Mantener un inventario completo, actualizado y clasificado de los activos de información que soportan la solución `[Solución]`, para poder priorizar su protección, medir el riesgo y demostrar cumplimiento.

## 2. Metodología

- Identificación: componentes físicos, lógicos, de datos y de servicios de la solución completa (por ej. BD, servidores, SIEM, contenedores, credenciales, correo electrónico).
- Clasificación según confidencialidad e impacto.
- Dueños designados para cada activo.
- Revisión periódica `[mensual / trimestral]`.

## 3. Clasificación

| Nivel | Confidencialidad | Ejemplo | Impacto si se compromete |
|---|---|---|---|
| Público | Sin restricción | `[ej. landing]` | Nulo |
| Interno | Uso interno | `[ej. manuales]` | Menor |
| Confidencial | Acceso restringido | `[ej. datos usuarios]` | Alto |
| Secreto | Altamente restringido | `[ej. llaves maestras, hash]` | Muy alto |

## 4. Inventario de activos

| ID | Nombre del activo | Tipo (HW/SW/Dato/Servicio) | Ubicación | Dueño | Clasificación | Crítico (S/N) | Software/versión |
|---|---|---|---|---|---|---|---|
| A01 | | | | | | | |
| A02 | | | | | | | |
| A03 | | | | | | | |

> El inventario DEBE cubrir toda la solución de la letra: infraestructura, plataforma central, agentes/sensores, SIEM, base de datos, servidor de mail, servicios de autenticación, etc.

## 5. Matriz crítica (para riesgo e impacto)

| Proceso de negocio | Activos que lo soportan | Categoría más alta | RTO deseado |
|---|---|---|---|
| `[Proceso]` | `[A01, A02...]` | `[Alta]` | `[horas]` |

## 6. Responsabilidades

| Rol | Responsabilidad |
|---|---|
| Dueño del activo | Clasificar, aprobar accesos, validar periódicamente |
| Administrador | Mantener inventario y copias de seguridad |
| RSI | Aprobar la clasificación y la matriz de criticidad |

---

## Guía de llenado (Blue Team)

| Fase | Cómo completar | Con qué evidencia |
|---|---|---|
| Blue Team - Diseño | Crear inventario preliminar desde el diagrama de arquitectura | Diagrama, RF/RNF |
| Blue Team - Implementación | Enriquecer con lo realmente desplegado (versiones, IPs, hostnames) | `docker ps`, `ip a`, configs, GRUB/KVM, etc. |
| Red Team | NO se modifica; se usa como referencia para evaluar activos expuestos | Informe Red Team |

## Check de aceptación

- [ ] Todos los componentes de la solución tienen un ID de activo.
- [ ] Cada activo tiene dueño y clasificación.
- [ ] La matriz crítica identifica los procesos de negocio.
- [ ] Cubre datos personales (URCDP) y activos críticos (BCU).
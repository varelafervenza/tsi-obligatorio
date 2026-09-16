# Plantilla ISACA/COBIT 2019 — Política de Seguridad de la Información

---

## Encabezado de mapeo normativo

> **Complete esta tabla en función de su organización/solución. Los ítems marcados son las referencias mínimas.**

| Marco | Ítem | Detalle / aporte |
|---|---|---|
| **MCU 5.0 (función)** | Gobernar | Aporta a la función **Gobernar** (GV-01, GV-02) del Marco de Ciberseguridad 5.0. |
| **MCU 5.0 (categorías)** | GV-01 a GV-06, PR-01 | Establece el marco de gobierno y las reglas de protección de la información. |
| **Perfil comunitario** | Básico / Estándar / Avanzado | El perfil al que la organización declara aspirar. |
| **COBIT 2019** | APO13 (Gestión de Seguridad), APO01 (Gestión de marco de gobierno), EDM03 | La política es el insumo controlador del objetivo de gestión APO13. |
| **ISO/IEC 27001:2022** | A.5.1 Políticas de seguridad de la información, A.5.24 | Cumple los controles A.5.1, A.5.9, A.5.24. |
| **ISO/IEC 27002** | 5.1, 5.9, 6.2 | Guía de implementación de políticas. |
| **BCU — Guía de Seguridad de la Información** | Requisito 1 (política de seguridad) | La GSI BCU exige política formal aprobada por la dirección. |
| **URCDP — Ley 18.331** | Art. 9 y 12 (seguridad de datos) | La política asegura medidas de seguridad de los datos personales. |
| **Documento de cumplimiento** | `docs/01-Politica-Seguridad.md` | Versión firmada/aprobada a entregar en el hito correspondiente. |

---

## Control del documento

| Campo | Valor |
|---|---|
| Nombre del documento | Política de Seguridad de la Información |
| Código | SI-POL-01 |
| Versión | 1.0 |
| Fecha de aprobación | `[DD/MM/AAAA]` |
| Aprobado por | `[Rol - Comité/Gerencia]` |
| Autor | `[Nombre de quien redacta]` |
| Próxima revisión | `[DD/MM/AAAA]` |

### Historial de versiones

| Versión | Fecha | Autor | Descripción de cambios |
|---|---|---|---|
| 1.0 | | | |
| | | | |

---

## 1. Objetivo

Establecer el marco directivo que fija los principios, responsabilidades y reglas para proteger la confidencialidad, integridad y disponibilidad de la información de `[Organización]` y de la solución `[Solución]`.

## 2. Alcance

`[Describir a qué sistemas, activos, personas y procesos aplica la política. Para el curso: describir a qué componente de la tarea aplica.]`

## 3. Marco normativo y su posición

Describa cómo esta política se relaciona con:
- **MCU 5.0** (función Gobernar).
- **Plataforma/marco del BCU** (Gobernanza de ciberseguridad).
- Normativa interna y regulaciones aplicables (ISO 27001, Ley 18.331, CUI-Uruguay si aplica).

## 4. Principios de seguridad

| Principio | Descripción |
|---|---|
| Confidencialidad | `[Describir]` |
| Integridad | `[Describir]` |
| Disponibilidad | `[Describir]` |
| Mínimo privilegio | `[Describir]` |
| Defence in depth / Zero Trust | `[Describir]` |

## 5. Estructura de la política

Definir jerarquía documental: Política ⮕ Políticas específicas ⮕ Normas/procedimientos ⮕ Guías.

| Política específica | Contenido mínimo | Referencia |
|---|---|---|
| Política de control de accesos | Reglas de identidad, TOTP, SSO (Keycloak/Wirenboard), revisión de accesos | `09-gestion-accesos` |
| Política de gestión de contraseñas | Reglas de generación, crypto, rotación, brechas | `10-gestion-vulnerabilidades` |
| Política de gestión de incidentes | Detección, reporte, escalamiento | `04-gestion-incidentes` |
| Política de monitoreo y logs | Eventos, SIEM, retención | `07-monitoreo-logs` |
| Política de copias de seguridad | Retención, prueba de restauración | `06-plan-continuidad` |

## 6. Roles y responsabilidades

| Rol | Responsabilidades |
|---|---|
| RSI / CISO | `[Describir]` |
| Administrador de la plataforma | `[Describir]` |
| Usuarios | `[Describir]` |
| Equipo de desarrollo (Blue Team) | `[Describir]` |
| Equipo de validación (Red Team) | `[Describir]` |

## 7. Concientización y cumplimiento

- Frecuencia de capacitación: `[anual / trimestral]`.
- Consecuencias por incumplimiento: `[sanciones definidas]`.
- Canal de reporte: `[correo / ticketera]`.

## 8. Vigencia y revisión

- Esta política entra en vigencia a partir de `[fecha]`.
- Será revisada como mínimo una vez al año o ante cambios significativos.

---

## Guía de llenado (para el curso)

| Fase | Cómo completar | Con qué evidencia |
|---|---|---|
| Blue Team - Inicio | Redactar sobre la base del diseño de la solución y los RF/RNF | Diagrama de arquitectura, RF de la letra |
| Blue Team - Implementación | Ajustar a lo realmente implementado | Capturas de pantalla, config de servicios |
| Entrega a Red Team | Congelar la versión firmada | Git tag / hash del documento |
| Red Team - Evaluación | El Red Team NO modifica esta política; la evalúa contra hallazgos técnicos | Informe de hallazgos del Red Team |

## Check de aceptación

- [ ] Aprobada formalmente con fecha y firmas.
- [ ] Menciona MCU 5.0, BCU, ISO y Ley 18.331.
- [ ] Define roles y responsabilidades.
- [ ] Referencia el resto de las políticas de la carpeta `docs/`.
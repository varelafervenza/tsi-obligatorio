# Plantilla ISACA/COBIT 2019 — Plan de Continuidad del Negocio / Recuperación (BCP / DRP)

---

## Encabezado de mapeo normativo

| Marco | Ítem | Detalle / aporte |
|---|---|---|
| **MCU 5.0 (función)** | **Recuperar** (y respaldo en Proteger) | Recuperación de las operaciones (RC-01, RC-02, RC-03). |
| **MCU 5.0 (categorías)** | RC-01 Plan de recuperación, RC-02 Comunicación, RC-03 Pruebas | Copias de seguridad y restauración. |
| **COBIT 2019** | DSS04 (Continuidad), APO13 | Continuidad del servicio y disponibilidad. |
| **ISO/IEC 27001:2022** | A.5.29, A.5.30, A.8.13, A.8.14 | Controles de continuidad y backup. |
| **ISO 22301** | Todas | Sistema de gestión de continuidad de negocio. |
| **BCU — Guía de Seguridad de la Información** | Requisito de continuidad y backups | Backups diarios, copia fuera del centro de procesamiento, restauración probada periódicamente. |
| **URCDP — Ley 18.331, Art. 9 y Dec. 414/009** | Medidas de seguridad de datos personales | Backups y continuidad aplicados a bases con datos personales; registro de restauraciones. |
| **NIST SP 800-34** | BCP/DRP | Referencia técnica de plan de contingencia. |

---

## Control del documento

| Campo | Valor |
|---|---|
| Código | SI-BCP-06 |
| Versión | 1.0 |
| Responsable | `[Nombre]` |
| Fecha | `[DD/MM/AAAA]` |

---

## 1. Objetivos de recuperación

| Métrica | Valor objetivo |
|---|---|
| RTO (Recovery Time Objective) — BD | `[ej. 4 h]` |
| RTO — Servicios de front | `[ej. 2 h]` |
| RPO (Recovery Point Objective) — BD | `[ej. 15 min]` |
| RPO — Logs | `[ej. 5 min]` |

## 2. Inventario de respaldo

| Activo | Datos a respaldar | Herramienta | Frecuencia | Ubicación (dentro/fuera del sitio) | Retención |
|---|---|---|---|---|---|
| `[BD]` | `[dump binario/lógico]` | `[pg_dump / mariadb-dump]` | diario | `[off-site]` | `[30 d]` |
| `[SIEM logs]` | `[índices]` | `[logstash/elasticsearch snapshot]` | diario | `[off-site]` | `[90 d]` |
| `[Configs]` | `[archivos de config]` | `[git + encripción]` | bajo demanda | `[git]` | `[indefinido]` |

## 3. Procedimiento de prueba de restauración

| Paso | Descripción | Evidencia |
|---|---|---|
| 1 | Restaurar BD en entorno staging | Log de restauración exitosa |
| 2 | Restaurar últimos logs del SIEM | Consulta de integridad |
| 3 | Validar consistencia de datos | `SELECT count(*)` / `elasticsearch health` |
| 4 | Documentar resultado | Firma del responsable |

> El BCU exige: **backup diario**, **una copia fuera del centro de procesamiento** y **prueba de restauración periódica**. Esto debe quedar evidenciado.

## 4. Escenarios y procedimientos de recuperación

| Escenario | Procedimiento | RTO | Responsable |
|---|---|---|---|
| Pérdida del servidor principal | `[pasos]` | `[tiempo]` | `[rol]` |
| Corrupción de la BD | `[pasos]` | `[tiempo]` | `[rol]` |
| Ransomware en un nodo | `[aislar + restaurar]` | `[tiempo]` | `[rol]` |
| Caída del SIEM/SOAR | `[pasos]` | `[tiempo]` | `[rol]` |

## 5. Comunicación de crisis

| Canal de comunicación | Herramienta | Contacto |
|---|---|---|
| Email de alerta | `[servidor mail]` | `[grupo]` |
| Teléfono / presencial | `[celular]` | `[RSI]` |
| Dashboard de estado | `[grafana/alertmanager]` | `[url]` |

---

## Guía de llenado (Blue Team)

| Fase | Cómo completar | Con qué evidencia |
|---|---|---|
| Blue Team - Implementación | Definir herramientas de backup reales usadas | Scripts de backup, cron, logs |
| Blue Team - Prueba | Ejecutar una restauración completa y documentar | Log de prueba |
| Red Team | Evaluará disponibilidad; probar la recuperación | Informe Red Team |
| Cierre | Registrar lecciones de la prueba | Mejora del BCP |

## Check de aceptación

- [ ] RTO/RPO definidos.
- [ ] Inventario de respaldo con ubicación off-site.
- [ ] Prueba de restauración ejecutada y documentada.
- [ ] Cumple requisito BCU de backups diarios y copia fuera del sitio.
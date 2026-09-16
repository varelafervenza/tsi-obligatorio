# Plantilla ISACA/COBIT — Bitácora de Trabajo (Blue Team y Red Team)

> La **bitácora** es el registro cronológico y diario del trabajo realizado. Sirve como **evidencia de ejecución** para la auditoría: demuestra que el equipo *hizo* su trabajo, no solo que lo *dice* en un informe.
> Es obligatorio su uso en ambas partes (Blue Team y Red Team) de las tres tareas.

---

## Encabezado de mapeo normativo

| Marco | Ítem | Detalle / aporte |
|---|---|---|
| **MCU 5.0** | **Gobernar** (GV-04) / **Responder** (RS-05) | Evidencia de ejecución, trazabilidad y mejora continua. |
| **BCU — GSI** | Requisito de documentación y trazabilidad | Registro verificable de las actividades de seguridad. |
| **URCDP — Ley 18.331, Art. 9** | Due diligence en tratamiento de datos | La bitácora documenta decisiones sobre datos personales. |
| **ISO/IEC 27001:2022** | A.5.1, A.5.24, A.8.15 | Evidencias del SGSI. |
| **COBIT 2019** | MEA01 (Supervisión y evaluación) | Registro auditable del desempeño. |

---

## Control del documento

| Campo | Valor |
|---|---|
| Código | SI-BIT-99 |
| Dueño | `[Equipo / miembro]` |
| Período | `[desde] → [hasta]` |
| Versión | 1.0 |

---

## 1. Reglas de la bitácora (cómo hacerla bien)

| Regla | Detalle |
|---|---|
| **Registro diario** | Se debe cargar **el mismo día** en que se realizó el trabajo. Una bitácora escrita completa la noche anterior a la entrega no tiene valor de evidencia. |
| **Quién la escribe** | La escribe **cada miembro** por su trabajo, o el líder del día; se firma con nombre. |
| **Qué registrar** | Tarea concreta, comando/implementación realizada, resultado obtenido, hallazgo, tiempo dedicado. |
| **Evidencia anexa** | Cada entrada puede referenciar el archivo de evidencia real (`docs/evidencias/…`). |
| **No omitir fallos** | Los errores, intentos fallidos y retrabajos también se registran: en la auditoría, un fallo registrado y corregido **suma**, no resta. |
| **Horas únicas** | Usar fecha/hora con zona horaria; recomendar UTC en tareas con SIEM. |
| **Inmutabilidad** | Una vez cerrado el día, no modificar la entrada; las correcciones se agregan como nuevas filas al día siguiente. |
| **Versiones** | Guardar la bitácora como parte del repositorio (o como hoja del Excel). |

## 2. Estructura de la entrada de bitácora

Para formato **markdown** usar por entrada:

```
---
Fecha: DD/MM/AAAA
Equipo: [Blue | Red]
Responsable: [Nombre]
---

## Actividad: [Título corto]

- **Fase**: [Diseño / Implementación / Prueba / Ataque / Documentación / Auditoría]
- **Duración**: [horas]
- **Tarea realizada**: [qué se hizo, con detalle técnico]
- **Herramienta / comando**: [ej.: docker compose up -d; nmap -sV 192.168.1.10]
- **Resultado**: [éxito / parcial / fallido + descripción]
- **Evidencia anexa**: [ruta a docs/evidencias/...]
- **Incidencia / hallazgo**: [si hubo]
- **Observaciones**: [contexto adicional, decisión tomada]
```

## 3. Formato Excel (alternativo)

Se completa con las columnas `04-bitacora-planilla.xlsx`:

| Fecha | Hora | Fase | Responsable | Tarea | Herramienta/Comando | Resultado/Evidencia | Incidencia | Observaciones |
|---|---|---|---|---|---|---|---|---|

## 4. Ejemplo de entrada bien llenada

- **Fecha**: 14/09/2026 — **Equipo**: Blue — **Responsable**: M. López
- **Fase**: Implementación
- **Tarea**: Despliegue del control central y primer evento firmado.
- **Herramienta/Comando**: `docker compose -f control-central/compose.yml up -d`
- **Resultado**: Éxito. API operando en `https://192.168.10.20:8443`; se generó el primer evento de alta de contraseña y llegó al SIEM (Wazuh, índice `wazuh-alerts-*`). El correo de notificación se recibió en `rsi@correo.local`.
- **Evidencia anexa**: `docs/evidencias/2026-09-14-evento-alta.png`
- **Incidencia**: En el primer intento el SMTP rechazó la conexión por falta de TLS (error 530 5.7.1). Se corrigió configurando `smtpd_tls_cert_file` y se normalizó.
- **Observaciones**: Se dejará como alerta crítica para el Red Team el cambio de la contraseña maestra.

## 5. Bitácora en la auditoría

- Al presentar cada control, el equipo debe mostrar **la entrada de bitácora** que corresponde a la evidencia (motor de validación de autenticidad).
- El Red Team debe mantener bitácora de **cada intento de ataque** (fase, herramienta, resultado), cerrando el ciclo con la respuesta del Blue Team.
- La bitácora se incluye en la carpeta `docs/` de cada tarea y en el tag de entrega.

## Check de aceptación

- [ ] Registro diario sin lagunas superiores a `[n]` días.
- [ ] Cada miembro firma sus entradas.
- [ ] Cada hallazgo del Red Team tiene su bitácora de ataque.
- [ ] Cada control auditado puede relacionarse con una o más entradas de bitácora.
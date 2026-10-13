# Plan de Continuidad y Recuperación (BCP/DRP) — Gestor de Contraseñas con Control Centralizado

> Basado en `plantilla/isaca/06-plan-continuidad.md`. Función **Recuperar** del MCU 5.0.
> Describe los respaldos que existen de verdad, la prueba de restauración que se corrió y lo
> que todavía falta para cumplir el requisito BCU (copia fuera del sitio y backup diario automático).

---

## Encabezado de mapeo normativo

| Marco | Ítem | Detalle / aporte |
|---|---|---|
| **MCU 5.0 (función)** | Recuperar (y respaldo en Proteger) | RC-01 (plan de recuperación), RC-03 (pruebas). |
| **COBIT 2019** | DSS04 | Continuidad del servicio. |
| **ISO/IEC 27001:2022** | A.5.29, A.5.30, A.8.13 | Continuidad y respaldo de la información. |
| **BCU — GSI** | Backups diarios, copia fuera del sitio y restauración probada | Ver sección 6: diario y copia externa pendientes. |
| **URCDP — Ley 18.331, Art. 9** | Medidas de seguridad | Respaldo de la base con datos personales (`clientes`/usuarios). |
| **NIST SP 800-34** | Plan de contingencia | Referencia técnica. |

---

## Control del documento

| Campo | Valor |
|---|---|
| Código | SI-BCP-06 |
| Versión | 0.2 (aprobado por el RSI) |
| Responsable | Blue Team (Horacio Duarte, Pablo Morales, Andrés Varela) |
| Fecha | 11/10/2026 |
| Aprobado por | Andrés Varela (RSI), 12/10/2026 |

---

## 1. Objetivos de recuperación

| Métrica | Objetivo (letra, RNF-03) | Medido | Comentario |
|---|---|---|---|
| RTO — BD (`control-central`) | 4 h | 1 s para restaurar 29 eventos | La medición es sobre una base chica. Con datos reales el tiempo sería mayor. |
| RTO — servicio de front (API) | 4 h | No cronometrado | Procedimiento: `docker compose up --build -d` en `infra/` y `curl http://localhost:8001/healthz`. Ver rollback (RNF-10) abajo. |
| RPO — BD | 24 h (backup diario, requisito BCU) | Depende de la última corrida manual | El backup diario no está automatizado todavía. |
| RPO — logs | 5 min (propuesto) | Depende de la última copia | La copia del JSONL es manual. |

---

## 2. Inventario de respaldo

| Activo | Datos | Herramienta | Frecuencia | Ubicación | Retención | Estado |
|---|---|---|---|---|---|---|
| Base `control_central` | Eventos, alertas, incidentes y usuarios del panel | `pg_dump -Fc` (`scripts/backup_bd.sh`) | Manual; objetivo diario | `infra/backups/` (local, no versionado) | Pendiente de definir | Backup probado |
| Log JSONL del SIEM | Eventos sin secretos | Copia de archivo + SHA-256 | Manual | `infra/backups/` | Pendiente | Copia probada |
| Bóveda del usuario | Credenciales cifradas | Exportación `.gex` con contraseña de transporte (RF-12) | Por el usuario | Carpeta que elija el usuario | Por el usuario | Restauración probada por test automático (ver sección 3) |
| Código y configuración | Repo y `infra/` | Git | Cada commit | GitHub (fuera del equipo) | Historial completo | Activo. No contiene secretos (`.gitignore`). |
| Claves de agentes | `keys/agentes/` (pública y privada) | — | — | — | — | **Pendiente.** Hoy no tienen copia; la privada está excluida del repo por diseño. |
| Copia **fuera del sitio** de la BD | Dump de la base | — | — | — | — | **Pendiente.** Requisito BCU. |

---

## 3. Prueba de restauración

Se ejecutó el 06/10/2026 (evidencia en `docs/evidencias/06-restauracion-bd.txt`):

| Paso | Descripción | Resultado | Evidencia |
|---|---|---|---|
| 1 | Backup de la BD con `scripts/backup_bd.sh` | Dump de 20 478 bytes con SHA-256 | `06-restauracion-bd.txt` |
| 2 | Verificar la integridad del dump (`sha256sum -c`) | OK | `06-restauracion-bd.txt` |
| 3 | Restaurar en la base `control_central_restore`, separada de la original | Restauración en 1 s | `06-restauracion-bd.txt` |
| 4 | Comparar conteos original y restaurada | `audit_events` 29/29, `alerts` 4/4, `incidents` 0/0, `panel_users` 0/0 | `06-restauracion-bd.txt` |
| 5 | Copia del log JSONL y verificación de líneas y hash | 29 líneas en ambas, hash OK | `06-restauracion-bd.txt` |

**Segunda corrida con datos (06/10/2026, tarde):** se cargaron un incidente (desde la alerta 1, resuelto) y
un usuario del panel con TOTP activo. Se repitió el backup y la restauración
(`docs/evidencias/06-restauracion-bd-con-datos.txt`): los conteos coinciden en las cuatro tablas, incluidas
`incidents` 1/1 y `panel_users` 1/1. La restauración tardó 1 s. El dump tiene su SHA-256 verificado.

**Restauración de la bóveda (`.gex`):** la restauración de la exportación se prueba en el CI con el test
`vault::store::tests::favorito_y_copia_cifrada_viajan_a_otra_boveda`, que pasa en cada ejecución del
workflow `cliente-gestor` (últimos runs verdes en GitHub Actions).

**Firma del responsable:** Andrés Varela (RSI), 12/10/2026.

---

## 4. Escenarios y procedimientos de recuperación

| Escenario | Procedimiento | RTO estimado | Probado | Responsable |
|---|---|---|---|---|
| Pérdida del servidor `control-central` | 1) Levantar el stack en otro equipo con `docker compose up --build -d` en `infra/`. 2) Restaurar la BD con `scripts/restaurar_bd_prueba.sh` apuntando al último dump. 3) `keys/agentes/` arranca vacía: cada agente vuelve a tocar "Guardar y copiar clave pública" desde su propia app. No hace falta backup: `keys/agentes/` solo tiene públicas, y cada cliente ya tiene la suya (decisión de custodia, `01-Politica-Seguridad.md` sección 5.1). 4) Verificar con `healthz` y una firma válida. | No estimado (depende de la descarga de imágenes) | Restauración de la BD: sí. Stack completo: no. | Pablo Morales |
| Corrupción de la BD | Restaurar desde el último dump en una base nueva, validar conteos y renombrar la base. | < 1 h para la base actual | Sí (restauración en base separada) | Pablo Morales |
| Pérdida de la bóveda de un usuario | Importar el `.gex` con su contraseña de transporte. Si se perdió esa contraseña, el archivo `.gex` no se puede abrir: es el diseño del cifrado, y se avisa al usuario al exportar. | Inmediato si tiene el archivo | Sí (test automático en CI) | Usuario final |
| Caída del SIEM (Wazuh) | El control central sigue guardando el JSONL y las alertas de la API. Al volver el manager, el agente relee `audit-events.jsonl`. | Manager + agente 4.14.8 en el compose (sin indexer) | Parcial (alerta 100120 verificada el 07/10) | Andrés Varela |
| Falla del correo | El evento se guarda igual; el correo no se reintenta. Se verifica en el panel de alertas y en Mailpit. | No aplica | No | Andrés Varela |

### Rollback (RNF-10)

La letra pide documentación de despliegue, config y rollback. El despliegue y la config están en
`README.md` (sección backend) y en `infra/`. El rollback, por caso:

**API o contenedor caído.** Desde `tarea1-gestor-contrasenas/infra/`:

```powershell
docker compose up --build -d
curl http://localhost:8001/healthz
```

Esperado: `{"status":"ok","database":"up"}`. Si el contenedor quedó a medias:
`docker compose down` y el `up` de nuevo. No toca la bóveda del usuario.

**Base corrupta o datos mal cargados.** No se restaura sobre `control_central` a ciegas.
El script de prueba levanta una base aparte y compara conteos:

```powershell
bash scripts/backup_bd.sh
bash scripts/restaurar_bd_prueba.sh
```

Corrido el 06/10: 1 s, conteos iguales (`docs/evidencias/06-restauracion-bd-con-datos.txt`).
Si hay que volver atrás de verdad: tomar el dump de `infra/backups/`, restaurarlo sobre una
base nueva y cambiar el `DATABASE_URL` de `control-central.env` a esa base. El script de
prueba no pisa producción.

**App de escritorio mala (instalador nuevo).** Bajar el `.exe` del Release anterior
(`v0.0-test` hoy; `v1.0` cuando exista) e instalar encima. La bóveda queda en la carpeta
de datos de la app (`uy.tsi.gestor-contrasenas`), no en el instalador.

**Bóveda perdida o a volver atrás.** Importar el `.gex` con la contraseña de transporte
(RF-12). Lo cubre el test `favorito_y_copia_cifrada_viajan_a_otra_boveda` del workflow
`cliente-gestor`. Sin esa contraseña el archivo no se abre: es el diseño.

**Código.** `git checkout` del tag o del commit anterior. El repo no guarda dumps ni
`.sqlite` (`.gitignore`).

Qué no cubre este rollback: backup diario automático, copia fuera del equipo y copia de
`keys/agentes/`. No los pide la letra; están en la sección 6 por el BCU.

---

## 5. Comunicación de crisis

| Canal | Herramienta | Contacto | Estado |
|---|---|---|---|
| Correo de alerta | Mailpit. Mailu no se despliega en este laboratorio | `rsi@correo.local` | Implementado con Mailpit |
| Dashboard de estado | Grafana (tablero "Control central") | `http://localhost:3000` | Implementado |
| Teléfono | No implementado (Wazo es opcional en la letra) | — | N.A. justificado en el Excel de controles |

---

## 6. Pendientes para cumplir el requisito BCU

1. **Backup diario automático** con el Programador de tareas de Windows o cron, llamando a `scripts/backup_bd.sh`.
2. **Copia fuera del sitio** de los dumps y de los logs: otra unidad física o un servicio de almacenamiento. Hoy todo queda en `infra/backups/`, en el mismo equipo.
3. **Política de retención** de dumps: definir cuántos días se guardan.
4. **Repetir la restauración** cuando haya incidentes y usuarios del panel, para que la prueba cubra todas las tablas.

**Decidido, no pendiente:** la copia de `keys/agentes/` no se hace. El RSI decidió (12/10/2026, ver
`01-Politica-Seguridad.md` sección 5.1) que ninguna clave privada de agente se respalda en ningún
lugar fuera de la máquina que la generó, para no agregar un blanco de ataque extra. Si se pierde la
privada, se enrola el agente de nuevo; es el comportamiento esperado, no una falla.

---

## Guía de llenado (Blue Team)

| Fase | Cómo completar | Con qué evidencia |
|---|---|---|
| Implementación | Backup con `scripts/backup_bd.sh` y copia de logs | Dump con SHA-256, `06-restauracion-bd.txt` |
| Prueba | Restaurar en base separada y comparar conteos | Salida de `scripts/restaurar_bd_prueba.sh` |
| Pendiente | Automatizar el backup diario y la copia externa | Tarea programada, copia en otro medio |
| Red Team | Evaluará la recuperación del servicio | Informe Red Team |

---

## Check de aceptación

- [x] RTO y RPO definidos (la letra y el BCU).
- [x] Inventario de respaldo con ubicación.
- [ ] Copia fuera del sitio (pendiente, requisito BCU).
- [x] Prueba de restauración ejecutada y documentada.
- [x] Prueba con tablas no vacías (incidentes y usuarios del panel).
- [ ] Backup diario automático (pendiente, requisito BCU).
- [x] Firma del responsable: Andrés Varela (RSI), 12/10/2026.

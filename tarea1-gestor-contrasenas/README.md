# Tarea 1 — Gestor de Contraseñas con Control Centralizado

## Contenido

- **`LETRA.md`** — La letra completa de la tarea: marco teórico, glosario, requerimientos funcionales/no funcionales, arquitectura sugerida, Partes Blue Team y Red Team, matriz de documentación, hitos, KPIs y criterios de evaluación.
- **`docs/`** — Carpeta de trabajo del equipo. Aquí se pegan las plantillas de la carpeta `../plantilla/` y se completan con la documentación de la solución.
- **`cliente-gestor/`** — App de escritorio offline (Tauri: Rust + React/TS). Bóveda cifrada local, MFA, generador de contraseñas, emisor de eventos firmados.
- **`control-central/`** — API REST (FastAPI + PostgreSQL) que recibe eventos de auditoría, gestiona usuarios/MFA del panel y alimenta el dashboard.
- **`infra/`** — `docker-compose.yml` del control central + notas de despliegue de Wazuh, Mailu, Grafana y TheHive.
- **`scripts/`** — Utilidades: envío de evento de prueba, verificación de integridad del tag de entrega.

Stack y decisiones de arquitectura completas en la bitácora `docs/99-bitacora-trabajo.md` (entrada del 15/09/2026). Lo implementado el 03/10/2026 está en las entradas de Horacio Duarte de ese día.

## Estado al 03/10/2026

En código ya están la bóveda offline, los eventos firmados (incluido el vencimiento), el correo por Mailpit, los incidentes, el panel `GET /api/dashboard/kpis`, el tablero de Grafana, los usuarios del panel con TOTP y hash elegible, el delay al fallar la maestra, y las reglas de Wazuh en `infra/wazuh/local_rules.xml`. El enrolamiento TOTP de la bóveda muestra un QR para la app de autenticación. La ruta inicial del archivo es `boveda-prueba.sqlite` en la carpeta de datos de la app.

Sigue afuera del código, como despliegue o evidencia: TLS y WebAuthn/Windows Hello. El correo del laboratorio es Mailpit (decisión en `docs/00-arquitectura-c4.md`). El manager y el agente de Wazuh suben con el compose de `infra/` (sin indexer; las alertas quedan en `alerts.json`). La retención de 90 días corre al arrancar el control central. Hay que reconstruir el compose para crear la tabla `alerts`.

## Cómo empezar

1. Lea `LETRA.md` completo.
2. Revise la matriz de documentación (sección 8) para saber qué plantilla llenar en cada semana.
3. Copie de `../plantilla/isaca/` las plantillas indicadas a su carpeta `docs/` (o copie toda la carpeta `plantilla` como referencia).
4. Trabaje en su repositorio GIT y congele la entrega con `git tag v1.0` en el hito H4.

## Documentación que debe entregar (resumen)

| Fecha | Documento |
|---|---|
| Semana 1 (14-18/09) | Política de seguridad, arquitectura (4+1 y C4), inventario de activos, Excel activos |
| Semana 2 (21-25/09) | Análisis de riesgos, Excel controles MCU (Avanzado) |
| Semana 3-4 (28/09-02/10) | Gestión de accesos (TOTP/WebAuthn/Hello/Argon2), monitoreo/logs, bitácora |
| Semana 5 (05-07/10) | Vulnerabilidades, plan de continuidad, incidentes, SoA + brecha MCU, notificaciones |
| **Miércoles 07/10/2026** | **Pre-entrega congelada** (`git tag v1.0`) + Excel/bitácora completos |
| **Miércoles 14/10/2026** | **Auditoría formal por función MCU 5.0 (defensa)** — demo + recorrido de controles |
| **Miércoles 28/10/2026** | **Pre-entrega Red Team** (informe preliminar ≥ 70%) |
| **Lunes 09/11/2026** | **Entrega final Red Team** (informe `plantilla/informe-red-team.md` + presentación) |

- **Equipo A (Blue Team):** desarrolla la solución y rinde la auditoría (semana del 14/09 al 14/10).
- **Equipo B (Red Team):** ataca la solución entregada (28/10 → 09/11) y emite informe.

## Requisitos y cómo evaluar

### 1. Backend (`control-central`) — cualquier máquina con Docker

Requisito: Docker Desktop abierto. Desde `infra/`:

```powershell
copy .env.example .env
copy control-central.env.example control-central.env
docker compose up --build -d
curl http://localhost:8001/healthz
```

Esperado: `{"status":"ok","database":"up"}`. La API queda en el puerto **8001** del host
(`http://localhost:8001/docs`); dentro del contenedor escucha en 8000. Grafana en
`http://localhost:3000` y Mailpit en `http://localhost:8025`.

Ese mismo `docker compose up` levanta Wazuh. La primera vez baja las imágenes
`wazuh/wazuh-manager:4.14.8` y `wazuh/wazuh-agent:4.14.8` (pesan) y el agente espera a que
el manager pase el chequeo de salud, cerca de un minuto. No hay pantalla de Wazuh: el
indexer no entra en este Docker. Las alertas quedan en `alerts.json` y el panel las cuenta
en `alertas_siem`.

Desde `infra/`, cuando el manager figura como healthy:

```powershell
docker exec infra-wazuh-manager-1 /var/ossec/bin/agent_control -l
```

Esperado: `agente-control-central` en estado **Active**. El agente lee
`infra/logs/audit-events.jsonl`. Una línea con `"tipo": "cambio_maestra"` dispara la regla
**100120**. Cinco `intento_fallido_maestra` del mismo `agente_id` en dos minutos disparan
la **100101**. Cinco `borrado_credencial` en las mismas condiciones disparan la **100111**.
El conteo de esas reglas (y de las base 100100 y 100110) está en
`http://localhost:8001/api/dashboard/kpis`, campo `kpis.falsos_positivos.alertas_siem`.

Para ver las últimas alertas:

```powershell
docker exec infra-wazuh-manager-1 tail -n 5 /var/ossec/logs/alerts/alerts.json
```

No hay puertos publicados en el host: el agente y el manager se hablan por la red interna
de Docker (`blue-team-net`). Para el manager, usar `docker exec` como en los dos comandos
de arriba. El detalle y lo que queda afuera (indexer, FIM de la bóveda en Windows, retención
del JSONL) está en `infra/wazuh/README.md`.

### 2. Tests del cliente — Linux o Windows con Rust

Los tests de la lógica de la bóveda (cripto, TOTP, generador, exportación) corren en el CI
(`.github/workflows/cliente-gestor.yml`). Para correrlos localmente hace falta Rust y, en
Windows, Visual Studio Build Tools con el workload de C++.

### 3. Aplicación de escritorio — Windows

Requisitos: Node 20 y Rust (sólo si se compila) o el instalador `.exe` del artefacto
`cliente-gestor-instalador` del workflow de GitHub Actions.

**Limitación de Windows 11 con Smart App Control:** el instalador no está firmado digitalmente.
Smart App Control lo bloquea mientras está activo. El equipo decidió no firmar el instalador
(costo) y el evaluador tiene que desactivar Smart App Control para ejecutarlo, o usar un Windows
sin esa función (por ejemplo Windows 10).

**Para que los eventos queden firmados** (`firma_valida: true`), el central tiene que tener la
clave pública del agente. Con el stack de `infra/` ya levantado:

1. En la app, abrir **Control central**.
2. Dejar la URL `http://localhost:8001/api/events/`.
3. En **Carpeta de claves públicas del central**, pegar la ruta absoluta a
   `tarea1-gestor-contrasenas\keys\agentes` (la carpeta del repo, no un archivo).
4. Tocar **Guardar y copiar clave pública**. El aviso tiene que decir que copió un `.pub.pem`.
5. Recién después: alta, modificación o borrado. En `http://localhost:8001/api/events/` el último
   evento tiene que traer `firma_valida: true`.

Sin ese paso el evento igual se guarda, pero llega como no firmado. Detalle en
`cliente-gestor/README.md` (sección «Eventos al control central») y prueba C-04 de
`docs/guia-de-pruebas.md`.

### 4. Limitaciones de alcance (acordadas con el docente)

- **Windows Hello y WebAuthn** no forman parte de la entrega. Ver `docs/09-Gestion-Accesos.md`.
- **TLS** no se usa en el laboratorio. Ver `docs/00-arquitectura-c4.md`.
- **Mailpit** es el correo del laboratorio. Mailu, SPF y DKIM no se despliegan: el aviso no sale de Docker. Ver `docs/00-arquitectura-c4.md`.

Las limitaciones y sus riesgos residuales están en `docs/03-Analisis-Riesgos.md` (R05 y R06).

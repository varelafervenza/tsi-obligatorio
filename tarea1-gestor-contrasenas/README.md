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

Sigue afuera del código, como despliegue o evidencia: levantar el manager de Wazuh (las mismas reglas ya corren en la API y se clasifican en `/api/alerts/`), Mailu con SPF/DKIM, TLS, y WebAuthn/Windows Hello. La retención de 90 días corre al arrancar el control central. Hay que reconstruir el compose para crear la tabla `alerts`.

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
# Instrucciones para Claude en este repositorio

Repositorio del curso Seguridad de la Información (equipo Blue Team: Horacio Duarte, Pablo Morales, Andrés Varela).
El trabajo principal es la **Tarea 1**, en `tarea1-gestor-contrasenas/`.

## Antes de empezar

- **Pendientes de la entrega**: `tarea1-gestor-contrasenas/docs/00-pendientes-entrega.md`. Leélo antes de proponer trabajo.
  Actualizalo en el mismo commit que cierra un punto.
- **Enunciado**: `tarea1-gestor-contrasenas/LETRA.md`.
- **Bitácora**: `tarea1-gestor-contrasenas/docs/99-bitacora-trabajo.md`. Cada commit de la Tarea 1 lleva su entrada, con responsables reales.

## Cómo trabajar

- Commit y push van en **pasos separados**, cada uno con confirmación previa del usuario.
- Antes de cambiar el repositorio, proponé el mensaje de commit y los archivos. Esperá el "sí".
- Los commits pasan por el hook de `gitleaks` (`.pre-commit-config.yaml`). Si el hook bloquea, no lo saltees con `--no-verify`: revisá el hallazgo.
- Si un equipo no tiene el hook instalado, instalarlo con `python -m pip install pre-commit` y `python -m pre_commit install`.

## Tests de `control-central`

No hay Python instalado con las dependencias en el host. Los tests corren dentro de la imagen Docker, contra la base `control_central_test`, nunca contra `control_central`:

```
docker run --rm --network infra_blue-team-net -e DATABASE_URL=<url a control_central_test> \
  -v "<ruta>/control-central/tests:/app/tests:ro" infra-control-central \
  sh -c "pip install -q pytest==8.3.3 httpx==0.27.2 && python -m pytest -q tests"
```

Requiere que la base `control_central_test` exista y que el stack esté levantado desde `tarea1-gestor-contrasenas/infra/`.

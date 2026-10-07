# Instrucciones para Claude en este repositorio

Repositorio del curso Seguridad de la Información (equipo Blue Team: Horacio Duarte, Pablo Morales, Andrés Varela).
El trabajo principal es la **Tarea 1**, en `tarea1-gestor-contrasenas/`.

## Antes de empezar

- **Pendientes de la entrega**: `tarea1-gestor-contrasenas/docs/00-pendientes-entrega.md`. Leélo antes de proponer trabajo.
  Actualizalo en el mismo commit que cierra un punto.
- **Enunciado**: `tarea1-gestor-contrasenas/LETRA.md`.
- **Bitácora**: `tarea1-gestor-contrasenas/docs/99-bitacora-trabajo.md`. Cada commit de la Tarea 1 lleva su entrada, con responsables reales.

## Cómo trabajar

- **Antes de cada commit, hacé `git fetch` y revisá si hay commits nuevos en `origin` que tu copia local todavía no
  tiene** (`git log --oneline HEAD..origin/<rama>`). El equipo pushea directo sin pasar por esta sesión, y un
  sync automático (de VS Code, por ejemplo) puede traer esos commits a la copia local sin que lo notes. Si hay
  commits nuevos, avisá qué son antes de seguir.
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

## Compilar o testear `cliente-gestor` (Rust) en este equipo

`cargo check`/`cargo build`/`cargo test` suelen fallar con `could not exec the linker link.exe: Access is denied
(os error 5)`, o lo mismo con `rustc.exe`. No es Smart App Control (verificado:
`VerifiedAndReputablePolicyState=0`, o sea apagado) ni un proceso que retenga el archivo. Es Windows Defender
escaneando cada ejecutable nuevo del linker mientras varios corren en paralelo: es intermitente, y cada intento
avanza más porque `cargo` cachea lo que ya compiló. Reintentar con `-j 1` unas pocas veces (hasta 5-8) resuelve.
Lo que funcionó fue reintentar `cargo check`/`test` con `-j 1` por fuera del sandbox del Bash tool
(`dangerouslyDisableSandbox: true`), unas 5 a 8 veces. No queda confirmado si el sandbox es parte de la causa o
si alcanzaba con reintentar: no se probó a fondo dentro del sandbox. El build de `npm run build`
(TypeScript + Vite) no tiene este problema y corre normal dentro del sandbox.

# RT-P11 — Revisión de secretos y configuración del repositorio

**Fecha:** 30/10/2026 (America/Montevideo). **Objetivo de la consigna:** RT-11, OSINT del repositorio. **Tipo:** revisión de solo lectura; no se ejecutó la aplicación ni se enviaron solicitudes al backend.

## Qué se revisó

Se examinaron los archivos versionados de `tarea1-gestor-contrasenas` y los **87 commits accesibles en las referencias locales de Git**. Se buscaron nombres de archivos sensibles (`.env`, `.pem`, `.key`, `id_rsa`, credenciales), delimitadores de claves privadas, formatos conocidos de tokens de GitHub y AWS, JWS compactos y asignaciones literales de contraseñas o tokens. Se revisaron las plantillas de entorno, `.gitignore`, `Dockerfile` y `infra/docker-compose.yml`. También se comprobó por separado la carpeta Red Team **sin versionar**, para evitar publicar evidencia con valores completos.

La búsqueda por patrones no sustituye una auditoría exhaustiva de todos los formatos de secreto, la inspección de todas las ramas remotas posibles ni el análisis de imágenes Docker. No se leyeron claves locales privadas ni bóvedas personales. El resultado resumido, sin valores sensibles, está en [evidencias/RT-P11](evidencias/RT-P11/resumen-revision.txt).

## Resultado

1. **Historial Git revisado:** no se encontró una clave privada PEM, un token GitHub/AWS con los formatos buscados ni un JWS compacto completo. Los únicos archivos de entorno encontrados en el historial fueron `infra/.env.example` y `infra/control-central.env.example`; son plantillas. Las asignaciones que merecieron revisión adicional correspondían a un token fijo y su hash en una prueba automatizada; el token se crea dentro de esa prueba y no se identificó como credencial operativa.
2. **Configuración actual:** `tarea1-gestor-contrasenas/.gitignore` excluye los `.env` reales, `keys/`, bóvedas `*.sqlite` y copias. El compose versionado requiere variables para las contraseñas de PostgreSQL y Grafana. El Dockerfile usa `python:3.12-slim` y no fija un digest de imagen; esto es una observación de reproducibilidad y actualización, **no una vulnerabilidad explotada**. La copia local del compose tiene cambios de laboratorio y no se atribuye a la entrega original.
3. **Archivos Red Team locales aún sin versionar:** nueve archivos de Postman/evidencia contienen **tres JWS distintos** repetidos. Sus campos `exp` ya habían vencido al 30/10/2026. Son mensajes firmados de auditoría usados para pruebas, no claves privadas ni tokens de login del panel. No se demostró que funcionen ahora contra la API. Deben reemplazarse por marcadores o valores truncados en la copia que se publique; conservar los originales únicamente en un archivo privado si se necesitan para trazabilidad. El archivo `docs/red-team.rar` no se inspeccionó y debe revisarse por separado antes de publicarlo.

## Conclusión y recomendación

**RT-11 fue abordado con una revisión acotada y no produjo un secreto operativo confirmado en el historial local.** Existe una tarea de higiene de publicación: retirar los JWS completos de las evidencias locales antes de incorporarlas a Git. No se asigna CVSS ni se declara acceso indebido. Mantener `.env` y claves fuera de Git, usar variables y secretos distintos por entorno, y repetir la revisión antes de cada publicación. Si se descubre una credencial operativa, revocarla y documentar el incidente sin copiar su valor al informe.


# Nota sobre las evidencias publicadas

La copia de `docs/red-team/` publicada en GitHub sustituye tres valores completos de `firma_jws` por marcadores `FIRMA_JWS_REDACTADA_1`, `_2` y `_3`. Los valores originales se conservan únicamente en los archivos locales de laboratorio. La sustitución impide usar directamente los cuerpos históricos de Postman o los JSON publicados para reenviar esos eventos; para una reproducción se debe generar una firma nueva y vigente con el agente autorizado.

Todos los archivos `SHA256SUMS.txt` de `evidencias/` corresponden a los bytes de **esta copia publicada**, después de la redacción y de la normalización de texto de Git. Las capturas PNG no se editaron. La sustitución conserva la igualdad o diferencia entre firmas de cada caso, así como los IDs, las marcas de validación y los tiempos registrados, pero no permite verificar criptográficamente el JWS original desde la copia pública.

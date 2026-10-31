# RT-05 — Revisión documental del canal sin TLS

**Fecha de revisión:** 31/10/2026. **Tipo:** análisis documental, sin ataque ejecutado. **Estado del objetivo:** no probado mediante man-in-the-middle.

## Evidencia de la decisión del Blue Team

En [Arquitectura C4, «Limitación de alcance: TLS en el laboratorio»](../00-arquitectura-c4.md), líneas 85–109, el Blue Team declara que la entrega no usa TLS en el canal cliente → API ni en el correo, que el diseño objetivo de TLS 1.2+ no está implementado en la versión congelada y que el tráfico de cliente a API circula en texto plano en la red de laboratorio. También registra la aceptación del riesgo residual de lectura o alteración de eventos por alguien con acceso a esa red.

En [Análisis de riesgos](../03-Analisis-Riesgos.md), fila R05, el Blue Team clasifica el riesgo de MITM como una limitación aceptada, con mitigación no aplicada. El [README](../README.md) también enumera TLS como pendiente de la entrega.

El [extracto de evidencia](evidencias/RT-05/01-fuentes-documentales.txt) conserva los pasajes relevantes, las ubicaciones y los hashes SHA-256 de ambos documentos al momento de esta revisión. Son declaraciones del Blue Team; no sustituyen una medición de tráfico ni una prueba de ataque.

## Alcance y conclusión

**No se realizó interceptación, modificación de tráfico, prueba de certificados ni man-in-the-middle.** La ausencia de TLS ya estaba documentada y aceptada en el laboratorio; esta revisión no constituye un hallazgo nuevo del Red Team. RT-05 queda **documentado como limitación conocida, sin prueba técnica de explotación**. La aceptación del riesgo del laboratorio no equivale a que HTTP sea adecuado para otro despliegue.

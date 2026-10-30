# RT-P08 — Enumeración acotada de puertos del backend

**Fecha de ejecución informada:** 28/10/2026, America/Montevideo. **Origen:** Zenmap/Nmap en la VM `Windows-Lab-RedTeam`. **Destino:** VM Linux del laboratorio `192.168.77.10`. Esta prueba nueva se ejecutó desde la VM Windows; no cambia la procedencia local de P01–P07.

## Objetivo y procedimiento

Comprobar qué puertos seleccionados del backend son accesibles desde la red interna del laboratorio. Se ejecutó un único escaneo TCP de conexión sobre tres puertos concretos. No se hizo detección de versiones, scripts NSE, explotación ni prueba de carga.

```text
nmap -sT -p 8001,3000,8025 --max-rate 5 -Pn 192.168.77.10
```

Un intento previo escribió `80001` en vez de `8001`; Nmap lo rechazó porque excede el rango válido de puertos. Ese intento no produjo resultados de red. La captura conservada corresponde al comando corregido.

## Resultado observado

Zenmap informó que `192.168.77.10` estaba disponible y que **3000/tcp, 8001/tcp y 8025/tcp estaban abiertos** desde la VM Windows. En la arquitectura del laboratorio esos puertos se asignan respectivamente a Grafana, API y Mailpit, pero este escaneo solo comprobó el estado TCP. Los nombres `ppp`, `vcom-tunnel` y `ca-audit-da` mostrados por Nmap son etiquetas de su base de puertos; no identifican por sí mismos los servicios reales.

La accesibilidad se comprobó **desde la red interna de VirtualBox**. No se probó acceso desde Internet ni se descubrió una vulnerabilidad por el mero hecho de que un puerto esté abierto. Para confirmar cada servicio se requiere una petición de aplicación separada y para valorar exposición se debe contrastar la política de red prevista.

## Evidencia y límites

- [Captura original de Zenmap](evidencias/RT-P08/01-zenmap-puertos.png) y [SHA-256](evidencias/RT-P08/SHA256SUMS.txt).
- La captura muestra en la VM `2026-10-21 08:51 -0300`, mientras que la ejecución se informó en esta conversación el 28/10/2026. **El reloj de la VM Windows está desactualizado**; esa hora no se usa como hora real de ejecución ni como timestamp de evidencia independiente.
- No se capturaron paquetes ni versiones de servicio. El resultado representa un punto de observación y no acredita cobertura de toda la superficie de red.

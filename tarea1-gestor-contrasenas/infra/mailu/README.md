# Mailu (servidor de correo propio)

Mailu empaqueta Postfix+Dovecot+Rspamd+webmail en un compose oficial generado por su
asistente web, lo que evita configurar cada pieza a mano.

```bash
# Generar el docker-compose.yml y mailu.env con el asistente oficial:
# https://setup.mailu.io/  (elegir dominio del laboratorio, ej. correo.local)
docker compose -f docker-compose.yml up -d
```

## Puntos críticos para esta tarea (los va a probar el Red Team — RT-04)

- [ ] Configurar **SPF, DKIM y DMARC** correctamente para el dominio del laboratorio.
- [ ] Deshabilitar **relay abierto** (solo el control-central puede enviar).
- [ ] TLS en el submission port (587) — no aceptar auth en texto plano.
- [ ] Probar el envío de notificación ante alta/mod/borrado/cambio de maestra (RF-07)
      y capturar evidencia en `docs/evidencias/`.

## H2 (mientras tanto): Mailpit

El prototipo envía por SMTP a **Mailpit** (`infra/docker-compose.yml`, UI en
`http://localhost:8025`). Las variables `SMTP_*` de `control-central.env.example`
apuntan ahí. Cuando Mailu esté en `blue-team-net`, cambiar `SMTP_HOST`/`SMTP_PORT`
y activar TLS + auth. El código del mailer no cambia.

## Pendiente

- [ ] Generar el compose con el asistente y copiarlo a este directorio.
- [ ] Conectar a la red `blue-team-net` de `infra/docker-compose.yml`.
- [ ] Apuntar `SMTP_HOST` a Mailu y probar SPF/DKIM/DMARC.

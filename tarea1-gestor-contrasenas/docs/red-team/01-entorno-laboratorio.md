# Entorno de laboratorio Red Team

Preparación: 19/10/2026. Este documento describe la instalación del laboratorio; no registra ataques ni confirma hallazgos.

## Entorno realmente usado en las pruebas documentadas

La VM Windows se preparó, pero **no fue el origen de las pruebas Red Team documentadas en P01–P07**. El gestor de escritorio y Postman se ejecutaron en el Windows principal del usuario. La bóveda ficticia y los archivos del agente pertenecen a ese cliente. El backend, PostgreSQL, Mailpit y Wazuh siguieron alojados en la VM Linux `Linux-Backend-RedTeam`. Desde Windows principal, la API se alcanzó mediante un túnel SSH en `127.0.0.1:18001` hacia `192.168.77.10:8001` en Linux; `localhost` designa el extremo local del túnel, no una segunda base de datos instalada en Windows.

P05 y P07 evaluaron únicamente el cliente y su bóveda local. P01–P04 combinaron cliente o Postman locales con verificaciones del backend Linux. P06 solo preparó una cuenta ficticia en el backend; los intentos incorrectos previstos no se ejecutaron. La preparación de la VM Windows descrita abajo no debe confundirse con el lugar de ejecución de estas pruebas.

## Máquinas

| Máquina VirtualBox | Función | Recursos |
|---|---|---|
| Windows-Lab-RedTeam | Cliente de escritorio y herramientas de evaluación preparados; no usados en P01–P07 documentadas | 8 GB RAM, 4 CPU, disco virtual de 100 GB. |
| Linux-Backend-RedTeam | Ubuntu Server 24.04 LTS, Docker Engine y servicios | 8 GB RAM, 4 CPU, disco virtual dinámico de 100 GB. |

Los archivos Linux se guardan en `D:\Laboratorio-RedTeam\Linux-Backend\`; la configuración de VirtualBox está en `D:\Laboratorio-RedTeam\Linux-Backend-RedTeam\`.

La imagen Ubuntu se descargó de `https://cloud-images.ubuntu.com/noble/current/noble-server-cloudimg-amd64.vmdk` y se comparó su SHA-256 con el manifiesto del mismo servidor. El comprobante local es `imagen-verificada.txt`.

## Red

Las dos máquinas tienen una segunda tarjeta conectada a la red interna VirtualBox `redteam-lab` (192.168.77.0/24). Linux tiene la dirección fija `192.168.77.10`. Windows recibe una dirección entre 192.168.77.20 y 192.168.77.50 mediante el DHCP de VirtualBox.

La primera tarjeta usa NAT para descargar paquetes y actualizaciones. NAT permite conexiones salientes: no implica aislamiento total del host ni de otras redes. Las pruebas deben dirigirse exclusivamente a los objetivos de laboratorio acordados. No hay adaptadores en modo puente ni carpetas compartidas configuradas por esta preparación.

Los servicios del proyecto se publican únicamente en la IP de la tarjeta interna de Linux. SSH administrativo se reenvía al loopback del host, `127.0.0.1:22222`.

## Acceso desde la VM Windows

| Servicio | Dirección |
|---|---|
| Salud de la API | `http://192.168.77.10:8001/healthz` |
| Documentación de la API | `http://192.168.77.10:8001/docs` |
| Grafana | `http://192.168.77.10:3000` |
| Correo Mailpit | `http://192.168.77.10:8025` |
| URL de eventos para el gestor | `http://192.168.77.10:8001/api/events/` |

PostgreSQL y Wazuh no publican puertos en la VM: se comunican por la red Docker. Wazuh usa manager y agente, sin indexer ni interfaz gráfica, según el compose del proyecto.

## Acceso administrativo desde el host

En PowerShell:

```powershell
ssh -i D:\Laboratorio-RedTeam\Linux-Backend\lab-admin.key -p 22222 lab@127.0.0.1
```

La primera conexión pide confirmar la clave del servidor. Hay una referencia guardada en `D:\Laboratorio-RedTeam\Linux-Backend\known_hosts`.

El usuario de consola es `lab`; su contraseña se guarda en `D:\Laboratorio-RedTeam\Linux-Backend\acceso-local.txt`. SSH utiliza clave y no permite autenticación por contraseña.

Dentro de Linux:

```bash
cd /opt/redteam-lab/infra
sudo docker compose ps
sudo docker compose logs --tail=50 control-central
sudo docker compose logs --tail=50 wazuh-manager
sudo docker compose stop
sudo docker compose start
```

Para consultar las credenciales de Grafana dentro de Linux:

```bash
sudo cat /opt/redteam-lab/infra/.env
```

Ese archivo contiene contraseñas del laboratorio: no incorporarlo al repositorio ni a capturas del informe. Se generaron claves nuevas y no se copiaron los `.env` ni las claves de agentes del host. Para probar firmas válidas, será necesario enrolar la clave pública del cliente de prueba en `/opt/redteam-lab/keys/agentes/`.

## Copia de la solución

El backend está en `/opt/redteam-lab/`, copiado desde los archivos locales del proyecto. No se extrajo del tag congelado. Antes de una evaluación formal, confirmar la versión entregada con el Blue Team y documentar las diferencias.

Se modificó solamente la copia Linux del compose para sustituir los enlaces a `127.0.0.1` por `192.168.77.10` en API, Grafana y Mailpit, permitiendo acceso desde Windows. Se normalizaron los finales de línea de scripts `.sh` de Wazuh y se generaron credenciales nuevas. No atribuir a la entrega original cambios de exposición introducidos por este laboratorio.

## Uso y recuperación

1. Iniciar Linux-Backend-RedTeam desde VirtualBox y esperar a que los servicios respondan.
2. Iniciar Windows-Lab-RedTeam y acceder a las direcciones anteriores.
3. Para apagar Linux de forma ordenada, ejecutar `sudo poweroff` por SSH o desde su consola.
4. Apagar Windows desde su menú Inicio.
5. Antes de pruebas que alteren datos, tomar instantáneas de las máquinas apagadas.

## Cliente de escritorio en Windows

Se compiló el cliente local con `npm run tauri build -- --no-bundle`, en perfil release, incluyendo el frontend React dentro del ejecutable. El build terminó correctamente con dos advertencias de código sin uso. No requiere Node, Rust ni Vite para ejecutarse en la VM.

El ejecutable está en `D:\Laboratorio-RedTeam\Cliente-Windows\cliente-gestor.exe` y se incluyó en `cliente-gestor.iso`, montado en la unidad de DVD de Windows-Lab-RedTeam con etiqueta `GESTOR_REDTEAM`. Se comprobó que su SHA-256 coincide con la copia incluida en el DVD. El manifiesto está junto al ejecutable y dentro del DVD.

Dentro de Windows, abrir Explorador de archivos → Este equipo → DVD GESTOR_REDTEAM, copiar `cliente-gestor.exe` a una carpeta local (por ejemplo, Documentos\Gestor-Lab) y abrir esa copia. El DVD contiene también `LEEME.txt`. La transferencia no usa carpetas compartidas.

Quedan pendientes la apertura y comprobación del cliente dentro de Windows, configurar su URL de auditoría a `http://192.168.77.10:8001/api/events/` y registrar su clave pública en Linux. No se crearon bóvedas ni se copiaron claves existentes. Preparar el cliente y los servicios no ejecuta pruebas de seguridad ni completa el informe Red Team.

## Comprobaciones de instalación

Se verificaron seis contenedores en ejecución, PostgreSQL y Mailpit saludables, API con respuesta `{"status":"ok","database":"up"}`, Grafana con base de datos `ok`, Mailpit con HTTP 200 y el agente `agente-control-central` en estado `Active` en el manager Wazuh.

Se creó el archivo vacío de auditoría para que el agente pueda observarlo desde el inicio. No se generaron eventos de ataque. La unidad `redteam-lab.service` está habilitada para iniciar los contenedores automáticamente al encender Linux.

Se guardó la instantánea `Backend instalado` con Linux apagado ordenadamente. La instantánea `Windows limpio` es anterior a la incorporación de la segunda tarjeta de red; restaurarla puede requerir volver a conectar Windows a `redteam-lab`.

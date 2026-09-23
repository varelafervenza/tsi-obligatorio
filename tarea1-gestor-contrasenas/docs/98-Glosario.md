# Glosario de siglas — Tarea 1

> Glosario de trabajo del equipo con las siglas usadas en la documentación de esta tarea.
> Complementa a `plantilla/isaca/GLOSARIO.md`, que es el glosario oficial del curso (más
> orientado a términos conceptuales que a siglas técnicas).

## Marcos normativos y evaluación

| Sigla | Significado | Para qué sirve acá |
|---|---|---|
| MCU 5.0 | Marco de Ciberseguridad 5.0 (AGESIC, Uruguay) | Estándar contra el que se audita el Blue Team; define las funciones Gobernar/Identificar/Proteger/Detectar/Responder/Recuperar y el perfil Avanzado que tenemos que cumplir. |
| AGESIC | Agencia de Gobierno Electrónico y Sociedad de la Información y del Conocimiento | Organismo uruguayo que publica el MCU 5.0. |
| BCU — GSI | Banco Central del Uruguay — Guía de Seguridad de la Información | Exige 2FA, monitoreo de cambios sensibles y notificación de incidentes. |
| URCDP | Unidad Reguladora y de Control de Datos Personales | Aplica la Ley 18.331 (protección de datos personales) en Uruguay. |
| ISO/IEC 27001 | Norma internacional de sistemas de gestión de seguridad de la información | Sus controles se citan como "A.5.1", "A.8.15", etc. |
| COBIT 2019 | Marco de gobierno de TI de ISACA | Procesos como "APO13" (gestión de seguridad), "DSS05". |
| ISACA | Asociación internacional de auditoría y gobierno de TI | Autora de COBIT; de ahí el nombre de las plantillas `plantilla/isaca/`. |
| NIST | Instituto de estándares de EE.UU. | SP 800-207 es su publicación de referencia sobre Zero Trust. |
| SoA | *Statement of Applicability* (Declaración de Aplicabilidad) | Documento ISO 27001: qué controles aplican, cuáles no, y por qué. |
| RACI | Responsable / Aprueba / Consultado / Informado | Matriz de responsabilidades por proceso. |
| RF / RNF | Requerimiento Funcional / Requerimiento No Funcional | Numeración de la letra (RF-01, RNF-01...). |
| KPI | *Key Performance Indicator* | Indicador que mide qué tan bien funciona algo (ej. MTTD). |

## Criptografía y autenticación

| Sigla | Significado | Para qué sirve acá |
|---|---|---|
| KDF | *Key Derivation Function* | Convierte la contraseña maestra en una clave criptográfica (Argon2id, bcrypt, scrypt). |
| Argon2id / bcrypt | Algoritmos KDF "memory-hard" | Lentos y caros de atacar por fuerza bruta; elegibles desde el control central (RF-11). |
| AEAD | *Authenticated Encryption with Associated Data* | Cifrado que además garantiza que el dato no fue alterado (XChaCha20-Poly1305, AES-256-GCM). |
| JWS | *JSON Web Signature* | Firma digital de cada evento de auditoría; permite verificar origen e integridad. |
| HMAC | *Hash-based Message Authentication Code* | Alternativa a JWS con clave compartida en vez de par público/privado. |
| TOTP | *Time-based One-Time Password* (RFC 6238) | El código de 6 dígitos que cambia cada 30s. |
| WebAuthn / FIDO2 / U2F | Estándares de autenticación sin contraseña | Permiten usar Windows Hello como "authenticator" de plataforma. |
| MFA | *Multi-Factor Authentication* | Pedir más de un factor para autenticarse. |
| RBAC | *Role-Based Access Control* | Permisos según el rol del usuario del panel de control central. |
| TLS | *Transport Layer Security* | El protocolo que cifra la conexión ("https"). |

## Detección y respuesta

| Sigla | Significado | Para qué sirve acá |
|---|---|---|
| SIEM | *Security Information and Event Management* | Correlaciona eventos de seguridad (Wazuh, en nuestro caso). |
| HIDS | *Host-based Intrusion Detection System* | Detección de intrusos en un host puntual (archivos, procesos). |
| NIDS | *Network Intrusion Detection System* | Detección mirando tráfico de red (Suricata/Zeek — parte de Security Onion, que no vamos a usar). |
| FIM | *File Integrity Monitoring* | Vigila si un archivo cambió sin autorización (el archivo de bóveda, por ejemplo). |
| SOAR | *Security Orchestration, Automation and Response* | Automatiza/organiza la respuesta a incidentes (TheHive). |
| MTTD / MTTR | *Mean Time To Detect* / *Mean Time To Respond* | Cuánto tarda en detectarse / resolverse un incidente. |
| RTO / RPO | *Recovery Time Objective* / *Recovery Point Objective* | Tiempo máximo caído / datos máximos perdidos en una recuperación. |

## Ataques y hallazgos (relevantes para el Red Team)

| Sigla | Significado | Para qué sirve acá |
|---|---|---|
| IDOR | *Insecure Direct Object Reference* | Acceder a datos de otro usuario cambiando un ID en la petición. |
| XSS | *Cross-Site Scripting* | Inyectar JavaScript que se ejecuta en el navegador de otra persona (ej. en el dashboard). |
| SQLi | *SQL Injection* | Inyectar SQL a través de un campo mal validado. |
| MITM | *Man-in-the-Middle* | Interceptar/alterar la comunicación entre cliente y control central. |
| DoS | *Denial of Service* | Saturar un sistema para que deje de responder. |
| OSINT | *Open Source Intelligence* | Recolectar info pública (ej. revisar el repo en busca de secretos commiteados). |
| CVSS | *Common Vulnerability Scoring System* | Escala 0-10 de severidad de una vulnerabilidad. |
| MITRE ATT&CK | Catálogo de tácticas y técnicas de ataque | Usado para clasificar los hallazgos del Red Team. |
| SPF / DKIM / DMARC | Mecanismos de autenticación de correo | Evitan que se falsifique el dominio de correo del control central. |

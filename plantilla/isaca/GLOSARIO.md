# Glosario ISACA / COBIT 2019 / NIST (de referencia para las tareas)

Glosario de términos utilizados en las plantillas de documentación de seguridad.

| Término | Definición |
|---|---|
| **Activo** | Todo aquello que tiene valor para la organización y que debe protegerse: datos, hardware, software, servicios, personas, reputación. (COBIT/ISO) |
| **Activo crítico** | Activo cuya pérdida o compromiso impacta de forma grave a la operación del negocio. |
| **CIA (tríada)** | Confidencialidad, Integridad y Disponibilidad. Objetivos base de la seguridad de la información. |
| **Control** | Práctica, procedimiento o mecanismo que mitiga un riesgo. (ISO/IEC 27001) |
| **Riesgo** | Efecto de la incertidumbre sobre los objetivos; en seguridad, probabilidad × impacto de una amenaza explotando una vulnerabilidad. (ISO 31000) |
| **Tratamiento del riesgo** | Decisión y acción sobre el riesgo: mitigar, transferir, retener o evitar. (ISO 31000 / MCU 5.0) |
| **Riesgo residual** | Riesgo que permanece luego de aplicar los controles. Debe ser aceptado formalmente. |
| **Vulnerabilidad** | Debilidad que puede ser explotada por una amenaza. |
| **Amenaza** | Causa potencial de un incidente (persona, evento, fenómeno). |
| **Incidente** | Evento que compromete la CIA de la información o viola una política de seguridad. |
| **Evento de seguridad** | Ocurrencia observable en un sistema; solo si impacta genera incidente. |
| **TOTP** | *Time-based One-Time Password* (RFC 6238): clave de un solo uso generada por tiempo, para segundo factor. |
| **MFA / 2FA** | Autenticación multifactor / de dos factores. |
| **WebAuthn / U2F** | FIDO: autenticación por llaves físicas / plataforma sin contraseña. |
| **Windows Hello** | Sistema de autenticación biométrica/PIN del SO Windows. |
| **Argon2id / bcrypt** | Funciones de derivación de claves (KDF) con salt, recomendadas para almacenar contraseñas. |
| **SIEM** | *Security Information and Event Management*: correlaciona y almacena logs para detección. |
| **SOAR** | *Security Orchestration, Automation and Response*: automatiza la respuesta. |
| **HIDS / NIDS** | Sistema de detección de intrusos basado en host / en red. |
| **Zero Trust** | Modelo de seguridad "no confiar nunca, verificar siempre" (NIST SP 800-207). |
| **Seguridad por capas (defense in depth)** | Múltiples capas de control para reducir la probabilidad de compromiso. |
| **Declaración de aplicabilidad (SoA)** | Documento ISO 27001 que describe qué controles del Anexo A aplican, cuáles no y su justificación. |
| **RTO / RPO** | *Recovery Time Objective* (tiempo objetivo de recuperación) / *Recovery Point Objective* (pérdida de datos máxima admisible). |
| **MCU 5.0** | Marco de Ciberseguridad del Uruguay, versión 5.0, de AGESIC. Ejes: Gobernar, Identificar, Proteger, Detectar, Responder, Recuperar. |
| **GSI BCU** | Guía de Seguridad de la Información del Banco Central del Uruguay: requerimientos mínimos para el sistema financiero. |
| **URCDP** | Unidad Reguladora y de Control de Datos Personales (Uruguay). Vigila la Ley 18.331. |
| **COBIT 2019** | Marco de gobernanza y gestión de TI de ISACA (dominios EDM, APO, BAI, DSS, MEA). |
| **ISACA** | Organización internacional de auditoría y gobierno de TI; autor de COBIT, Risk IT y CISA/CISM. |
| **KPI / KRI** | Indicador clave de desempeño / indicador clave de riesgo. |
| **KV / BD tipo clave-valor** | Base tipo almacén de credenciales (por ejemplo, para gestor de contraseñas). |
| **KDF** | *Key Derivation Function:* función que deriva una clave de una contraseña (+salt) (Argon2, Pbkdf2). |
| **Perfect Forward Secrecy** | Propiedad criptográfica por la que no se puede descifrar tráfico pasado aunque se comprometa la clave a largo plazo. |
| **OSINT** | *Open Source Intelligence*: información de fuentes públicas (usada en pruebas Red Team). |
| **MITRE ATT&CK** | Base de conocimiento de tácticas y técnicas de adversarios (usada para modelar ataques). |
| **CVSS** | *Common Vulnerability Scoring System*: métrica de severidad de vulnerabilidades (0-10). |
| **FIT / STIX/TAXII** | Formatos de intercambio de inteligencia de amenazas. |
| **Honey Pot** | Recurso trampa para detectar/desviar ataques. |
| **Wazuh** | Suite open source HIDS + SIEM + SOAR (agentes OSSEC). |
| **Security Onion** | Distribución Linux open source para monitoreo de seguridad (Suricata, Zeek, Elastic, TheHive). |
| **Suricata** | NIDS/IPS open source de alto rendimiento (signatures y detección de anomalías). |
| **Zeek (Bro)** | Analizador de tráfico de red orientado a metadatos. |
| **TheHive / Cortex** | Plataforma open source de gestión de casos (SOAR con Cortex para respuesta automatizada). |
| **Wazo Platform** | Plataforma de comunicaciones open source (PBX/IP-PBX) basada en Asterisk. |
| **OpenVAS/Greenbone** | Escáner de vulnerabilidades open source. |
| **Nuclei / Nikto** | Escáneres de vulnerabilidades web. |
| **Gitea/Forgejo** | Forja Git open source (control de versiones). |
| **PostgreSQL** | Gestor de base de datos relacional open source. |
| **Hemmelig / Nitrokey / KeePassXC** | Herramientas de referencia relacionadas con gestión de secretos/contraseñas. |
| **Contraseña maestra** | Contraseña que cifra el conjunto de credenciales (memoria maestra) en un gestor de contraseñas. |
| **Salt** | Valor aleatorio único añadido antes del hash para evitar tablas rainbow. |
| **Sweet32 / SMBv1** | Referencias a protocolos vulnerables; nombres de casos de debilitamiento criptográfico. |
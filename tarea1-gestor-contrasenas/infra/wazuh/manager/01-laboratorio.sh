#!/bin/bash
# Sin indexer, Filebeat sale y s6 apaga el manager. Lo dejamos en espera.
cat > /etc/services.d/filebeat/run << 'EOF'
#!/bin/sh
echo "Filebeat en espera: este laboratorio no usa el indexer. Las alertas quedan en alerts.json."
exec sleep infinity
EOF
chmod 755 /etc/services.d/filebeat/run

if [ -f /var/ossec/etc/rules/local_rules.xml ]; then
  chown root:wazuh /var/ossec/etc/rules/local_rules.xml
  chmod 640 /var/ossec/etc/rules/local_rules.xml
fi

mkdir -p /var/ossec/logs/alerts
chown -R wazuh:wazuh /var/ossec/logs/alerts
chmod 750 /var/ossec/logs/alerts

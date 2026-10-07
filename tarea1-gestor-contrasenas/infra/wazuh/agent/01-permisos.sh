#!/bin/bash
if [ -f /var/ossec/etc/ossec.conf ]; then
  chown root:wazuh /var/ossec/etc/ossec.conf
  chmod 640 /var/ossec/etc/ossec.conf
fi

#!/bin/bash
CURRENT_DIR=$(pwd)
sed -e "s|{{dir}}|${CURRENT_DIR}/www/|g" -e "s|{{user}}|${USER}|g" ./etc/nginx.conf_tmpl > ./etc/nginx.conf
sudo nginx -c ${CURRENT_DIR}/etc/nginx.conf

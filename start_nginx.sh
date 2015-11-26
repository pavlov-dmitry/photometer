#!/bin/bash
# quiting nginx master process
pid=$(ps ax | grep "nginx: master" | head -n1 | awk '{print $1}')
sudo kill -s QUIT $pid

# making config for nginx
CURRENT_DIR=$(pwd)
sed -e "s|{{dir}}|${CURRENT_DIR}/www/|g" -e "s|{{user}}|${USER}|g" ./etc/nginx.conf_tmpl > ./etc/nginx.conf
# starting nginx
sudo nginx -c ${CURRENT_DIR}/etc/nginx.conf

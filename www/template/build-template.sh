#!/usr/bin/env sh
# debian 7 version
handlebars $1 --extension=hbs --amd --output ${1%.*}.js
# ubuntu 14.04 version
# nodejs /usr/local/bin/handlebars $1 --extension=hbs --amd --output ${1%.*}.js

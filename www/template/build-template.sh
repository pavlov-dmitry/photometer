#!/usr/bin/env sh
nodejs /usr/local/bin/handlebars $1 --extension=hbs --amd --output ${1%.*}.js
#!/bin/sh
set -ex
curl -L http://downloads.sourceforge.net/project/libexif/libexif/0.6.21/libexif-0.6.21.zip > libexif.zip
unzip libexif.zip -d libexif
rm -v libexif.zip
cd libexif/libexif-0.6.21 && ./configure --libdir=/usr/lib/x86_64-linux-gnu --enable-shared && make CFLAGS=-fPIC && sudo make install
cd ../../ && rm -r -f -v libexif
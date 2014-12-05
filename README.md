photometer
==========

Сборка [![Build Status](https://travis-ci.org/pavlov-dmitry/photometer.png?branch=master)](https://travis-ci.org/pavlov-dmitry/photometer)
==========

Собирается под Ubuntu(linux).
Зависим от `libssl-dev` и `libexif-dev`.
В 64х битной версии Ubuntu `libexif` собран без флага ["-fPIC"](http://en.wikipedia.org/wiki/Position-independent_code) из-за чего rust не хочет её линковать.
Собрать `libexif` пригодную для линковки можно при помощи `build_libexif_64.sh`
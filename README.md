photometer
==========

Сборка [![Build Status](https://travis-ci.org/pavlov-dmitry/photometer.png?branch=master)](https://travis-ci.org/pavlov-dmitry/photometer)
==========

## back-end ([Rust](http://www.rust-lang.org/))

Собирается под Ubuntu(linux).
Зависим от `libssl-dev` и `libexif-dev`.
В 64х битной версии Ubuntu `libexif` собран без флага ["-fPIC"](http://en.wikipedia.org/wiki/Position-independent_code) из-за чего rust не хочет её линковать.
Собрать `libexif` пригодную для линковки можно при помощи `build_libexif_64.sh`

Для сборки выполнить ```cargo build```

## front-end ([JS](https://ru.wikipedia.org/wiki/JavaScript))

Для сборки front-end необходим [node.js](https://nodejs.org/). [npm](https://www.npmjs.com/) необходим для установки "компилятора" шаблонов [Handlebars](http://handlebarsjs.com/)

Для Windows достаточно просто установит дистрибутив с сайта [node.js](https://nodejs.org/) [npm](https://www.npmjs.com/) входит в его состав.

Для установки на Ubuntu, пара команд:
```
sudo apt-get install nodejs
sudo apt-get install npm
```

Для сборки шаблонов [Handlebars](http://handlebarsjs.com/), нужно его предустановить при помощи [npm](https://www.npmjs.com/)
```
npm install handlebars -g
```

В папке `tools` есть скрипт для полной сборки front-end части под Windows/Ubuntu.

В папке `www/template/` есть скрипты для сборки шаблонов [Handlebars](http://handlebarsjs.com/)
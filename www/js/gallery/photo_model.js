define( function(require) {
    var Backbone = require( "lib/backbone" );

    var PhotoModel = Backbone.Model.extend({
        defaults: {
            'id': 0,
            'user_id': 0,
            // Какого-то хера эта фигня не устанваливается, видимо по
            // умолчанию можно ставить только те поля что
            // предусмотрены Хребтом
            'photo_url': "gallery/photo_info",
            'name': "нет имени"
        },

        fetch: function() {
            var self = this;
            var Request = require( "request" );
            var handler = Request.get( this.photo_url, { photo: this.id, user: this.user_id } );
            handler.good = function( data ) {
                self.set( data.photo );
            }

            handler.bad = function( data ) {
                var errorsHandler = require( "errors_handler" );
                var header = "Ошибка получения информации о фотографии";
                var text = "";
                if ( data.reason == "not_found" ) {
                    text = "Фотография с идентификатором " + this.id + " не найдена.";
                }
                else {
                    text = JSON.stringify( data );
                }
                errorsHandler.error( header, text );
            }
        }
    })



    return PhotoModel;
})

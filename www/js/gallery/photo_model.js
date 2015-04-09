define( function(require) {
    var Backbone = require( "lib/backbone" );

    var PhotoModel = Backbone.Model.extend({
        defaults: {
            'id': 0,
            'name': "нет имени"
        },

        fetch: function() {
            var self = this;
            var Request = require( "request" );
            var handler = Request.get( /photo_info/ + this.id );
            handler.good = function( data ) {
                self.set( data );
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

define( function(require) {
    var Backbone = require( "lib/backbone" );

    var PhotoModel = Backbone.Model.extend({
        defaults: {
            'id': 0,
        },

        fetch: function() {
            var self = this;
            var Request = require( "request" );
            var handler = Request.get( this.photo_url, this.photo_data );
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

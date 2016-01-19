define( function(require) {
    var Backbone = require( "lib/backbone" ),
    PhotoModel = require( "gallery/photo_model" ),
    Request = require( "request" );

    var GalleryCollection = Backbone.Collection.extend({
        model: PhotoModel,

        fetch: function( page ) {
            var self = this;
            var handler = Request.get( "/gallery", { page: page } );
            handler.good = function( data ) {
                self.reset();
                self.add( data.photos );
                self.trigger( "pages_changed", data.pagination );
                // сцуко, то само вызывалось, то млин теперь самому приходится дёргать, что-то не пойму.
                self.trigger( "update" );
            };

            handler.bad = function( data ) {
                var errorHandler = require( "errors_handler" );
                errorHandler.oops( "Не смог загрузить галлерею", JSON.stringify( data ) );
            }
        }
    });

    return GalleryCollection;
})

define( function(require) {
    var Backbone = require( "lib/backbone" ),
    PhotoModel = require( "gallery/photo_model" ),
    Request = require( "request" );

    var GalleryCollection = Backbone.Collection.extend({
        model: PhotoModel,

        fetch: function() {
            var self = this;
            var handler = Request.get( "/gallery", { page: 0 } );
            handler.good = function( data ) {
                self.reset();
                self.add( data );
            };

            handler.bad = function( data ) {
                var errorHandler = require( "errors_handler" );
                errorHandler.oops( "Не смог загрузить галлерею", JSON.stringify( data ) );
            }
        }
    });

    return GalleryCollection;
})

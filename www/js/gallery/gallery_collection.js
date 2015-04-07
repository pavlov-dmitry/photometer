define( function(require) {
    var Backbone = require( "lib/backbone" ),
    PhotoModel = require( "gallery/photo_model" ),
    Request = require( "request" );

    var GalleryCollection = Backbone.Collection.extend({
        model: PhotoModel,

        fetch: function() {
            var handler = Request.get( "/gellery", {} );
            handler.good = function( data ) {
                this.reset();
                this.add( data );
            };

            handler.bad = function( data ) {
                require( ['app'], function( app ) {
                    app.oops( "Не смог загрузить галлерею", JSON.stringify( data ) );
                } );
            }
        }
    });

    return GalleryCollection;
})

define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Request = require( "request" );

    var GalleryModel = Backbone.Model.extend({
        defaults: {
            is_own_gallery: false,
            owner: {},
            photos: [],
            pagination: {}
        },

        fetch: function( page ) {
            var self = this;
            var owner = self.get("owner");
            var handler = Request.get( "/gallery", {
                user_id: owner.id,
                page: page
            });
            handler.good = function( data ) {
                data.prefix_url = "#gallery/" + data.owner.id + "/";
                self.set( data );
            };

            handler.bad = function( data ) {
                var errorHandler = require( "errors_handler" );
                errorHandler.oops( "Не смог загрузить галлерею", JSON.stringify( data ) );
            }
        }
    });

    return GalleryModel;
})

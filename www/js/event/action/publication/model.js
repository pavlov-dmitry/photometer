define( function(require) {
    var Backbone = require( "lib/backbone" ),
        request = require( "request" );

    var PublicationModel = Backbone.Model.extend({
        defaults: {
            id: 0
        },

        fetch: function( page ) {
            var handler = request.get( "gallery/unpublished", { page: page } );
            var self = this;
            handler.good = function( data ) {
                self.set( data );
            }
        },

        save: function( photo_id ) {
            var publication_id = this.get( "id" );
            var url = "/event/" + publication_id;
            return request.post( url, {id: photo_id} );
        }
    });
    return PublicationModel;
});

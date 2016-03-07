define ( function( require ) {
    var Backbone = require( "lib/backbone" ),
        request = require( "request" );

    var UserDescriptionModel = Backbone.Model.extend({
        defaults: {
            id: 0,
            name: "",
            groups: []
        },

        fetch: function() {
            var url = "/user/" + this.get( "id" );
            var handler = request.get( url, {} );
            var self = this;
            handler.good = function( data ) {
                self.set( data );
            }
            return handler;
        }
    });

    return UserDescriptionModel;
})

define( function( require ) {
    var Backbone = require( "lib/backbone" );

    var UserInviteModel = Backbone.Model.extend({
        defaults: {
            group_id: 0,
            id: 0,
            name: 0
        },

        fetch: function() {
            var request = require( "request" );
            var url = "/events/group/" + this.get( "group_id" ) + "/create/user_invite";
            var handler = request.get( url, {} );
            var self = this;
            handler.good = function( data ) {
                self.set( data );
            };
            return handler;
        }
    });

    return UserInviteModel;
});

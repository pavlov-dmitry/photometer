define( function( require ) {
    var Backbone = require( "lib/backbone" ),
        request = require( "request" );

    var UserInviteModel = Backbone.Model.extend({
        defaults: {
            group_id: 0,
            id: 0,
            name: "",
            description: "",
            user_id: 0
        },

        fetch: function() {
            var handler = request.get( this.make_url(), {} );
            var self = this;
            handler.good = function( data ) {
                self.set( data );
            };
            return handler;
        },

        save: function() {
            var handler = request.post( this.make_url(), {
                user_id: this.get("user_id"),
                text: this.get("description")
            });
            return handler;
        },

        make_url: function() {
            return "/events/group/" + this.get( "group_id" ) + "/create/user_invite";
        }
    });

    return UserInviteModel;
});

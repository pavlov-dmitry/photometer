define( function( require ) {
    var Backbone = require( "lib/backbone" ),
        UsersCollection = require( "group_creation/users_collection" )

    var GroupCreationModel = Backbone.Model.extend({
        defaults: {
            name: "",
            description: "",
            members: new UsersCollection()
        },

        urlRoot: "/events/create/2",

        save: function() {
            var request = require( "request" );
            return request.post( urlRoot, this.toJSON() );
        },

        add_new_member: function() {
            var members = this.get( "members" );
            members.add_one_more();
        }
    });

    return GroupCreationModel;
})

define ( function( require ) {
    var Backbone = require( "lib/backbone" ),
        UserModel = require( "group_creation/user_model" );

    var UsersCollection = Backbone.Collection.extend({

        model: UserModel,

        add_one_more: function() {
            this.add( UserModel.empty_model() );
        },

        add_user: function( data ) {
            this.add( new UserModel( data ) );
        }

    });

    return UsersCollection;
})

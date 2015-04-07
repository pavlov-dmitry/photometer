define( function( require ) {
    var Backbone = require( "lib/backbone" );

    var UserStateModel = Backbone.Model.extend({
        defaults: {
            isLogged: false,
            name: "",
            unreadedMessaged: 0
        },

        logged: function( name ) {
            this.set( {
                isLogged: true,
                name: name
            } );
        },

        logout: function() {
            this.set({
                isLogged: false,
                name: ""
            });
        }
    });

    return UserStateModel;
});

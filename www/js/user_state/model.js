define( function( require ) {
    var Backbone = require( "lib/backbone" );

    var UserStateModel = Backbone.Model.extend({
        defaults: {
            isLogged: false,
            name: "",
            unreadedMessaged: 0,
            isNavInGallery: false,
            isNavInMessages: false
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
        },

        //TODO: Является местополжение в навигации состоянием
        //пользователя ?, или стоит вынести это в отдельную модель
        navToGallery: function() {
            var navState = this.getResetedNav();
            navState.isNavInGallery = true;
            this.set( navState );
        },

        navToMessages: function() {
            var navState = this.getResetedNav();
            navState.isNavInMessages = true;
            this.set( navState );
        },

        getResetedNav: function() {
            return {
                isNavInMessages: false,
                isNavInGallery: false
            };
        }
    });

    return UserStateModel;
});

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

        fetch: function() {
            var self = this;
            var request = require( "request" );
            var handler = request.get( "user_info" );
            handler.good = function( data ) {
                self.set({
                    isLogged: true,
                    name: data.name
                });
            }

            handler.bad = function( data ) {
                self.set({
                    isLogged: false,
                    name: ""
                });
            }
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

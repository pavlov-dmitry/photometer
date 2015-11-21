define( function( require ) {
    var Backbone = require( "lib/backbone" );

    var UserStateModel = Backbone.Model.extend({
        defaults: {
            isLogged: false,
            name: "",
            id: 0,
            unreaded_messages: 0,
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
                    name: data.name,
                    unreaded_messages: data.unreaded_messages_count,
                    id: data.id
                });
            }

            handler.unauthorized = function( data ) {
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

        resetNav: function() {
            var navState = this.getResetedNav();
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

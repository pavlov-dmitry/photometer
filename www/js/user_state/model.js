define( function( require ) {
    var Backbone = require( "lib/backbone" );

    var UserStateModel = Backbone.Model.extend({
        defaults: {
            isLogged: false,
            name: "",
            id: 0,
            unreaded_messages: 0,
            isNavInGallery: false,
            isNavInMessages: false,
            isNavInGroup: false,
            is_groups: false,
            is_many_groups: false,
            has_unreaded_in_groups: false,
            current_group: {}
        },

        fetch: function() {
            var self = this;
            var request = require( "request" );
            var handler = request.get( "user_info" );
            handler.good = function( data ) {
                var model_data = {
                    isLogged: true,
                    name: data.name,
                    unreaded_messages: data.unreaded_messages_count,
                    id: data.id,
                    groups: data.groups
                };
                if ( 0 < model_data.groups.length ) {
                    model_data.is_groups = true;
                    model_data.current_group = model_data.groups[ 0 ];
                }
                if ( 1 < model_data.groups.length ) {
                    model_data.is_many_groups = true;
                    model_data.current_group = _.max(
                        model_data.groups,
                        function( x ) {return x.unwatched_events;}
                    );
                }
                model_data.has_unreaded_in_groups = _.some( model_data.groups, function( x ) {
                    return x.unwatched_events != 0;
                });
                self.set( model_data );
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

        navToGroup: function() {
            var navState = this.getResetedNav();
            navState.isNavInGroup = true;
            this.set( navState );
        },

        resetNav: function() {
            var navState = this.getResetedNav();
            this.set( navState );
        },

        getResetedNav: function() {
            return {
                isNavInMessages: false,
                isNavInGallery: false,
                isNavInGroup: false
            };
        },

        set_current_group: function( id ) {
            var data = this.toJSON();
            data.current_group = _.find( data.groups, function( x ) { return x.id == id; } );
            this.set( data );
        }
    });

    return UserStateModel;
});

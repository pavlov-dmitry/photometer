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
            has_unreaded_in_other_groups: false,
            current_group: null,
            current_group_id: 0
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
                    groups: data.groups,
                    is_groups: false,
                    current_group: null,
                    current_group_id: self.get( "current_group_id" ),
                    is_many_groups: false,
                    has_unreaded_in_groups: false
                };
                if ( 0 < model_data.groups.length ) {
                    model_data.is_groups = true;
                    model_data.current_group = model_data.groups[ 0 ];
                }
                if ( 1 < model_data.groups.length ) {
                    model_data.is_many_groups = true;
                    if ( model_data.current_group_id == 0 ) {
                        self.choose_current_group_in( model_data );
                        model_data.current_group_id = model_data.current_group.id;
                    }
                    self.set_current_group_in( model_data, model_data.current_group_id );
                }
                var all_unreaded_count = _.reduce(
                    model_data.groups,
                    function( memo, group ) {
                        return memo += group.unwatched_events;
                    },
                    0
                );
                if ( model_data.current_group ) {
                    model_data.has_unreaded_in_other_groups = model_data.current_group.unwatched_events < all_unreaded_count;
                }
                self.set( model_data );

                if ( self.after_fetch ) {
                    self.after_fetch();
                    self.after_fetch = null;
                }
            }

            handler.unauthorized = function( data ) {
                self.set({
                    isLogged: false,
                    name: ""
                });
                app.unauthorized();
            }
        },

        logout: function() {
            this.set({
                isLogged: false,
                name: ""
            });
        },

        is_logged_in: function() {
            return this.get("isLogged");
        },

        user_id: function() {
            return this.get("id");
        },

        // TODO: подумать чтоб убрать этот галимый костыль из-за того что я разделил
        // информацию о пользователе и инфу о странице на два запроса
        on_ready: function( f ) {
            if ( this.user_id() !== 0 ) {
                f();
            }
            else {
                this.after_fetch = f;
            }
        },

        //TODO: Является местополжение в навигации состоянием
        //пользователя ?, или стоит вынести это в отдельную модель
        navToGallery: function( user_id ) {
            if ( user_id === this.get("id") ) {
                var navState = this.getResetedNav();
                navState.isNavInGallery = true;
                this.set( navState );
            }
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

        set_current_group_in: function( data, id ) {
            data.current_group = _.find( data.groups, function( x ) { return x.id == id; } );
            data.current_group_id = id;
        },

        choose_current_group_in: function( data ) {
            data.current_group = _.max(
                data.groups,
                function( x ) {return x.unwatched_events;}
            );
        },

        set_current_group: function( id ) {
            var data = this.toJSON();
            this.set_current_group_in( data, id );
            this.set( data );
        },

        get_current_group_id: function() {
            var group = this.get("current_group");
            if ( group != null )
            {
                return group.id;
            }
            return 0;
        }
    });

    return UserStateModel;
});

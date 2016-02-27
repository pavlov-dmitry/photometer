define( function( require ) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" ),
        errors_handler = require( "errors_handler" );
    require( "template/user_invite" );

    var UserInviteView = Backbone.View.extend({
        el: $( "#workspace" ),
        template: Handlebars.templates.user_invite,

        events: {
        },

        initialize: function() {
            this.listenTo( this.model, "change", this.render );

            var handler = this.model.fetch();
            handler.not_found = this.not_found;
        },

        close: function() {

        },

        render: function() {
            this.$el.html( this.template( this.model.toJSON() ) );
            $("#user").dropdown({
                fields: {
                    value: "name"
                },
                apiSettings: {
                    mockResponseAsync: function( settings, callback ) {
                        var response = {
                            success: true,
                            message: "",
                            results: {}
                        };
                        var request = require( "request" );
                        var handler = request.get(
                            "search/users",
                            { like: settings.urlData.query }
                        );
                        handler.good = function( data ) {
                            response.results = data.users;
                            callback( response );
                        }
                        handler.bad = function( data ) {
                            response.success = false;
                            response.message = "ERROR:" + JSON.stringify( data );
                            callback( response );
                        }
                    }
                }
            });
        },

        not_found: function() {
            errors_handler.oops(
                "Ошибка запроса информации о группе",
                "Что-то группа которую вы запрашиваете, не найдена, то-ли что-то пошло не так, то-ли кто-то что-то не то запрашивает. В общем фиг его знает, если вы не мудрили с адресной строкой, то похоже стоит обратиться к разработчикам."
            )
        }
    });

    return UserInviteView;
});

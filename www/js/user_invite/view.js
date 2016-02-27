define( function( require ) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" ),
        errors_handler = require( "errors_handler" );
    require( "template/user_invite" );

    var UserInviteView = Backbone.View.extend({
        el: $( "#workspace" ),
        template: Handlebars.templates.user_invite,

        events: {
            "change #description-input": "description_changed",
            "keyup #description-input": "description_changed",
            "submit #form-user-invite": "submit",
        },

        initialize: function() {
            this.listenTo( this.model, "change:name", this.render );

            var handler = this.model.fetch();
            handler.not_found = this.not_found;
        },

        close: function() {

        },

        render: function() {
            this.$el.html( this.template( this.model.toJSON() ) );
            $("#user").dropdown({
                fields: {
                    name: "name",
                    value: "id"
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

        description_changed: function() {
            markdown = require( 'showdown_converter' );

            var description = $( "#description-input" ).val();
            this.model.set({ description: description });
            var desc_html = markdown.makeHtml( description );

            $( "#info-preview").html( desc_html );
        },

        submit: function() {
            $("#form-errors").addClass( "hidden" );

            var user_id = $("#user").val();
            this.model.set({ user_id: user_id });
            var handler = this.model.save();
            $("#form-invite-user").addClass( "loading" );
            var self = this;
            handler.good = function() {
                var growl = require( "growl" );
                growl({
                    positive: true,
                    header: "Приглашение выслано",
                    msg: "Приглшение пользователю выслано. Вам выслано письмо с ссылкой по которой вы можете отслеживать прогресс приглашения."
                }, "long");
                var app = require( "app" );
                app.userState.fetch();
                app.navMessages();
            }
            handler.bad = function( data ) {
                self.show_errors( data );
            }
            handler.finish = function() {
                $("#form-invite-user").removeClass( "loading" );
            }
            handler.not_found = this.not_found;
        },

        show_errors: function( data ) {
            $("#errors-list").empty();
            var msg = "Неизвестная ошибка";
            if ( data.field == "text" && data.reason == "empty" ) {
                msg = "Нужно какое-то объясниение, почему вы решили пригласить пользователя.";
            }
            if ( data.field == "text" && data.reason == "too_long" ) {
                msg = "Слишком длинный текст описания. Попробуйте быть чуть менее многословными.";
            }
            if ( data.field == "user" && data.reason == "not_found" ) {
                msg = "Выбранный пользователь не найден.";
            }
            if ( data.field == "user" && data.reason == "in_group" ) {
                msg = "Выбранный пользователь уже является членом данной группы.";
            }
            var newError = "<li>" + msg + "</li>";
            $("#errors-list").append( $(newError) );
            $("#form-errors").removeClass( "hidden" );
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

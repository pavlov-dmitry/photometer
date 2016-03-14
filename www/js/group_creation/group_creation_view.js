define( function( require ) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" ),
        UserView = require( "group_creation/user_view" ),
        group_creation_template = require( "template/group_creation" ),
        success_tmpl = require( "template/success_info" );

    var GroupCreationView = Backbone.View.extend({

        el: $( "#workspace" ),

        template: Handlebars.templates.group_creation,

        events: {
            "change #description-input": "description_changed",
            "keyup #description-input": "description_changed",
            "keyup #name-input": "description_changed",
            "change #name-input": "name_changed",
            "submit #form-group-creation": "submit"
        },

        initialize: function() {
            this.render();
        },

        close: function() {
            var members = this.model.get( "members" );
            members.reset();
        },

        render: function() {
            this.$el.html( this.template( this.model.toJSON() ) );
            $(".markdown").popup({
                hoverable: true,
                popup: "#markdown-help",
                lastResort: true
            });
            $("#users").dropdown({
                fields: {
                    value: "id"
                },
                apiSettings: {
                    // cache: false,
                    mockResponseAsync: function( settings, callback ) {
                        // console.log( "mock async: " + JSON.stringify( settings ) );
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
                            // console.log( "search result: " + JSON.stringify( response ) );
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

            var group_name = $( "#name-input" ).val();
            var description = $( "#description-input" ).val();

            this.model.set({ description: description });

            var desc_html = markdown.makeHtml( description );
            var all_html = "<h1>" + group_name + "</h1>" + desc_html;

            $( "#info-preview").html( all_html );
        },

        name_changed: function() {
            this.model.set({ name: $("#name-input").val() });
        },

        submit: function() {
            $("#form-group-creation").addClass( "loading" );
            var self = this;
            var members = this.model.get( "members" );
            members.reset();
            var users_value =  $("#users").val();
            _.forEach( users_value, function( data ) {
                members.add_user( { id: data } );
            })
            var handler = this.model.save();

            handler.good = function() {
                console.log( "group_created" );
                var success_template = Handlebars.templates.success_info;
                var success_html = success_template({
                    caption: "Группа создана.",
                    text: "Проверьте свои сообщения за дальнейшими инструкциями."
                });
                self.$el.html( success_html );

                // обновляем состояние, чтоб увидеть что пришло новое письмо
                var app = require( "app" );
                app.userState.fetch();
            };

            handler.bad = function( data ) {
                console.log( "group creation failed: " + JSON.stringify( data ) );
                self.show_errors( data );
            }

            handler.finish = function() {
                $("#form-group-creation").removeClass( "loading" );
            }
        },

        show_errors: function( data ) {
            var NAME = "name";
            var DESCRIPTION = "description";
            var EMPTY = "empty";
            var TOO_LONG = "too_long";
            var NOT_FOUND = "not_found";
            $("#errors-list").empty();

            _.forEach( data, function( desc ) {
                var text_error = "Unknown error.";

                if ( desc.field === "members" && desc.reason === NOT_FOUND ) {
                    text_error = "Может быть пригласим кого-нибудь в эту группу ?";
                }
                else if ( desc.field === NAME && desc.reason === EMPTY ) {
                    text_error = "Имя группы не может быть пустым.";
                }
                else if ( desc.field === DESCRIPTION && desc.reason === EMPTY ) {
                    text_error = "Описание группы не может быть пустым.";
                }
                else if ( desc.field === NAME && desc.reason === TOO_LONG ) {
                    text_error = "Имя группы не может превышать 64-х символов.";
                }
                else if ( desc.field === DESCRIPTION && desc.reason === TOO_LONG ) {
                    text_error = "Описание группы не может превышать 2048-х символов.";
                }
                else if ( desc.field === "group" && desc.reason === "exists" ) {
                    text_error = "Группа с таким именем уже существует";
                }
                else if ( desc.reason === NOT_FOUND ) {
                    text_error = "Пользователь с ID " + desc.field + " не найден";
                }

                var newError = "<li>" + text_error + "</li>";
                $("#errors-list").append( $(newError) );
            });
            $("#form-errors").removeClass( "hidden" );
        },
    });

    return GroupCreationView;
})

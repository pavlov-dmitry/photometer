define( function( require ) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" ),
        UserView = require( "group_creation/user_view" ),
        group_creation_template = require( "template/group_creation" ),
        closable_error_templ = require( "template/closeable_error"),
        success_tmpl = require( "template/success_info" );

    var GroupCreationView = Backbone.View.extend({

        el: $( "#workspace" ),

        template: Handlebars.templates.group_creation,

        events: {
            "change #description-input": "description_changed",
            "keyup #description-input": "description_changed",
            "keyup #name-input": "description_changed",
            "click #add-member-btn": "add_user_clicked",
            "change #name-input": "name_changed",
            "submit #form-group-creation": "submit"
        },

        initialize: function() {
            // var members = this.model.get( "members" );
            // this.listenTo( members, "add", this.user_added );
            // this.listenTo( members, "remove", this.check_users_for_remove );

            this.render();
        },

        close: function() {
            var members = this.model.get( "members" );
            members.off( null, this.user_added );
            members.off( null, this.check_users_for_remove );
            members.reset();
        },

        render: function() {
            this.$el.html( this.template( this.model.toJSON() ) );
            $("#users").dropdown({
                apiSettings: {
                    // debug: true,
                    // url: "/search",
                    // cache: false,
                    mockResponseAsync: function( settings, callback ) {
                        console.log( "mock async: " + JSON.stringify( settings ) );
                        var response = {
                            success: true,
                            results: {

                            }
                        };
                        window.setTimeout( function() {
                            callback( response );
                        }, 3000);
                    },
                    successTest: function( response ) {
                        console.log( "successTest" );
                        return true;
                    },
                    onFailure: function( response ) {
                        console.log( "onFailure" );
                    },
                    onResponse: function( response ) {
                        console.log( "onResponse" );
                        return response;
                    }
                }
            });
        },

        add_user_clicked: function() {
            this.model.add_new_member();
        },

        user_added: function( data ) {
            var members = this.model.get( "members" );
            var view = new UserView({
                model: data,
                id: "user-" + members.size()
            });
            var user_el = view.render().$el;
            this.$("#users-list").append( user_el );
            this.check_users_for_remove();
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

        check_users_for_remove: function() {
            var members = this.model.get( "members" );
            var is_removeable = members.size() !== 1;
            members.forEach( function( m ) {
                m.set_removeable( is_removeable );
            });
        },

        submit: function() {
            var self = this;
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
        },

        show_errors: function( data ) {
            var NAME = "name";
            var DESCRIPTION = "description";
            var EMPTY = "empty";
            var TOO_LONG = "too_long";
            var NOT_FOUND = "not_found";

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
                    text_error = "Пользователь с именем " + desc.field + " не найден";
                }

                var templ = Handlebars.templates.closeable_error;
                var newError = $( templ({ text: text_error }) );
                $("#errors-list").append( newError );
            });
        },
    });

    return GroupCreationView;
})

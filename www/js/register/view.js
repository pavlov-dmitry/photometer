define( function(require){
    var Handlebars = require( "handlebars.runtime" ),
        Backbone = require( "lib/backbone" ),
        Config = require( "config" );
    require( "template/register" );
    require( "template/registered" );

    var RegisterView = Backbone.View.extend({

        el: "#workspace",

        template: Handlebars.templates.register,

        events: {
            "submit #form-register": "submit"
        },

        initialize: function() {
            this.model.on( "change", this.render, this );
            this.model.on( "registered", this.registered, this );
            this.render();
            this.animatedShow();
        },

        render: function() {
            $(this.el).html( this.template( this.model.toJSON() ) )
            var has_error = this.model.get( 'has_error' );
            $("#form-reg-error").toggleClass( 'hidden', !has_error );
            $("#name").focus();

            this.form_validation();

            return this;
        },

        submit: function() {
            var $form = $("#form-register");
            var is_valid = $form.form( "is valid" )
            if ( is_valid ) {
                $form.addClass( "loading" );
                var self = this;
                var name = $("#name").val();
                var mail = $("#mail").val();
                var handler = this.model.register( name,
                                                   $("#pasw").val(),
                                                   mail );
                handler.finish = function() {
                    $form.removeClass( "loading" );
                }
                handler.bad = function( data ) {
                    var state = {
                        name: name,
                        email: mail,
                        has_error: true
                    };
                    if ( data.reason === "exists" ) {
                        state.error = "Пользователь с таким именем или почтой уже существует";
                    }
                    else if ( data.field == "user" && data.reason == "invalid" ) {
                        state.error = "Имя пользователя может содержать только буквы, пробел, подчеркивание или знак минуса";
                    }
                    else { // для всех необработанных ошибок
                        state.error = JSON.stringify( data );
                    }
                    self.model.set( state );
                }
            }
        },

        registered: function() {
            $(this.el).html( Handlebars.templates.registered( this.model.toJSON() ) );
            this.animatedShow();
        },

        animatedShow: function() {
            $(this.el).children().hide();
            $(this.el).children().fadeIn( Config.showAniTime );
        },

        form_validation: function() {
            var rules = {
                fields: {
                    name: {
                        identifier: 'name',
                        rules: [
                            {
                                type: 'empty',
                                prompt: "Ну и как нам вас величать?"
                            },
                            {
                                type: 'maxLength[24]',
                                prompt: "Ох и имечко, может быть как-нить по короче?"
                            },
                            {
                                type: "regExp[/^[\\w \\u0430-\\u044F\\u0410-\\u042F-]+$/]",
                                prompt: "Имя пользователя может содержать только буквы, пробел, подчеркивание или знак минуса"
                            }
                        ]
                    },
                    pasw: {
                        identifier: 'pasw',
                        rules: [
                            {
                                type: 'empty',
                                prompt: "Нужен какой-нибудь пароль."
                            }
                        ]
                    },
                    pasw2: {
                        identifier: "pasw2",
                        rules: [
                            {
                                type: 'empty',
                                prompt: 'Надо бы пароль повторить, чтоб уж точно быть уверенным, что ввели.'
                            },
                            {
                                type: 'match[pasw]',
                                prompt: 'Уверены что точно ввели, что хотели, а то что-то не совпадают пароли то.'
                            }
                        ]
                    },
                    mail: {
                        identifier: "mail",
                        rules: [
                            {
                                type: "email",
                                prompt: "Нам бы почту, ну только чтоб только важные оповещения слать. Честно, честно."
                            },
                            {
                                type: 'maxLength[128]',
                                prompt: "Никогда не думал что бывают такие длинные имена почтовых ящиков o_O"
                            }
                        ]
                    }
                },
                inline: true,
                on: 'submit'
            };
            $("#form-register").form( rules );
        }
    });

    return RegisterView;
});

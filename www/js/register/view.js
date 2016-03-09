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

            // var closable_message = require( "helpers/closable_message");
            // closable_message();

            this.form_validation();

            return this;
        },

        submit: function() {
            var $form = $("#form-register");
            var is_valid = $form.form( "is valid" )
            if ( is_valid ) {
                $form.addClass( "loading" );
                var self = this;
                var handler = this.model.register( $("#name").val(),
                                                   $("#pasw").val(),
                                                   $("#mail").val() );
                handler.finish = function() {
                    $form.removeClass( "loading" );
                }
                handler.bad = function( data ) {
                    var state = {
                        has_error: true
                    };
                    if ( data.reason === "exists" ) {
                        state.error = "Пользователь с таким именем или почтой уже существует";
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
                                type: 'maxLength[64]',
                                prompt: "Ох и имечко, может быть как-нить по короче?"
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
                                prompt: 'Уверены что чточно ввели, что хотели, а то что-то не совпадают пароли то.'
                            }
                        ]
                    },
                    mail: {
                        identifier: "mail",
                        rules: [
                            {
                                type: "email",
                                prompt: "Нам бы почту, ну только чтоб только важные оповещения слать. Честно, честно."
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

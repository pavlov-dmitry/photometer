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
            "submit #form-register": "on_submit"
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

            var closable_message = require( "helpers/closable_message");
            closable_message();

            this.form_validation();

            return this;
        },

        on_submit: function() {
            $("#form-register").form( "validate form" );
        },

        submit: function() {
            this.model.register( $("#name").val(),
                                 $("#pasw").val(),
                                 $("#mail").val() );
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
                on: 'blur'
            };
            var self = this;
            rules.onSuccess = function() {
                self.submit();
            };
            $("#form-register").form( rules );
        }
    });

    return RegisterView;
});

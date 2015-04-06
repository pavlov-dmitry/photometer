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
            $("#form-reg-name").focus();
            return this;
        },

        submit: function() {
            this.model.register( $("#form-reg-name").val(),
                                 $("#form-reg-pasw").val(),
                                 $("#form-reg-mail").val());
        },

        registered: function() {
            $(this.el).html( Handlebars.templates.registered( this.model.toJSON() ) );
            this.animatedShow();
        },

        animatedShow: function() {
            $(this.el).children().hide();
            $(this.el).children().fadeIn( Config.showAniTime );
        }

    });

    return RegisterView;
});

define( function(require) {

    var Handlebars = require( "handlebars.runtime" ),
        Backbone = require( "lib/backbone" ),
        Config = require( "config" ),
        login_tmpl = require( "template/login" );

    var UserLoginView = Backbone.View.extend({
        el: '#workspace',

        template:  Handlebars.templates.login,

        events: {
            "submit #form-login": "submit",
        },

        initialize: function() {
            this.model.on( "change:has_error", this.render, this );
            // this.model.once( "login_success", this.close, this );
            this.model.on( "request", function() {
            });
            this.model.on( "sync", function() {
            });
            this.render();
            $(this.el).children().hide();
            $(this.el).children().fadeIn( Config.showAniTime );
        },

        submit: function() {
            var self = this;
            this.$form.addClass( "loading" );
            var handler = this.model.login( $("#login-name").val(), $("#login-pasw").val() );
	    handler.bad = function( err ) {
		if ( err.reason === "not_found" ) {
		    self.model.set( {
			has_error: true,
			error: "Пользователь с таким паролем не найден."
		    } );
		}
	    }
            handler.finish = function() {
                this.$form.removeClass( "loading" );
            }
        },

        render: function() {
            $(this.el).html( $( this.template( this.model.toJSON() ) ) );
	    var has_error = this.model.get( 'has_error' );
            $( "#login-error" ).toggleClass( "hidden", !has_error );
	    $("#login-name").focus();
            this.$form = $("#form-login");

            return this;
        },

    });
    return UserLoginView;

});

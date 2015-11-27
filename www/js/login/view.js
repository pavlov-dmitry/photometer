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
                console.log( "start request" );
            });
            this.model.on( "sync", function() {
                console.log( "synced" );
            });
            this.render();
            $(this.el).children().hide();
            $(this.el).children().fadeIn( Config.showAniTime );
        },

        submit: function() {
            this.model.login( $("#login-name").val(), $("#login-pasw").val() );
        },

        render: function() {
            $(this.el).html( $( this.template( this.model.toJSON() ) ) );
	    var has_error = this.model.get( 'has_error' );
            $( "#login-error" ).toggleClass( "hidden", !has_error );
	    $("#login-name").focus();
            return this;
        },

    });
    return UserLoginView;

});

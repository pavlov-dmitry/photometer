define( function(require) {
    var Backbone = require( "lib/backbone" );
    var Request = require( "request" );
    var app = require( "app" );

    var UserLoginModel = Backbone.Model.extend ({
        url: '/login',
        defaults: {
            'user': '',
            'password': '',
            'has_error': false,
            'error': ''
        },

        login: function(usr, psw) {
            var model = this;
	    //TODO: возможно это и не надо делать?
	    model.set( { user: usr } );

	    var handler = Request.post( this.url, {
		user: usr,
		password: psw
	    } );

	    handler.good = function( data ) {
                app.makeLogin( data.sid );
	    }
            return handler;

        },
    });

    return UserLoginModel;
});

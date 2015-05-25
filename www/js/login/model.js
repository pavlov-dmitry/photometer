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

	    handler.bad = function( err ) {
		if ( err.reason === "not_found" ) {
		    model.set( {
			has_error: true,
			error: "Пользователь с таким паролем не найден."
		    } );
		}
	    }
        },

        // sync: function(method, model, options) {
        //     switch (method) {

        //         case 'create':
        //             options.url = model.url;
        //             options.method = 'POST';
        //             options.contentType = "application/json";
        //             options.data = JSON.stringify({
        //                     user: model.get( 'user' ),
        //                     password: model.get( 'password' )
        //                 });
        //             var ajaxObj = Backbone.$.ajax( options );
        //             model.trigger( "request" );
        //             return ajaxObj;
        //         break;

        //     }
        // }
    });

    return UserLoginModel;
});

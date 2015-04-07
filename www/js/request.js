/// Легкая обёртка на JQuery.ajax адаптированная под протокол работы с
/// фотометром

define( function( require ) {
    var $ = require( "jquery" );

    var Request = {
	get: function( url, data ) {
	    return this.send( "GET", url, data );
	},

	post: function( url, data ) {
	    return this.send( "POST", url, data );
	},

	send: function( method, url, data ) {
	    var self = this;
	    var handlerObj = {
		good: function() { console.log( "default good" ); },
		bad: function() { console.log( "default.bad" ); },
		access_denied: function() {
                    require( ['app'], function( app ) {
                        app.error( "Отказано в доступе. Кажется кто-то что-то химичит, или что-то пошле не так." )
                    });
                }
	    }
	    var ajaxHandler = $.ajax({
		url: url,
		contentType: "application/json",
		method: method,
		data: JSON.stringify( data ),
	    });
	    ajaxHandler.done( function( data ) {
		handlerObj.good( data );
	    });
	    ajaxHandler.fail( function( resp ) {
		if ( resp.status === 400 ) {
		    var responseData = JSON.parse( resp.responseText );
		    if ( responseData && responseData.access && responseData.access === "denied" ) {
			handlerObj.access_denied();
		    }
		    else if ( responseData ) {
			handlerObj.bad( responseData );
		    }
		    else {
			self.internalError( resp, this );
		    }
		}
		else {
		    self.internalError( resp, this );
		}
	    });

	    return handlerObj;
	},

	internalError: function( resp, ajax ) {
	    console.log( "internal error" );
	}
    };

    return Request;
});

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
                    var errorsHandler = require( "errors_handler" );
                    errorsHandler.error( "Отказано в доступе. Кажется кто-то что-то химичит, или что-то пошле не так." );
                },
                unauthorized: function() {
                    self.unauthorized();
                }
	    }

            var options = {
                url: url,
                method: method,
            };
            if ( data ) {
                options.data = JSON.stringify( data );
            }
            // if ( method !== "GET" ) {
                options.contentType = "application/json";
                options.dataType = "json";
            // }

	    var ajaxHandler = $.ajax( options );

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
                else if ( resp.status === 401 ) {
                    console.log( "unauthorized" );
                    handlerObj.unauthorized();
                }
		else {
		    self.internalError( resp, this );
		}
	    });

	    return handlerObj;
	},

	internalError: function( resp, ajax ) {
	    console.log( "internal error" );
	},

        unauthorized: function() {
            console.log( "unauthorized" );
        }
    };

    return Request;
});

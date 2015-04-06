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
		good: function( data ) { console.log( "default good" ); },
		bad: function( data ) { console.log( "default.bad" ); },
		//TODO: реализовать общую обработку отказа в доступе
		access_denied: function() { console.log( "default.access_denied" ); }
	    }
	    var ajaxHandler = $.ajax({
		url: url,
		contentType: "application/json",
		method: method,
		data: JSON.stringify( data ),
	    });
	    ajaxHandler.done( function( data ) {
		handlerObj.good( reponseData );
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

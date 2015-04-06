define( function (require) {
    var $ = require( "jquery" ),
    Backbone = require( "lib/backbone" ),
    Workspace = require( "workspace" ),
    Handlebars = require( "handlebars.runtime" ),
    Request = require( "request" );

    var app = {};

    app.processInternalError = function( response, ajax ) {
        require( "template/dev_error" );

        $( "#workspace" ).html( Handlebars.templates.dev_error( {
	    ajax: ajax,
	    response: response
        }));
    };

    $( function() {
	Request.internalError = app.processInternalError;

        app.workspace = new Workspace;

        //настройка обработки внутренних ошибок сервера
        app._backbone_sync = Backbone.sync;
        Backbone.sync = function(method, model, options) {
            app._backbone_sync( method, model, options )
        	.fail( function( resp ) {
		    app.processInternalError( resp, this );
		});
        };

        Backbone.history.start();
    });
    return app;
});

define( function (require) {
	var $ = require( "jquery" ),
        Backbone = require( "lib/backbone" ),
        Workspace = require( "workspace" ),
        Handlebars = require( "handlebars.runtime" );

    var app = {};

    app.processInternalError = function( response ) {
        require( "template/dev_error" );

        $( "#workspace" ).html( Handlebars.templates.dev_error( {
            error_msg: response.responseText
        }));
    };

	$( function() {
        app.workspace = new Workspace;

        //настройка обработки внутренних ошибок сервера
        app._backbone_sync = Backbone.sync;
        Backbone.sync = function(method, model, options) {
        	app._backbone_sync( method, model, options )
        	.fail( function( resp ) { 
                app.processInternalError( response ); 
            });
        };

        Backbone.history.start();
	});
    return app;
});
define( function (require) {
    var $ = require( "jquery" ),
    Backbone = require( "lib/backbone" ),
    Workspace = require( "workspace" ),
    Handlebars = require( "handlebars.runtime" ),
    Request = require( "request" );


    var app = {};

    // обработка ошибок сервера
    app.processInternalError = function( response, ajax ) {
        require( "template/dev_error" );

        $( "#workspace" ).html( Handlebars.templates.dev_error( {
	    ajax: ajax,
	    response: response
        }));
    };

    /// выполнить вход
    app.makeLogin = function( name, sid ) {
        require( "lib/jquery.cookie" );
        $.cookie( "sid", sid );
        console.log( "cookies setted" );
    }

    /// инициализация
    $( function() {
	Request.internalError = app.processInternalError;

        app.workspace = new Workspace;

        Backbone.history.start();
    });
    return app;
});

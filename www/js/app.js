define( function (require) {
	var $ = require( "jquery" ),
        Backbone = require( "lib/backbone" ),
        Workspace = require( "workspace" );

    var app = {};

	$( function() {
        app.workspace = new Workspace;
        Backbone.history.start();
	});
    return app;
});
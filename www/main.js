require.config({
    baseUrl: 'js',
    shim : {
        'bootstrap' : { deps :['jquery'] },
        'jquery.imgareaselect': { deps: ['jquery'] },
        'showdown' : {},
    },
    paths: {
	'jquery': 'lib/jquery',
        'jquery.ui.widget': 'lib/jquery.ui.widget',
        'jquery.imgareaselect': 'lib/jquery.imgareaselect',
	'bootstrap': 'lib/bootstrap',
        'underscore': 'lib/underscore',
        'template': '../template',
        'handlebars.runtime': 'lib/handlebars.runtime',
        'showdown': 'lib/showdown'
    }
});

require( [ 'bootstrap',
           'app',
           'workspace',
           'request',
           'user_state/model',
           'errors_handler',
           'user_state/header_view'],
         function ( bootstrap,
                    app,
                    Workspace,
                    request,
                    UserStateModel,
                    errorsHandler,
                    UserStateHeaderView ) {
    /// инициализация
    $( function() {
        app.workspace = new Workspace;
        app.userState = new UserStateModel;

        request.internalError = errorsHandler.processInternalError;
        request.unauthorized = function() {
            app.unauthorized();
        };

        var userStateHeaderView = new UserStateHeaderView( { model: app.userState } );
        app.userState.fetch();

        Backbone.history.start();
    });
} );

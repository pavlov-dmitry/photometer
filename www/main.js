require.config({
    baseUrl: 'js',
    shim : {
        'semantic' : { deps :['jquery'] },
        'jquery.imgareaselect': { deps: ['jquery'] },
        'showdown' : {},
    },
    paths: {
	'jquery': 'lib/jquery',
        'jquery.ui.widget': 'lib/jquery.ui.widget',
        'jquery.imgareaselect': 'lib/jquery.imgareaselect',
        'jquery.datetimepicker': 'lib/jquery.datetimepicker.full',
	'semantic': 'lib/semantic',
        'underscore': 'lib/underscore',
        'template': '../template',
        'handlebars.runtime': 'lib/handlebars.runtime',
        'showdown': 'lib/showdown',
        'moment': 'lib/moment-with-russian-locale'
    }
});

require( [ 'semantic',
           'app',
           'workspace',
           'request',
           'user_state/model',
           'errors_handler',
           'user_state/header_view',
           "jquery.datetimepicker",
           'moment' ],
         function ( semantic,
                    app,
                    Workspace,
                    request,
                    UserStateModel,
                    errorsHandler,
                    UserStateHeaderView,
                    datetimepicker,
                    moment ) {
    /// инициализация
    $( function() {
        app.workspace = new Workspace;
        app.userState = new UserStateModel;

        $.datetimepicker.setLocale('ru');

        request.internalError = errorsHandler.processInternalError;
        request.unauthorized = function() {
            app.unauthorized();
        };

        var userStateHeaderView = new UserStateHeaderView( { model: app.userState } );
        app.userState.fetch();

        Backbone.history.start();
    });
} );

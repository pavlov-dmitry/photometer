define( function (require) {
    var $ = require( "jquery" ),
    Backbone = require( "lib/backbone" ),
    Workspace = require( "workspace" ),
    Request = require( "request" ),
    UserStateModel = require( "user_state/model" );
    require( "lib/jquery.cookie" );


    var app = {
        /// выполнить вход
        makeLogin: function( sid ) {
            //TODO: возможно стоит работу с куками опустить в модель
            //состояния пользователя
            $.cookie( "sid", sid );
            this.userState.fetch();
        },

        logout: function() {
            $.removeCookie( "sid" );
            // this.userState.logout();
            this.userState.fetch();
            this.workspace.nav( "login" );
        },

        /// текущее состояние пользователя
        userState: new UserStateModel(),
        /// переключатель рабочей среды
        workspace: new Workspace()
    };

    /// инициализация
    $( function() {
        var errorsHandler = require( "errors_handler" );
        Request.internalError = errorsHandler.processInternalError;

        var UserStateHeaderView = require( "user_state/header_view" );
        var userStateHeaderView = new UserStateHeaderView( { model: app.userState } );
        app.userState.fetch();

        Backbone.history.start();
    });

    return app;
});

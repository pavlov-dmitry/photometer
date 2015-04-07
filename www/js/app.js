define( function (require) {
    var $ = require( "jquery" ),
    Backbone = require( "lib/backbone" ),
    Workspace = require( "workspace" ),
    Handlebars = require( "handlebars.runtime" ),
    Request = require( "request" ),
    UserStateModel = require( "user_state/model" );
    require( "lib/jquery.cookie" );


    var app = {
        /// обработка ошибок сервера
        processInternalError: function( response, ajax ) {
            require( "template/dev_error" );

            $( "#workspace" ).html( Handlebars.templates.dev_error( {
	        ajax: ajax,
	        response: response
            }));
        },

        /// выполнить вход
        makeLogin: function( name, sid ) {
            //TODO: возможно стоит работу с куками опустить в модель
            //состояния пользователя
            $.cookie( "sid", sid );
            this.userState.logged( name );
        },

        logout: function() {
            $.removeCookie( "sid" );
            this.userState.logout();
            this.workspace.nav( "login" );
        },

        /// текущее состояние пользователя
        userState: new UserStateModel(),
        /// переключатель рабочей среды
        workspace: new Workspace()
    };

    /// инициализация
    $( function() {
        Request.internalError = app.processInternalError;

        var UserStateHeaderView = require( "user_state/header_view" );
        var userStateHeaderView = new UserStateHeaderView( { model: app.userState } );

        Backbone.history.start();
    });

    return app;
});

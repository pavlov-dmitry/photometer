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

            // елси мы залогинились и хотели посетить какую-то опередленную страницию
            if ( this._redirectToAfterLogin !== "" ) {
                // то переходим на неё
                this.workspace.nav( this._redirectToAfterLogin );
            }
        },

        logout: function() {
            $.removeCookie( "sid" );
            // this.userState.logout();
            this.userState.fetch();
            this.workspace.nav( "login" );
        },

        unauthorized: function() {
            //находим текущее положение наше положение
            var locationUrl =  $(location).attr("href");
            var gridPos = locationUrl.indexOf( '#' );
            if ( 0 <= gridPos ) {
                locationUrl = locationUrl.substring( gridPos + 1, locationUrl.length );
            }
            else {
                locationUrl = "";
            }

            // елси это повтор то не обрабатываем
            if ( locationUrl !== "" &&
                 locationUrl === this._redirectToAfterLogin ) {
                return;
            }
            // неавторизованный логин тоже не обрабатываем
            if ( locationUrl === "login" ) {
                return;
            }
            // если есть куда вернуться, то запоминаем куда нам вернуться после авторизации
            if ( locationUrl !== "" ) {
                this._redirectToAfterLogin = locationUrl;
            }
            // идём авторизироваться
            this.workspace.nav( "login" );
        },

        /// текущее состояние пользователя
        userState: new UserStateModel(),
        /// переключатель рабочей среды
        workspace: new Workspace(),
        /// куда стоит перейти после логина, по умолчанию идём в галлерею
        _redirectToAfterLogin: "gallery"
    };

    /// инициализация
    $( function() {
        var errorsHandler = require( "errors_handler" );
        Request.internalError = errorsHandler.processInternalError;
        Request.unauthorized = function() {
            app.unauthorized();
        };

        var UserStateHeaderView = require( "user_state/header_view" );
        var userStateHeaderView = new UserStateHeaderView( { model: app.userState } );
        app.userState.fetch();

        Backbone.history.start();
    });

    return app;
});

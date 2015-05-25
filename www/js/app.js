define( function (require) {
    var $ = require( "jquery" ),
        Backbone = require( "lib/backbone" );
    var JQCookie = require( "lib/jquery.cookie" );

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
            this._redirectToAfterLogin = locationUrl;
            if ( this._redirectToAfterLogin === "" ) {
                this._redirectToAfterLogin = "gallery";
            }
            // идём авторизироваться
            this.workspace.nav( "login" );
        },

        user_id: function() {
            return this.userState.id;
        },

        /// текущее состояние пользователя
        userState: null, //new UserStateModel(),
        /// переключатель рабочей среды
        workspace: null, //new Workspace(),
        /// куда стоит перейти после логина, по умолчанию идём в галлерею
        _redirectToAfterLogin: ""
    };


    return app;
});

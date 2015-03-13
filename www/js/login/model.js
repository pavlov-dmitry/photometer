define( function(require) {
    var Backbone = require( "lib/backbone" );

    var UserLoginModel = Backbone.Model.extend ({
        //url: '/users/reg',
        //paramRoot: 'user',
        defaults: {
            'name': '',
            'has_error': false,
            'error': ''
        },

        login: function(login, psw) {
            if ( login === "close" ) {
                this.trigger( "login_success" );
            } else {
                this.set( {
                    name: login,
                    has_error: true, 
                    error: "Ну нет такого имени"
                } );
            }
        }
    });
    return UserLoginModel;
});
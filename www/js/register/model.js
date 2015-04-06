define( function(require) {
    var Backbone = require( "lib/backbone" );
    var Request = require( "request" );

    var RegisterModel = Backbone.Model.extend({
        defaults: {
            'name': '',
            'email': '',
            'has_error': false
        },

        register: function( usr, psw, mail ) {
            var self = this;
            var handler = Request.post( "/join_us", {
                user: usr,
                password: psw,
                email: mail
            } );

            handler.good = function( data ) {
                self.trigger( "registered" );
            };

            handler.bad = function( data ) {
                var state = {
                    name: usr,
                    email: mail,
                    has_error: true
                };
                if ( data.reason === "exists" ) {
                    state.error = "Пользователь с таким именем или почтой уже существует";
                }
                else { // для всех необработанных ошибок
                    state.error = JSON.stringify( data );
                }

                self.set( state );
            };
        }
    });

    return RegisterModel;
})

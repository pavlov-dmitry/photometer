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
            return handler;
        }
    });

    return RegisterModel;
})

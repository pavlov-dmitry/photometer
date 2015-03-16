define( function(require) {
    var Backbone = require( "lib/backbone" );

    var UserLoginModel = Backbone.Model.extend ({
        url: '/login',
        //paramRoot: 'user',
        defaults: {
            'user': '',
            'password': '',
            'has_error': false,
            'error': ''
        },

        login: function(usr, psw) {
            var model = this;
            this.save(
                { user: usr, password: psw }
            )
            .fail( function( resp ) {
                var app = require( "app" );
                app.processInternalError( resp, this );
            })
            .done( function( data ) {
                console.log( "done" );
                if ( data.errors_exists ) {
                    model.set({
                        has_error: true,
                        error: JSON.stringify( data.errors[ 0 ] )
                    });
                }
                if ( data.records_exists ) {
                    console.log( "need set cookies" );
                }
            });
        },

        sync: function(method, model, options) {
            switch (method) {

                case 'create':
                    options.url = model.url;
                    options.method = 'POST';
                    options.contentType = "application/json";
                    options.data = JSON.stringify({
                            user: model.get( 'user' ),
                            password: model.get( 'password' )
                        });
                    var ajaxObj = Backbone.$.ajax( options );
                    model.trigger( "request" );
                    return ajaxObj;
                break;

            }
        }

    });

    return UserLoginModel;
});

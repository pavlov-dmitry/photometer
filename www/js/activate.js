define( function(require) {
    var Request = require( "request" );
    var Handlebars = require( "handlebars.runtime" );

    var activate = function( key ) {
        var url = "/registration/" + key;
        var handler = Request.get( url, {} );

        handler.good = function( data ) {
            require( ['app'], function( app ) {
                app.makeLogin( "NAME", data.sid );
            } );
        };

        handler.bad = function( data ) {
            require( "template/text_error" );
            var template = Handlebars.templates.text_error;
            $("#workspace").html( template( {
                header: "Ошибка активации учётной записи",
                text: "Учётная запись, с таким ключём активации, не найдена."
            } ) );
        }
    }

    return activate;
} );

define( function(require) {
    var Request = require( "request" );
    var Handlebars = require( "handlebars.runtime" );
    var app = require( "app" );

    var activate = function( key ) {
        var url = "/registration/" + key;
        var handler = Request.get( url, {} );

        handler.good = function( data ) {
            app.makeLogin( data.sid );
            app.workspace.nav( "mailbox/unreaded" );
        };

        handler.bad = function( data ) {
            var errorHandler = require( "errors_handler" );
            errorHandler.oops( "Ошибка активации учётной записи",
                               "Учётная запись, с таким ключём активации, не найдена." );
        }
    }

    return activate;
} );

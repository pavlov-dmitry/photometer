define( function(require) {
    var Handlebars = require( "handlebars.runtime" ),
        markdown = require( "showdown_converter" ),
        moment = require( "moment" );

    Handlebars.registerHelper( "markdown", function( data ) {
        return markdown.makeHtml( data );
    });

    Handlebars.registerHelper( "shutter", function( data ) {
        if ( data < 0 ) {
            return "1/" + Math.abs( data );
        }
        return data;
    });

    Handlebars.registerHelper( "aperture", function( data ) {
        data = data * 10;
        data = Math.floor( data ) / 10;
        return data;
    });

    Handlebars.registerHelper( "include", function( name, data ) {
        var template = Handlebars.templates[name];
        if ( template ) {
            return template( data );
        }
        else {
            throw "handlebars template \"" + name + "\" not found by include helper";
        }
    });

    Handlebars.registerHelper( "time", function( data ) {
        return moment( data ).format( "HH:mm dddd, DD MMMM YYYY");
    });

    Handlebars.registerHelper( "duration_from_now", function( data ) {
        return moment( data ).fromNow();
    });

    return {};
})

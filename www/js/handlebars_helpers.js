define( function(require) {
    var Handlebars = require( "handlebars.runtime" ),
        markdown = require( "showdown_converter" );

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

    return {};
})

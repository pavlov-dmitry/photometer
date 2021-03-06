define( function(require) {
    var Handlebars = require( "handlebars.runtime" ),
        markdown = require( "showdown_converter" ),
        moment = require( "moment" ),
        make_pagination = require( "make_pagination" );
    require( "template/pagination" );
    require( "template/markdown_help");


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

    Handlebars.registerHelper( "fulltime", function( data ) {
        return moment( data ).format( "HH:mm dddd, DD MMMM YYYY");
    });

    Handlebars.registerHelper( "time", function( data ) {
        return moment( data ).format( "HH:mm ddd, DD MMM YYYY");
    });

    Handlebars.registerHelper( "duration_from_now", function( data ) {
        return moment( data ).fromNow();
    });

    Handlebars.registerHelper( "pagination", function( data, link_prefix ) {
        if ( 1 < data.count ) {
            var pagination_data = make_pagination( data.current, data.count, link_prefix );
            return Handlebars.templates.pagination( pagination_data );
        }
    });

    Handlebars.registerHelper( "if_equal", function( data, value, options ) {
        if ( data === value ) {
            return options.fn( this );
        }
        else {
            return options.inverse( this );
        }
    });

    Handlebars.registerHelper( "humanize", function( value, one, two2four, ten ) {
        var value = value % 100;
        if ( 20 < value )
        {
            value = value % 10;
        }
        switch( value ) {
        case 1:
            return one;
        case 2:
        case 3:
        case 4:
            return two2four;
        }
        return ten;
    });

    Handlebars.registerHelper( "as_percent", function( data ) {
        data = Math.floor( data * 100 );
        return data;
    });

    return {};
})

define( function(require) {
    var Handlebars = require( "handlebars.runtime" );
    require( "template/publication_info");

    var late_publication_info = {
        makeHtml: function( data ) {
            return Handlebars.templates.publication_info( data );
        }
    };
    return late_publication_info;
})

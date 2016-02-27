define( function(require) {
    var Handlebars = require( "handlebars.runtime" );
    require( "template/user_invite_info" );

    var publication_info = {
        makeHtml: function( data ) {
            return Handlebars.templates.user_invite_info( data );
        }
    };
    return publication_info;
})

define( function(require) {
    var Handlebars = require( "handlebars.runtime" ),
        internal_collection = require( "event/internal_collection" );
    require( "template/group_voting_info" );

    var group_voting_info = {
        caption: function( name, data ) {
            var info_data = JSON.parse( data.info );
            var info = internal_collection( data.internal_id );
            return info.caption( name );
        },
        makeHtml: function( data ) {
            var info_data = JSON.parse( data.info );
            var info = internal_collection( data.internal_id );
            data.internal_html = info.makeHtml( info_data );
            return Handlebars.templates.group_voting_info( data );
        }
    };
    return group_voting_info;
})

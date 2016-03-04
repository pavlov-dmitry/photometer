define( function(require) {
    var internal_collection = function( id ) {
        var base_info = {
            caption: function( name ) {
                return name;
            },
            makeHtml: function( data ) {
                return JSON.stringify( data );
            }
        };
        var info = {};

        switch ( id ) {
        case 'ChangeTimetable':
            info = require( "event/info/change_timetable" );
            break;

        case 'JoinToGroup':
            info = require( "event/info/join_to_group" );
            break;
        }

        $.extend( base_info, info );
        return base_info;
    }

    return internal_collection;
})

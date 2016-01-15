define( function(require) {
    var internal_collection = function( id ) {
        var base_info = {
            makeHtml: function( data ) {
                return JSON.stringify( data );
            }
        };
        var info = {};

        if ( id === 'ChangeTimetable' ) {
            info = require( "event/info/change_timetable" );
        }

        $.extend( base_info, info );
        return base_info;
    }

    return internal_collection;
})

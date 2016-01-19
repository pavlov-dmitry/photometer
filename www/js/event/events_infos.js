define( function(require) {
    var events_info_collection = function( id ) {
        var base_info = {
            caption: function( name ) {
                return name;
            },
            makeHtml: function( data ) {
                return JSON.stringify( data );
            },
            answer: "Вы согласны?"
        };

        var info = {};

        if ( id === 'GroupCreation') {
            info = require( "event/info/group_creation" );
        }
        else if ( id == 'GroupVoting' ) {
            info = require( "event/info/group_voting" );
        }
        else if ( id === 'Publication' ) {
            info = require( "event/info/publication" );
        }

        $.extend( base_info, info );
        return base_info;
    }

    return events_info_collection;
})

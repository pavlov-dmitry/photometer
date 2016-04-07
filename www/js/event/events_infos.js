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

        switch ( id ) {
            case 'GroupCreation':
                info = require( "event/info/group_creation" );
                break;

            case 'GroupVoting':
                info = require( "event/info/group_voting" );
                break;

            case 'Publication':
                info = require( "event/info/publication" );
                break;

            case 'LatePublication':
                info = require( "event/info/late_publication" );
                break;

            case 'UserInvite':
                info = require( "event/info/user_invite" );
                break;
        }

        $.extend( base_info, info );
        return base_info;
    }

    return events_info_collection;
})

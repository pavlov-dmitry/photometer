define( function(require) {
    var factory = function( event_id ) {
        switch ( event_id ) {
        case 'GroupVoting': return require( "group_feed/views/group_voting" );
        case 'Publication': return require( "group_feed/views/publication" );
        }
        return null;
    }
    return factory;
});

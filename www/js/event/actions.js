define( function(require) {
    return function( action ) {
        switch (action) {

        case "None": return require( "event/action/none" );
        case "Vote": return require( "event/action/vote" );

        }
        return {};
    }
})

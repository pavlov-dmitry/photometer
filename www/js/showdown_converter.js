define( function( require ) {
    var Showdown = require( 'showdown' );
    var converter = new Showdown.converter();
    return converter;
})

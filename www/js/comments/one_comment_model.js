define( function( require ) {
    var Backbone = require( "lib/backbone" );

    var OneCommentModel = Backbone.Model.extend({
        defaults: {
            id: 0
        }
    });

    return OneCommentModel;
})

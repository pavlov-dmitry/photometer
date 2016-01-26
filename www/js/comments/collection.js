define( function(require) {
    var Backbone = require( "lib/backbone" ),
        OneCommentModel = require( "comments/one_comment_model" );

    var CommentsCollection = Backbone.Collection.extend({
        model: OneCommentModel
    });

    return CommentsCollection;
})

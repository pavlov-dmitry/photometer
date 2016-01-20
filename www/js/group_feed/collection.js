define( function(require) {
    var Backbone = require( "lib/backbone" ),
        FeedElementModel = require( "group_feed/element_model" );
    var FeedCollection = Backbone.Collection.extend({
        model: FeedElementModel
    });
    return FeedCollection;
})

define( function(require) {
    var Backbone = require( "lib/backbone" );
    var FeedElementModel = Backbone.Model.extend({
        fetch: function() {
            var request = require( "request" );
            var handler = request.get( "group/feed/element", { id: this.get("id") } );
            var self = this;
            handler.good = function( data ) {
                self.set( data );
            }
            return handler;
        }
    })
    return FeedElementModel;
})

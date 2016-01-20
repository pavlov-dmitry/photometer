define( function(require) {
    var Backbone = require( "lib/backbone" ),
        GroupFeedCollection = require( "group_feed/collection" ),
        request = require( "request" );

    var GroupFeedModel = Backbone.Model.extend({
        defaults: {
            id: 0,
            page: 0,
            feed: new GroupFeedCollection()
        },

        fetch: function() {
            var self = this;
            var handler = request.get( "group/feed", {
                group_id: this.get("id"),
                page: 0
            });
            handler.good = function( data ) {
                console.log( JSON.stringify( data ) );
                var common_data = {
                    group_id: data.group_id,
                    group_name: data.group_name
                };
                self.set( common_data );
                var feed = self.get( "feed" );
                feed.add( data.feed );
            }
            return handler;
        },

        need_more: function() {

        }
    });
    return GroupFeedModel;
})

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
            return this.fetch_page( 0 );
        },

        need_more: function() {
            var page = this.get("page");
            page += 1;
            console.log( "need more " + page );
            this.set({ page: page });
            return this.fetch_page( page );
        },

        fetch_page: function( page ) {
            var self = this;
            var handler = request.get( "group/feed", {
                group_id: this.get("id"),
                page: page
            });
            handler.good = function( data ) {
                var common_data = {
                    group_id: data.group_id,
                    group_name: data.group_name
                };
                self.set( common_data );
                var feed = self.get( "feed" );
                feed.add( data.feed );
                self.trigger("fetched");
                if ( data.feed.length === 0 ) {
                    self.trigger( "no_more" );
                }
            }
            return handler;
        }
    });
    return GroupFeedModel;
})

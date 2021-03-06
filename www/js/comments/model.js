define( function(require) {
    var Backbone = require( "lib/backbone" ),
        CommentsCollection = require( "comments/collection" ),
        request = require( "request" );

    var CommentsModel = Backbone.Model.extend({
        defaults: {
            event_id: null,
            photo_id: null,
            page: 0,
            comments: new CommentsCollection(),
            text: ""
        },

        fetch: function( page ) {
            if ( !page ) {
                page = this.get("page");
            }
            this.set({ page: page });
            var url = this.get_url();
            var id = this.get_id();
            var handler = request.get( url, { id: id, page: page });
            var self = this;
            handler.good = function( data ) {
                self.set({
                    all_count: data.all_count,
                    pagination: data.pagination
                });
                var comments = self.get("comments");
                comments.reset();
                comments.add( data.comments );
            }
            return handler;
        },

        get_url: function() {
            var url = "";
            if ( this.get("event_id") ) {
                url = "/event/comments";
            }
            if ( this.get("photo_id") ) {
                url = "/photo/comments";
            }
            return url;
        },

        get_id: function() {
            var id = 0;
            id = this.get("event_id");
            if ( !id ) {
                id = this.get("photo_id");
            }
            return id;
        },

        save: function() {
            var url = this.get_url();
            var id = this.get_id();
            var handler = request.post( url, {
                id: id,
                text: this.get("text")
            });
            var self = this;
            handler.good = function( data ) {
                self.set({text: ""});
                self.trigger( "reset" );
                self.fetch();
            }
            return handler;
        }
    })

    return CommentsModel;
})

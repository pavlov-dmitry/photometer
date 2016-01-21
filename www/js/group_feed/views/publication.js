define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" ),
        request = require( "request" ),
        errors_handler = require( "errors_handler" );
    require( "template/publication_feed_view" );

    var PublicationFeedView = Backbone.View.extend({
        initialize: function() {
            this.listenTo( this.model, "change", this.render );
            var self = this;
            var handler = request.get( "publication", { id: this.model.get("scheduled_id") });
            handler.good = function( data ) {
                self.model.set({ photos: data });
            }
        },
        render: function() {
            var data = this.model.toJSON();
            var html = Handlebars.templates.publication_feed_view( data );
            this.$el.replaceWith( html );
            return this;
        }
    });
    return PublicationFeedView;
});

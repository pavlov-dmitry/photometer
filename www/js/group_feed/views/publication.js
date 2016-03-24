define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" ),
        request = require( "request" ),
        errors_handler = require( "errors_handler" );
    require( "template/publication_feed_view" );

    var PublicationFeedView = Backbone.View.extend({
        initialize: function() {
            this.listenTo( this.model, "change:photos", this.render );
            var self = this;
            var handler = request.get( "publication", { id: this.model.get("scheduled_id") });
            handler.good = function( data ) {
                self.model.set({ photos: data });
            }
        },
        render: function() {
            var data = this.model.toJSON();
            var html = Handlebars.templates.publication_feed_view( data );
            // NOTE: так как после replaceWith find уже не работет у
            // this.$el, то мы ищем наши картинки по другому
            this.$el.replaceWith( html );
            $("#feed-event-" + data.id).find(".image img").visibility({
                type: "image",
                transition: "fade in",
                duration: 500
            });
            return this;
        }
    });
    return PublicationFeedView;
});

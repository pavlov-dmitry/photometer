define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" ),
        request = require( "request" ),
        errors_handler = require( "errors_handler" ),
        make_upload_button = require( "make_upload_button" ),
        app = require( "app" );
    require( "template/publication_feed_view" );

    var PublicationFeedView = Backbone.View.extend({
        initialize: function() {
            this.listenTo( this.model, "change:photos", this.render );
            var self = this;
            var is_finish = this.model.get("state") === "Finish";
            if ( is_finish ) {
                var handler = request.get( "publication", { id: this.model.get("scheduled_id") });
                handler.good = function( data ) {
                    self.model.set({ photos: data });
                }
            }
        },
        render: function() {
            var data = this.model.toJSON();
            var html = Handlebars.templates.publication_feed_view( data );
            var is_start = this.model.get("state") === "Start";
            //NOTE: я реально упрел биться с этим браузером, и рендером, в одном случае генерит, в другом нет .. это просто ппц
            if ( is_start ) {
                this.$el = $( html );
            }
            else {
                this.$el.replaceWith( html );
            }
            // NOTE: так как после replaceWith find уже не работет у
            // this.$el, то мы ищем наши картинки по другому
            $("#feed-event-" + data.id).find(".image img").visibility({
                type: "image",
                transition: "fade in",
                duration: 500
            });

            if ( is_start ) {
                var self = this;
                var scheduled_id = this.model.get( "scheduled_id" );
                make_upload_button(
                    this, this.$el, "/upload_and_publish/" + scheduled_id,
                    function( photo_id ) {
                        app.workspace.nav( "edit_photo/" + photo_id );
                    }
                );
            }
            return this;
        }
    });
    return PublicationFeedView;
});

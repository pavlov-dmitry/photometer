define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" ),
        app = require( "app" );
    require( "template/group_feed_element" );

    var GroupFeedElementView = Backbone.View.extend({
        el: "#workspace",
        template: Handlebars.templates.group_feed_element,

            initialize: function() {
            this.listenTo( this.model, "change:event_id", this.render );
            this.model.fetch();
        },

        render: function() {
            var data = this.model.toJSON();
            this.$el.html( this.template( data ) );
            app.userState.set_current_group( data.group.id );

            var views_factory = require( "group_feed/views_factory" );
            var View = views_factory( this.model.get("event_id") );
            this.sub_view = new View({ model: this.model });
            $("#feed-view").append( this.sub_view.render().$el );
        },

        close: function() {
            if ( this.sub_view && this.sub_view.close ) {
                this.sub_view.close();
            }
        }

    });
    return GroupFeedElementView;
})

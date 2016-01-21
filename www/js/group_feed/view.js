define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" );
    require( "template/group_feed" );

    var GroupFeedView = Backbone.View.extend({
        el: "#workspace",
        template: Handlebars.templates.group_feed,

        initialize: function() {
            this.feeds = this.model.get("feed");
            this.feeds.reset();
            this.listenTo( this.feeds, "add", this.add );
            this.listenTo( this.model, "change:group_name", this.render );

            var handler = this.model.fetch();
            handler.bad = function() {
                var errors_handler = require( "errors_handler" );
                error_handler.oops( "Запрошенная группа не найдена." );
            }
        },

        close: function() {
            this.stopListening( this.feeds );
        },

        render: function() {
            this.$el.html( this.template( this.model.toJSON() ) );
        },

        add: function( model ) {
            var views_factory = require( "group_feed/views_factory" );
            var View = views_factory( model.get("event_id") );
            var view = new View( {model: model} );
            $("#feeds").append( view.render().$el );
        },

    });

    return GroupFeedView;
});

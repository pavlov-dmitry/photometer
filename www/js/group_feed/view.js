define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" );
    require( "template/group_feed" );

    var GroupFeedView = Backbone.View.extend({
        el: "#workspace",
        template: Handlebars.templates.group_feed,

        initialize: function() {
            this.feeds = this.model.get("feed");
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
            var View = this.view_by_event( model.get("event_id") );
            var view = new View( {model: model} );
            $("#feeds").append( view.render().$el );
        },

        view_by_event: function( event_id ) {
            switch ( event_id ) {
            case 'GroupVoting': return require( "group_feed/views/group_voting" );
            case 'Publication': return require( "group_feed/views/publication" );
            }
            return null;
        }
    });

    return GroupFeedView;
});

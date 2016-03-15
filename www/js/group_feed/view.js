define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" ),
        app = require( "app" );
    require( "template/group_feed" );

    var GroupFeedView = Backbone.View.extend({
        el: "#workspace",
        template: Handlebars.templates.group_feed,

        initialize: function() {
            this.feeds = this.model.get("feed");
            this.feeds.reset();
            this.listenTo( this.feeds, "add", this.add );
            this.listenTo( this.model, "fetched", this.feeds_changed );
            this.listenTo( this.model, "change:group_name", this.render );
            this.listenTo( this.model, "no_more", this.no_more );

            var handler = this.model.fetch();
            handler.bad = function() {
                var errors_handler = require( "errors_handler" );
                error_handler.oops( "Запрошенная группа не найдена." );
            }
            app.userState.set_current_group( this.model.get( "id" ) );
        },

        close: function() {
            this.stopListening( this.feeds );
        },

        render: function() {
            this.$el.html( this.template( this.model.toJSON() ) );
            this.$feeds_loader = $("#feeds-loader");

            var self = this;
            $("#feeds").visibility({
                once: false,
                observeChanges: true,
                initialCheck: false,
                onBottomVisible: function() {
                    if ( !self.no_more_content ) {
                        self.$feeds_loader.addClass( "active" );
                        self.model.need_more();
                    }
                }
            });
        },

        add: function( model ) {
            var views_factory = require( "group_feed/views_factory" );
            var View = views_factory( model.get("event_id") );
            var view = new View( {model: model} );
            $("#feeds").append( view.render().$el );
        },

        feeds_changed: function() {
            this.$feeds_loader.removeClass( "active" );
            $("#feeds").visibility( "refresh" );
            app.userState.fetch();
        },

        no_more: function() {
            this.no_more_content = true;
            $("#feeds-loader").removeClass( "active" );
        }

    });

    return GroupFeedView;
});

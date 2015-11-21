define( function( require ) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" ),
        user_tmpl = require( "template/user_for_add_view" );

    var UserView = Backbone.View.extend({

        template: Handlebars.templates.user_for_add_view,

        events: {
            "change .user-name": "user_changed",
            "click .remove-btn": "remove_clicked"
        },

        initialize: function() {
            this.listenTo( this.model, 'change', this.render );
            this.listenTo( this.model, 'destroy', this.remove );
            this.listenTo( this.model, 'removable_changed', this.removeable_changed );
        },

        render: function() {
            var as_html = this.template( this.model.toJSON() );
            this.$el.html( as_html );
            return this;
        },

        user_changed: function() {
            this.model.set({ name: this.$(".user-name").val() });
        },

        remove_clicked: function() {
            this.model.destroy();
        },

        removeable_changed: function( is ) {
            if ( is ) {
                this.$( ".remove-btn" ).removeClass( "disabled" );
            } else {
                this.$( ".remove-btn" ).addClass( "disabled" );
            }
        }
    });

    return UserView;
})

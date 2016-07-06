define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" ),
        preview_tmpl = require( "template/mail_view" )

    var MailView = Backbone.View.extend({

        template: Handlebars.templates.mail_view,

        initialize: function() {
            this.listenTo( this.model, 'change', this.render );
            this.listenTo( this.model, 'destroy', this.remove );
        },

        render: function() {
            this.$el = $( this.template( this.model.toJSON() ) );

            var self = this;
            this.$el.find( ".readed-btn" ).click( function() {
                self.readed( true );
            });

            this.$el.find( ".description a" ).click( function() {
                self.readed( false );
            });

            return this;
        },

        readed: function( need_update ) {
            if ( this.model.get("readed") == false ) {
                this.model.mark_as_readed( need_update );
            }
        },

    });

    return MailView;

})

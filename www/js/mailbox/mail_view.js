define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" ),
        preview_tmpl = require( "template/mail_view" )

    var MailView = Backbone.View.extend({

        template: Handlebars.templates.mail_view,

        events: {
            "click .readed-btn": "readed"
        },

        initialize: function() {
            this.listenTo( this.model, 'change', this.render );
            this.listenTo( this.model, 'destroy', this.remove );
        },

        render: function() {
            this.$el.html( this.template( this.model.toJSON() ) );
            return this;
        },

        readed: function() {
            this.model.mark_as_readed();
        },

    });

    return MailView;

})

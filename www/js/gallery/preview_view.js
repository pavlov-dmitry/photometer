define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" ),
        preview_tmpl = require( "template/photo_preview" )

    var PreviewView = Backbone.View.extend({

        template: Handlebars.templates.photo_preview,

        events: {
            // "click .rename-btn": "rename"
        },

        initialize: function() {
            this.listenTo( this.model, 'change', this.render );
            this.listenTo( this.model, 'destroy', this.remove );
        },

        render: function() {
            this.$el.html( this.template( this.model.toJSON() ) );
            return this;
        },

    });

    return PreviewView;

})

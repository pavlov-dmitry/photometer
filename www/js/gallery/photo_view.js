define( function( require ) {
    var Backbone = require( 'lib/backbone' ),
        Handlebars = require( 'handlebars.runtime' );
    require( 'template/photo_view' );

    var PhotoView = Backbone.View.extend({

        el: $( "#workspace" ),

        template: Handlebars.templates.photo_view,

        initialize: function() {
            this.listenTo( this.model, 'change', this.render );
        },

        render: function() {
            this.$el.html( this.template( this.model.toJSON() ) );
            return this;
        },

    });

    return PhotoView;
});

define( function( require ) {
    var Backbone = require( "lib/backbone" );
    var Handlebars = require( "handlebars.runtime" );
    require( "template/photo_edit_view" );

    var EditView = Backbone.View.extend({

        template: Handlebars.templates.photo_edit_view,

        events: {
        },

        initialize: function() {
            this.el = $( "#workspace" );
            this.model.on( "all", this.render, this );
            this.model.fetch();
        },

        render: function() {
            this.el.html( this.template( this.model.toJSON() ) );
            return this;
        },
    });

    return EditView;
});

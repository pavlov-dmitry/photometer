define( function( require ) {
    var Backbone = require( 'lib/backbone' ),
        Handlebars = require( "handlebars.runtime" );
    require( 'template/photo_view' );
    require( "handlebars_helpers" );
    var fit_image = require( "helpers/fit_image" );

    var PhotoView = Backbone.View.extend({

        el: $( "#workspace" ),

        template: Handlebars.templates.photo_view,

        initialize: function() {
            this.listenTo( this.model, 'change', this.render );

            this.fit_image_options = {
                img: "#photo",
                container: "#photo-container",
                height_coeff: 1,
                bottom_offset: 0,
                top_offset: 0
            };
            var self = this;
            this.resize_handler = function() {
                fit_image( self.fit_image_options );
            };
            this.model.fetch();
        },

        render: function() {
            this.$el.html( this.template( this.model.toJSON() ) );

            var photo = this.model.get("photo");
            this.fit_image_options.height_coeff = photo.height / photo.width;

            $(window).on( "resize", this.resize_handler );
            var self = this;
            $("#photo").on( "load", function() {
                $("#loader").remove();
                self.resize_handler();
                document.getElementById( 'photo' ).scrollIntoView();
            });
            return this;
        },

    });

    return PhotoView;
});

define( function( require ) {
    var Backbone = require( 'lib/backbone' ),
        Handlebars = require( "handlebars.runtime" ),
        CommentsModel = require( "comments/model" ),
        CommentsView = require( "comments/view" );
    require( 'template/photo_view' );
    require( "handlebars_helpers" );
    var fit_image = require( "helpers/fit_image" );

    var PhotoView = Backbone.View.extend({

        el: $( "#workspace" ),

        template: Handlebars.templates.photo_view,

        events: {
            "click #show-photo": "show_photo"
        },

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
            self.photo_loaded = false;
            $("#photo").on( "load", function() {
                self.photo_loaded = true;
                self.resize_handler();
                $("#loader").dimmer( "hide" );
                self.show_photo();
            });
            // если фотка не покажется в течении некоторого вермени, то показываем загрузку
            // setTimeout( function() {
                if ( !self.photo_loaded ) {
                    $("#loader").dimmer( { duration: {show: 0, hide: 0} } );
                    $("#loader").dimmer( "show" );
                }
            // }, 200 );
            self.resize_handler();

            var comments_model = new CommentsModel({ photo_id: this.model.get("id") });
            this.comments_view = new CommentsView({ model: comments_model });

            $(".ui.sticky").sticky();

            return this;
        },

        close: function( is_next_photo ) {
            $(window).off( "resize", this.resize_handler );
            if ( this.comments_view ) {
                this.comments_view.close();
            }
            if ( !is_next_photo ) {
                document.getElementById( 'main-menu' ).scrollIntoView();
            }
        },

        show_photo: function() {
            document.getElementById( 'photo-container' ).scrollIntoView();
        }

    });

    return PhotoView;
});

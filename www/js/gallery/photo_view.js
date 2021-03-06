define( function( require ) {
    var Backbone = require( 'lib/backbone' ),
        Handlebars = require( "handlebars.runtime" ),
        CommentsModel = require( "comments/model" ),
        CommentsView = require( "comments/view" )
        app = require("app");
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
            if ( !self.photo_loaded ) {
                $("#loader").dimmer( { duration: {show: 0, hide: 0} } );
                $("#loader").dimmer( "show" );
            }
            self.resize_handler();

            var photo_info = this.model.get("photo");
            var new_comments_page = Math.floor(
                ( photo.comments_count - photo.unreaded_comments ) / 10
            );
            var max_page = Math.max( 0, Math.ceil( photo.comments_count / 10 ) - 1 );
            new_comments_page = Math.min( new_comments_page, max_page );
            var comments_model = new CommentsModel({
                photo_id: this.model.get("id"),
                page: new_comments_page
            });
            this.comments_view = new CommentsView({ model: comments_model });

            $(".ui.sticky").sticky();

            var photo = this.model.get("photo");
            app.userState.navToGallery( photo.owner.id );

            return this;
        },

        close: function( is_next_photo ) {
            $(window).off( "resize", this.resize_handler );
            if ( this.comments_view ) {
                this.comments_view.close();
            }
            if ( !is_next_photo ) {
                var main_menu = document.getElementById( 'main-menu' );
                if ( main_menu ) {
                    main_menu.scrollIntoView();
                }
            }
        },

        show_photo: function() {
            document.getElementById( 'photo-container' ).scrollIntoView();
        }

    });

    return PhotoView;
});

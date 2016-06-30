define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" ),
        gallery_tmpl = require( "template/gallery_view" ),
        FilesUpload = require( "lib/jquery.fileupload" ),
        app = require( "app" ),
        make_upload_button = require("make_upload_button");

    var GalleryPreview = Backbone.View.extend({

        el: $( "#workspace" ),

        template: Handlebars.templates.gallery_view,
        pagination_tmpl: Handlebars.templates.pagination,
        context_url: "gallery_photo/",

        initialize: function() {
            this.listenTo( this.model, "change", this.render );
        },

        render: function() {
            this.$el.html( this.template( this.model.toJSON() ) );

            var self = this;
            make_upload_button( this, this.$el, "/upload", function() {
                self.model.fetch( 0 );
            });

            $(".image img").visibility({
                type: "image",
                transition: "fade in",
                duration: 500
            });

            var owner = this.model.get("owner");
            app.userState.navToGallery( owner.id );

            return this;
        },

        init_upload_button: function() {
            var self = this;
        },
    });

    return GalleryPreview;
})

define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" ),
        gallery_tmpl = require( "template/gallery_view" ),
        pagination_tmpl = require( "template/pagination" ),
        PreviewView = require( "gallery/preview_view" ),
        FilesUpload = require( "lib/jquery.fileupload" ),
        make_pagination = require( "make_pagination" ),
        app = require( "app" );

    var GalleryPreview = Backbone.View.extend({

        el: $( "#workspace" ),

        template: Handlebars.templates.gallery_view,
        pagination_tmpl: Handlebars.templates.pagination,
        context_url: "gallery_photo/",

        initialize: function() {
            var self = this;

            this.context_url = "gallery_photo/";// + app.user_id() + "/",

            this.listenTo( this.model, "add", this.addOne );
            this.listenTo( this.model, "reset", this.addAll );
            // this.listenTo( this.model, "all", this.render );
            this.listenTo( this.model, "pages_changed", this.pagesChanged );


            this.render();

            this.$upload_file.fileupload({

                url: "/upload",
                type: 'POST',
                paramName: "upload_img",
                limitMultiFileUploadSize: 3 * 1024 * 1024,

                start: function() {
                    self.$progress.show();
                    self.$progress_bar.css( 'width', '0%' );
                    self.$upload_btn.hide();
                },

                always: function() {
                    self.$progress.hide();
                    self.$upload_btn.show();
                },

                done: function() {
                    console.log( "upload done" );
                    self.model.fetch( 0 );
                },

                fail: function() {
                    var errorHandler = require( "errors_handler" );
                    errorHandler.error( "Не получилось загрузить файл, может попробуем еще раз?" );
                    console.log( "upload fail" );
                },

                progressall : function( e, data ) {
                    console.log( "progress loaded - " + data.loaded );
                    console.log( "progress total - " + data.total );
                    var progress = parseInt(data.loaded / data.total * 100, 10);
                    console.log( "progress - " + progress );
                    self.$progress_bar.css(
                        'width',
                        progress + '%'
                    );
                }
            })

            // this.model.fetch( this.page );
        },

        render: function() {
            this.$el.html( this.template({}) );

            this.$progress = $( "#upload-progress" );
            this.$progress_bar = $( "#upload-progress .progress-bar" );
            this.$upload_file = $( "#upload-file" );
            this.$upload_btn = $( "#upload-btn" );

            this.$progress.hide();
            return this;
        },

        addOne: function( data ) {
            console.log( JSON.stringify( data ) );
            var owner_id = data.get("owner_id");
            data.set({ url: this.context_url + owner_id + "/" + data.id });
            var view = new PreviewView({
                model: data,
                id: "preview-" + data.id
            });
            this.$("#preview-list").append( view.render().$el );
        },

        addAll: function() {
            this.$("#preview-list").empty();
            this.model.each( this.addOne, this );
        },

        pagesChanged: function( data ) {
            if ( 1 < data.pages_count ) {
                var pagination = make_pagination( data.current_page, data.pages_count, "#gallery/" );

                var content = this.pagination_tmpl( pagination );
                $("#header-pagination").html( content );
                $("#footer-pagination").html( content );
            }
        },

        addNewImageToGallery: function() {
            console.log( "add to gallery" );
        }

    });

    return GalleryPreview;
})

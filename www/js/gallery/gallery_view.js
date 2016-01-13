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
            this.listenTo( this.model, "update", this.check_on_empty );
            this.listenTo( this.model, "pages_changed", this.pagesChanged );


            this.render();

            this.$upload_file.fileupload({

                url: "/upload",
                type: 'POST',
                paramName: "upload_img",
                limitMultiFileUploadSize: 3 * 1024 * 1024,

                start: function() {
                    self.$progress.progress({ percent: 0 });
                    self.$progress.show();
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
                    self.$progress.progress({ percent: progress });
                }
            })

            // this.model.fetch( this.page );
        },

        render: function() {
            this.$el.html( this.template({}) );

            this.$progress = $( "#upload-progress" );
            this.$progress.progress();
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

        check_on_empty: function() {
            console.log( "check on empty" );
            if ( this.model.length == 0 ) {
                $("#preview-list").removeClass( "cards" );
                $("#preview-list").html(
                    "<div class=\"ui center aligned very padded basic segment\"><h1 class=\"ui disabled icon header\"><i class=\"film icon\"></i>Галлерея пуста.</h1><h3 class=\"ui disabled header\">Начните её заполнять, нажав на кнопку \"Добавить\", в левом верхнем углу.</h3></div>"
                );
            }
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

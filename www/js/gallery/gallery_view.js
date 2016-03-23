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

            this.context_url = "gallery_photo/";

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
                    self.model.fetch( 0 );
                },

                error: function( e ) {
                    var errorHandler = require( "errors_handler" );
                    if ( e.status == 413 ) {
                        errorHandler.error( "Слишком большой файл. Попробуй что нить меньше 2Мб." );
                    }
                    else if ( e.status == 400 ) {
                        if ( e.responseJSON.photo && e.responseJSON.photo == "bad_image" ) {
                            errorHandler.error( "Какая-то странная картинка, что это за формат такой? Уж извините, но мы такого не знаем. Попробуйте сохранить в Baseline JPEG." );
                        }
                        else {
                            errorHandler.error( "Что-то не так с загрузкой, но что не понятно. Пора пообщаться с разработчиком." );
                        }
                    }
                    else {
                        errorHandler.error( "Неизвестная ошибка. Пора пообщаться с разработчиком." );
                    }
                },

                progressall : function( e, data ) {
                    var progress = parseInt(data.loaded / data.total * 100, 10);
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
            this.$empty_gallery = $("#empty-gallery");

            this.$progress.hide();
            return this;
        },

        addOne: function( data ) {
            var owner = data.get("owner");
            data.set({ url: this.context_url + owner.id + "/" + data.id });
            var view = new PreviewView({
                model: data,
                id: "preview-" + data.id
            });
            this.$("#preview-list").append( view.render().$el );
            $(".image img").visibility({
                type: "image",
                transition: "fade in",
                duration: 500
            });
        },

        addAll: function() {
            this.$("#preview-list").empty();
            this.model.each( this.addOne, this );
        },

        check_on_empty: function() {
            if ( this.model.length == 0 ) {
                this.$empty_gallery.html(
                    "<div class=\"ui center aligned very padded basic segment\"><h1 class=\"ui disabled icon header\"><i class=\"film icon\"></i>Галлерея пуста.</h1><h3 class=\"ui disabled header\">Начните её заполнять, нажав на кнопку \"Добавить\", в левом верхнем углу.</h3></div>"
                );
            }
            else {
                this.$empty_gallery.empty();
            }
        },

        pagesChanged: function( data ) {
            if ( 1 < data.count ) {
                var pagination = make_pagination( data.current, data.count, "#gallery/" );

                var content = this.pagination_tmpl( pagination );
                $("#header-pagination").html( content );
                $("#footer-pagination").html( content );
            }
        },

        addNewImageToGallery: function() {
            // console.log( "add to gallery" );
        }

    });

    return GalleryPreview;
})

define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" ),
        gallery_tmpl = require( "template/gallery_view" ),
        FilesUpload = require( "lib/jquery.fileupload" ),
        app = require( "app" );

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

            this.$progress = $( "#upload-progress" );
            this.$progress.progress();
            this.$upload_file = $( "#upload-file" );
            this.$upload_btn = $( "#upload-btn" );

            this.init_upload_button();
            this.$progress.hide();

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
        },
    });

    return GalleryPreview;
})

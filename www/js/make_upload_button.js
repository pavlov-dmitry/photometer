define( function(require) {
    var make_upload_button = function( self, $el, url, done ) {
        self.$progress = $el.find( "#upload-progress" );
        self.$progress.progress();
        self.$upload_file = $el.find( "#upload-file" );
        self.$upload_btn = $el.find( "#upload-btn" );
        self.$progress.hide();

        self.$upload_file.fileupload({
            url: url,
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

            done: function( e, data ) {
                done( data.result );
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
        });
    };
    return make_upload_button;
})

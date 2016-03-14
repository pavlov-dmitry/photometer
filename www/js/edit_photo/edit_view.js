define( function( require ) {
    var Backbone = require( "lib/backbone" );
    var Handlebars = require( "handlebars.runtime" );
    require( "template/photo_edit_view" );
    require( 'jquery.imgareaselect' );
    var request = require( "request" );
    var errorsHandler = require( "errors_handler" );
    var fit_image = require( "helpers/fit_image" );
    var growl = require( "growl" );

    var EditView = Backbone.View.extend({

        el: "#workspace",

        template: Handlebars.templates.photo_edit_view,

        events: {
            "submit #rename-photo-form": "rename",
            "click #crop-btn": "crop",
        },

        initialize: function() {
            this.model.on( "change", this.render, this );
            this.model.fetch();

            var self = this;
            this.fit_image_options = {
                img: "#photo",
                container: "#img-container",
                height_coeff: 1,
                bottom_offset: 10
            };
            this.resize_handler = function() {
                self.fit_image();
            };
            this.cropping_now = false;
            this.renaming_now = false;
        },

        render: function() {
            this.$el.html( this.template( this.model.toJSON() ) );

            if ( !this.selection ) {
                var photo = this.model.get("photo");
                var halfSideSize = Math.min( photo.width, photo.height ) / 2;
                var halfImgWidth = photo.width / 2;
                var halfImgHeight = photo.height / 2;
                this.selection = {
                    x1: halfImgWidth - halfSideSize,
                    y1: halfImgHeight - halfSideSize,
                    x2: halfImgWidth + halfSideSize,
                    y2: halfImgHeight + halfSideSize
                };

                this.fit_image_options.height_coeff = photo.height / photo.width;
                this.fit_image_options.top_offset = $("#photo").offset().top;
                this.fit_image_options.bottom_offset = $("#crop-btn").outerHeight() + 2;
            }


            $(window).on( "resize", this.resize_handler );
            $("#photo").on( "load", this.resize_handler );

            return this;
        },

        rename: function() {
            if ( this.renaming_now )
                return;

            this.renaming_now = true;
            var self = this;
            $("#rename-photo-form").addClass( "loading");
            var handler = request.post( "/rename", {
                id: this.model.get( "id" ),
                name: $("#new-name-input").val()
            } );

            handler.good = function() {
                self.renaming_now = false;
                growl({
                    header: "Переименовано",
                    msg: "Новое имя задано для вашей фотографии",
                    positive: true
                }, "short" );
            };

            handler.bad = function( data ) {
                self.renaming_now = false;
                $form.removeClass( "loading" );
                var msg = "";
                if ( data.photo === "not_found" ) {
                    msg = "Опа, а такое фото не найдено! Что-то здесь не так.";
                } else {
                    msg = JSON.stringify( data );
                }
                errorsHandler.error( msg );
            };
            handler.finish = function() {
                $("#rename-photo-form").removeClass( "loading");
            }
        },

        enable_crop_btn: function() {
            var $btn = $("#crop-btn");
            $btn.removeClass( "loading" );
            $btn.removeClass( "disabled" );
            this.cropping_now = false;
        },
        disable_crop_btn: function() {
            var $btn = $("#crop-btn");
            $btn.addClass( "loading" );
            $btn.addClass( "disabled" );
            this.cropping_now = true;
        },

        crop: function() {
            if ( this.cropping_now )
                return;

            this.disable_crop_btn();
            var self = this;
            var handle = request.post( "/crop", {
                id: this.model.get( "id" ),
                x1: this.selection.x1,
                y1: this.selection.y1,
                x2: this.selection.x2,
                y2: this.selection.y2
            } );

            handle.good = function() {
                growl({
                    header: "Миниатюра готова",
                    msg: "Новая миниатюра для вашей фотографии подготовлена.",
                    positive: true
                }, "short" );
            }
            handle.bad = function( data ) {
                var msg = "";
                if ( data.photo === "not_found" ) {
                    msg = "Опа, а такое фото не найдено! Что-то здесь не так.";
                } else if ( data.photo == "bad_image" ) {
                    msg = "Опа, кажется фото испортилось =(, может не будем об этом никому рассказывать ?";
                } else {
                    msg = JSON.stringify( data );
                }
                errorsHandler.error( msg );
            }
            handle.finish = function() {
                self.enable_crop_btn();
            }
        },

        fit_image: function() {
            fit_image( this.fit_image_options );

            var self = this;
            var photo = this.model.get("photo");
            this.ias = $("img#photo").imgAreaSelect({
                instance: true,
                aspectRatio: "1:1",
                handles: true,
                imageWidth: photo.width,
                imageHeight: photo.height,
                x1: this.selection.x1,
                x2: this.selection.x2,
                y1: this.selection.y1,
                y2: this.selection.y2,
                onSelectEnd: function(img, selection) {
                    self.selection = selection;
                }
            });
        },

        // TODO: пообщаться с Саньком на счёт какого-то другого более нормального решения
        close: function() {
            if ( this.ias ) {
                this.ias.cancelSelection();
            }
            $(window).off( "resize", this.resize_handler );
        },
    });

    return EditView;
});

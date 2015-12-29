define( function( require ) {
    var Backbone = require( "lib/backbone" );
    var Handlebars = require( "handlebars.runtime" );
    require( "template/photo_edit_view" );
    require( 'jquery.imgareaselect' );
    var request = require( "request" );
    var errorsHandler = require( "errors_handler" );
    var fit_image = require( "helpers/fit_image" );

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
        },

        render: function() {
            this.$el.html( this.template( this.model.toJSON() ) );

            if ( !this.selection ) {
                var imgWidth = this.model.get( "width" );
                var imgHeight = this.model.get( "height" );
                var halfSideSize = Math.min( imgWidth, imgHeight ) / 2;
                var halfImgWidth = imgWidth / 2;
                var halfImgHeight = imgHeight / 2;
                this.selection = {
                    x1: halfImgWidth - halfSideSize,
                    y1: halfImgHeight - halfSideSize,
                    x2: halfImgWidth + halfSideSize,
                    y2: halfImgHeight + halfSideSize
                };

                this.fit_image_options.height_coeff = imgHeight / imgWidth;
                this.fit_image_options.bottom_offset = $("#crop-btn").outerHeight() + 2;
            }


            $(window).on( "resize", this.resize_handler );
            $("#photo").on( "load", this.resize_handler );

            return this;
        },

        rename: function() {
            var handler = request.post( "/rename", {
                id: this.model.get( "id" ),
                name: $("#new-name-input").val()
            } );

            handler.good = function() {
                $( "#rename-photo-form" ).addClass( "has-success" );
            };

            handler.bad = function( data ) {
                $( "#rename-photo-form" ).addClass( "has-error" );
                var msg = "";
                if ( data.photo === "not_found" ) {
                    msg = "Опа, а такое фото не найдено! Что-то здесь не так.";
                } else {
                    msg = JSON.stringify( data );
                }
                errorsHandler.error( msg );
            };
        },

        crop: function() {
            console.log( "need crop selection: " + JSON.stringify( this.selection ) );
            var handle = request.post( "/crop", {
                id: this.model.get( "id" ),
                x1: this.selection.x1,
                y1: this.selection.y1,
                x2: this.selection.x2,
                y2: this.selection.y2
            } );

            handle.good = function() {
                $( "#photo" ).addClass( "has-success" );
            }

            handle.bad = function( data ) {
                $( "#photo" ).addClass( "has-error" );

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
        },

        fit_image: function() {
            fit_image( this.fit_image_options );

            var self = this;
            this.ias = $("img#photo").imgAreaSelect({
                instance: true,
                aspectRatio: "1:1",
                handles: true,
                imageWidth: this.model.get( "width" ),
                imageHeight: this.model.get( "height" ),
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

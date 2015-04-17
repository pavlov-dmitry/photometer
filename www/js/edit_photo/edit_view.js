define( function( require ) {
    var Backbone = require( "lib/backbone" );
    var Handlebars = require( "handlebars.runtime" );
    require( "template/photo_edit_view" );

    var EditView = Backbone.View.extend({

        el: "#workspace",

        template: Handlebars.templates.photo_edit_view,

        events: {
            "submit #rename-photo-form": "rename",
        },

        initialize: function() {
            this.model.on( "all", this.render, this );
            this.model.fetch();
        },

        render: function() {
            this.$el.html( this.template( this.model.toJSON() ) );
            return this;
        },

        rename: function() {
            var request = require( "request" );
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
                var errorsHandler = require( "errors_handler" );
                errorsHandler.error( msg );
            };
        },
    });

    return EditView;
});

define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" ),
        PublicationModel = require( "event/action/publication/model" );
    require( "template/publication_action" );
    require( "handlebars_helpers" );

    var PublicationView = Backbone.View.extend({
        el: "#action",
        template: Handlebars.templates.publication_action,

        events: {
            "click .publish.button": "on_publish_clicked",
        },

        initialize: function() {
            this.listenTo( this.model, "change", this.on_action_model_change );
            this.publication_model = new PublicationModel();
            this.listenTo( this.publication_model, "change:photos", this.render );
        },

        close: function() {
            this.stopListening( this.publication_model );
        },

        on_action_model_change: function() {
            this.publication_model.set({ id: this.model.get("id") });
            this.publication_model.fetch( 0 );
        },

        render: function() {
            var html =  this.template( this.publication_model.toJSON() );
            this.$el.html( html );
            var self = this;
            $(".dimmable.image").dimmer({
                on: "hover"
            //     on: "click",
            //     onShow: function() {
            //         if ( self.$last_dimmed &&
            //              self.$last_dimmed.attr("id") != $(this).attr("id") )
            //         {
            //             self.$last_dimmed.dimmer( "hide" );
            //         }
            //         self.$last_dimmed = $(this);
            //     }
            });
            $(".pagination.button").click( function() {
                self.on_pagination( $(this).attr("data") );
            });
            return this;
        },

        on_publish_clicked: function( event ) {
            this.publish( $(event.target).attr("data") );
        },

        on_pagination: function( page ) {
            this.publication_model.fetch( page );
        },

        publish: function( photo_id ) {
            $( "#photos-container" ).addClass( "loading" );

            var self = this;
            var handler = this.publication_model.save( photo_id );
            handler.good = function() {
                self.model.finish();
            }
            handler.bad = function() {
                var growl = require( "growl" );
                growl({
                    header: "Ошибка",
                    msg: "Что-то не вышло опубликовать :( может поломалось что?",
                    negative: true
                });
            }
            handler.finish = function() {
                $( "#photos-container" ).removeClass( "loading" );
            }
        }
    });

    return PublicationView;
})

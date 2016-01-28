define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" ),
        CommentEditor = require( "comments/editor" );
    require( "template/one_comment_view");

    var OneCommentView = Backbone.View.extend({
        template: Handlebars.templates.one_comment_view,
        events: {
            // "click .quote.action": "quote"
        },

        initialize: function() {
            this.listenTo( this.model, "destroy", this.remove );
            this.listenTo( this.model, "cancel", this.on_cancel );
        },

        render: function() {
            var data = this.model.toJSON();
            var html = this.template( data );
            // console.log( "render one: " + html );
            // this.$el.replaceWith( html );
            this.$el = $( html );

            var self = this;
            this.$el.find(".quote.action").click( function() {
                console.log( "quote action" );
                self.model.quote();
            })
            this.$el.find(".edit.action").click( function() {
                self.edit_mode();
            });
            return this;
        },

        quote: function() {
            console.log( "quote event" );
            this.model.quote();
        },

        edit_mode: function() {
            this.model.start_edit();
            this.$el.find(".text-content").hide();
            this.$el.find(".actions").hide();
            var $editor = this.$el.find(".editor");
            if ( !this.editor ) {
                this.editor = new CommentEditor({
                    model: this.model,
                });
                $editor.append( this.editor.render().$el );
            }
            else {
                $editor.show();
                this.editor.render();
            }
        },

        on_cancel: function() {
            this.$el.find(".text-content").show();
            this.$el.find(".actions").show();
            this.$el.find(".editor").hide();
        }

    });

    return OneCommentView;
})

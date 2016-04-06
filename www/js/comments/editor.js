define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" );
    require( "template/comment_editor");
    require( "handlebars_helpers" );

    var Editor = Backbone.View.extend({
        template: Handlebars.templates.comment_editor,
        events: {
            "change .comment-body" : "on_comment_change",
            "keydown .comment-body": "on_comment_keydown",
            "click .edit.tab": "on_edit_tab",
            "click .preview.tab": "on_preview_tab",
            "submit .form": "on_submit",
            "click .cancel.button": "on_cancel"
        },

        initialize: function() {
            this.listenTo( this.model, "change:is_preview", this.render )
            this.listenTo( this.model, "reset", this.render )
        },

        close: function() {},

        render: function() {
            this.$el.html( this.template( this.model.toJSON() ) );
            this.$comment_body = this.$el.find(".comment-body");
            this.$form = this.$el.find( ".form" );
            this.$el.find(".markdown").popup({hoverable: true, popup: "#markdown-help"});
            return this;
        },

        on_comment_change: function() {
            this.model.set({ text: this.$comment_body.val() });
        },

        on_edit_tab: function() {
            this.model.set({ is_preview: false });
        },

        on_preview_tab: function() {
            this.model.set({ is_preview: true });
        },

        on_comment_keydown: function( e ) {
            var code = e.keyCode || e.which;
            if ( e.ctrlKey && code == 13 ) {
                this.on_comment_change();
                this.on_submit();
            }
        },

        on_submit: function() {
            var text = this.model.get("text");
            if ( text.length == 0 )
                return;

            this.$form.addClass( "loading" );
            var handler = this.model.save();
            var self = this;
            handler.finish = function() {
                self.$form.removeClass( "loading" );
            }
        },

        on_cancel: function() {
            this.model.cancel();
        }

    });

    return Editor;
})

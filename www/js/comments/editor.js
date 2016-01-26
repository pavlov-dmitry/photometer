define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" );
    require( "template/comment_editor");
    require( "handlebars_helpers" );

    var Editor = Backbone.View.extend({
        template: Handlebars.templates.comment_editor,
        events: {
            "change #comment-body" : "on_comment_change",
            "click .edit.tab": "on_edit_tab",
            "click .preview.tab": "on_preview_tab"
        },

        initialize: function() {
            this.listenTo( this.model, "change:is_preview", this.render )
        },

        close: function() {},

        render: function() {
            this.$el.html( this.template( this.model.toJSON() ) );
            this.$comment_body = $("#comment-body");
            return this;
        },

        on_comment_change: function() {
            this.model.set({ text: this.$comment_body.val() });
        },

        on_edit_tab: function() {
            console.log( "edit" );
            this.model.set({ is_preview: false });
        },

        on_preview_tab: function() {
            console.log( "preview" );
            this.model.set({ is_preview: true });
        }
    });

    return Editor;
})

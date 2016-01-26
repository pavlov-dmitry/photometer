define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" ),
        OneCommentView = require( "comments/one_comment_view" ),
        CommentEditor = require( "comments/editor" );
    require( "template/comments_view" );

    var CommentsView = Backbone.View.extend({
        el: "#comments",
        template: Handlebars.templates.comments_view,

        initialize: function() {
            this.listenTo( this.model, "change:pagination", this.render );
            this.comments = this.model.get("comments");
            this.listenTo( this.comments, "add", this.add );
            this.model.fetch();
            this.editor = new CommentEditor({ model: this.model, el: "#comment-editor" });
            this.editor.render();
        },

        close: function() {
            this.stopListening( this.comments );
            if ( this.editor && this.editor.close ) {
                this.editor.close();
            }
        },

        render: function() {
            var html = this.template( this.model.toJSON() );
            this.$el.html( html );
            this.$comments_list = $("#comments-list");
            return this;
        },

        add: function( model ) {
            var view = new OneCommentView({ model: model });
            this.$comments_list.append( view.render().$el );
        }
    });

    return CommentsView;
});

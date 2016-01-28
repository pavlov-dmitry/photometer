define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" ),
        OneCommentView = require( "comments/one_comment_view" ),
        CommentEditor = require( "comments/editor" );
    require( "template/comments_view" );
    require( "handlebars_helpers" );

    var CommentsView = Backbone.View.extend({
        el: "#comments",
        template: Handlebars.templates.comments_view,

        initialize: function() {
            this.listenTo( this.model, "change:pagination", this.render );
            this.comments = this.model.get("comments");
            this.listenTo( this.comments, "add", this.add );
            this.listenTo( this.comments, "reset", this.reset );
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
            console.log( "render" );
            var html = this.template( this.model.toJSON() );
            this.$el.html( html );
            this.$comments_list = $("#comments-list");

            var self = this;
            this.comments.forEach( function( model ) {
                self.add( model );
            })
            this.$el.find( ".pagination.button" ).click( function() {
                self.pagination( $(this).attr("data") );
            })
            return this;
        },

        add: function( model ) {
            var view = new OneCommentView({ model: model });
            this.listenTo( model, "quote", this.on_quote );
            this.listenTo( model, "edited", this.on_edited );
            this.$comments_list.append( view.render().$el );
            // view.render();
        },

        reset: function() {
            this.$comments_list.empty();
        },

        pagination: function( page ) {
            this.model.fetch( page );
        },

        on_quote: function( text ) {
            console.log( "on_quote:" + text );
            var self_text = this.model.get("text");
            self_text += "\n" + text + "\n";
            this.model.set({ text: self_text });
            this.model.trigger("reset");
        },

        on_edited: function() {
            this.model.fetch();
        }
    });

    return CommentsView;
});

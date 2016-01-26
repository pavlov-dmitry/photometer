define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" );
    require( "template/one_comment_view");

    var OneCommentView = Backbone.View.extend({
        template: Handlebars.templates.one_comment_view,

        initialize: function() {
            this.listenTo( this.model, "destroy", this.remove );
        },

        render: function() {
            var data = this.model.toJSON();
            this.$el.replaceWith( this.template( data ) );
            return this;
        }
    });

    return OneCommentView;
})

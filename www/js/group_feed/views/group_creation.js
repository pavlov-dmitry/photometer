define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" );
    require( "template/group_creation_feed_view" );

    var GroupCreationFeedView = Backbone.View.extend({
        render: function() {
            var data = this.model.toJSON();
            var html = Handlebars.templates.group_creation_feed_view( data );
            this.$el = $( html );
            return this;
        }
    });
    return GroupCreationFeedView;
});

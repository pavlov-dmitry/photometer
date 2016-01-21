define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" );
    require( "template/group_voting_feed_view" );

    var GroupVotingFeedView = Backbone.View.extend({
        render: function() {
            var data = this.model.toJSON();
            data.data = JSON.parse( data.data );
            var html = Handlebars.templates.group_voting_feed_view( data );
            this.$el = $( html );
            return this;
        }
    });
    return GroupVotingFeedView;
});

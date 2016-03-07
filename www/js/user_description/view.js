define( function( require ) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" );
    require( "template/user_description" );

    var UserDescriptionView = Backbone.View.extend({
        el: "#workspace",

        template: Handlebars.templates.user_description,

        initialize: function() {
            this.listenTo( this.model, 'change', this.render );
            this.model.fetch();
        },

        render: function() {
            var html = this.template( this.model.toJSON() );
            this.$el.html( html );
        }

    });

    return UserDescriptionView;
})

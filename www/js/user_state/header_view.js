define( function( require ) {
    var Backbone = require( 'lib/backbone' ),
    Handlebars = require( 'handlebars.runtime' );
    require( "template/user_state_header_view" );
    var app = require( "app" );

    var UserStateHeaderView = Backbone.View.extend( {

        template: Handlebars.templates.user_state_header_view,

        events: {
            "click #logout-action" : "logout"
        },

        initialize: function() {
            $el = $("#navbar");
            this.model.on( "change", this.render, this );
            this.render();
        },

        render: function() {
            $el.html( this.template( this.model.toJSON() ) );
            $(".ui.dropdown").dropdown();
        }

    } );

    return UserStateHeaderView;
} );

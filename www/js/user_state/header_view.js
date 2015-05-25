define( function( require ) {
    var Backbone = require( 'lib/backbone' ),
    Handlebars = require( 'handlebars.runtime' );
    require( "template/user_state_header_view" );

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

            // NOTE: отчего-то через таблицу событий Backbone events
            // не вышло привзяться к событию на нажатие, потому
            // привязываюсь после рендера вручную
            var self = this;
            $("#logout-action").click( function() {
                self.logout();
            } );
        },

        logout: function() {
            var app = require( "app" );
            app.logout();
        }

    } );

    return UserStateHeaderView;
} );

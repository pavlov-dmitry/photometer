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
            $("#user_menu").dropdown();
            if ( this.model.get( "is_many_groups") ) {
                $("#groups_menu").dropdown();
            }

            $("img.logo").popup({
                on: "click",
                title: "Посвящение",
                html: "Создание данного сайта посвящается моей, горячо любимой, Танюшечке-Лапотушечке <i class=\"red heart icon\"></i> верящей что я всё смогу, и всё у меня получится."
            });
        }

    } );

    return UserStateHeaderView;
} );

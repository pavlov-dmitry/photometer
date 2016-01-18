define ( function( require ) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" );
    require( "template/group_view" );
    require( "handlebars_helpers" );

    var GroupView = Backbone.View.extend({
        el: "#workspace",

        template: Handlebars.templates.group_view,

        events: {
        },

        initialize: function() {
            this.model.on( "change", this.render, this );
            var handler = this.model.fetch();
            handler.bad = this.process_error;
        },

        render: function() {
            this.$el.html( this.template( this.model.toJSON() ) );
            $("#edit-btn").dropdown();
            var app = require( "app" );
            app.userState.set_current_group( this.model.get( "id" ) );
            app.userState.navToGroup();
        },

        process_error: function( data ) {
            var errors_handler = require( "errors_handler" );
            if ( data.field == "group" && data.reason == "not_found" ) {
                errors_handler.oops( "Ошибка запроса группы", "Запрашиваемая группа не найдена.");
            } else {
                errors_handler.oops( "Ошибка", "Неизвестная ошибка: " + JSON.stringify( data ) );
            }
        }
    });

    return GroupView;
})

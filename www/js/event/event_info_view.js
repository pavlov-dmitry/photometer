define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" ),
        event_info_tmpl = require( "template/event_info" ),
        events_info_collection = require( "event/events_infos" ),
        actions = require( "event/actions" );

    var EventInfoView = Backbone.View.extend({
        el: '#workspace',

        template: Handlebars.templates.event_info,

        initialize: function() {
            this.model.on( "change", this.render, this );
            this.model.fetch();
        },

        render: function() {
            var data = this.model.toJSON();
            var info = events_info_collection( data.id );

            data.name = info.caption( data.name );
            data.state = this.to_state( data.state );
            var html_content = this.template( data );
            this.$el.html( html_content );

            var description_data = JSON.parse( data.description );
            $("#description").html( info.template( description_data ) );

            var ActionView = actions( data.action );
            var action_data = {
                id: this.model.scheduled_id(),
                answer: info.answer,
            }
            this.action_view = new ActionView();
            this.action_view.init( action_data );
        },

        to_state: function( state ) {
            switch ( state ) {
            case "Disabled": return {label:"danger", text:"Отключено"};
            case "NotStartedYet": return {label:"info", text:"Пока не активно"};
            case "Active": return {label:"success", text:"Активно"};
            case "Finished": return {label:"default", text:"Завершено"};
            default: return state
            }
        },

        close: function() {
            if ( this.action_view ) {
                this.action_view.undelegateEvents();
            }
        }
    });

    return EventInfoView;
})

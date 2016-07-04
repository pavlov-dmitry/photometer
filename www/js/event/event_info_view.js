define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" ),
        ActionModel = require( "event/action_model" ),
        CommentsModel = require( "comments/model" ),
        CommentsView = require( "comments/view" ),
        event_info_tmpl = require( "template/event_info" ),
        events_info_collection = require( "event/events_infos" ),
        actions = require( "event/actions" ),
        app = require( "app" );

    var EventInfoView = Backbone.View.extend({
        el: '#workspace',

        template: Handlebars.templates.event_info,

        initialize: function() {
            this.model.on( "change", this.render, this );
            this.model.fetch();
            this.action_model = new ActionModel();
            this.listenTo( this.action_model, "finished", this.on_finish );
        },

        render: function() {
            var data = this.model.toJSON();
            var info = events_info_collection( data.id );
            var description_data = JSON.parse( data.description );

            data.name = info.caption( data.name, description_data );
            data.state = this.to_state( data.state );
            var html_content = this.template( data );
            this.$el.html( html_content );

            $("#description").html( info.makeHtml( description_data ) );


            var ActionView = actions( data.action );
            var action_data = {
                id: this.model.scheduled_id(),
                answer: info.answer,
            }
            this.action_view = new ActionView({ model: this.action_model });
            this.action_model.set( action_data );

            var comments_model = new CommentsModel({
                event_id: this.model.get("scheduled_id")
            });
            this.comments_view = new CommentsView({ model: comments_model });
        },

        to_state: function( state ) {
            switch ( state ) {
            case "Disabled": return {color:"black", text:"Отключено"};
            case "NotStartedYet": return {color:"yellow", text:"Пока не активно"};
            case "Active": return {color:"green", text:"Активно"};
            case "Finished": return {color:"grey", text:"Завершено"};
            default: return state
            }
        },

        on_finish: function() {
            app.userState.fetch();
            this.model.fetch();
        },

        close: function() {
            this.stopListening( this.action_model );
            if ( this.comments_view ) {
                this.comments_view.close();
            }
            if ( this.action_view ) {
                this.action_view.undelegateEvents();
                if ( this.action_view.close ) {
                    this.action_view.close();
                }
            }
        }
    });

    return EventInfoView;
})

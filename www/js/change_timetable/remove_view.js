define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" );
    require( "template/remove_from_timetable" );

    var RemoveView = Backbone.View.extend({
        template: Handlebars.templates.remove_from_timetable,

        events: {
            "click .remove.button": "on_remove_clicked"
        },

        initialize: function() {
            this.listenTo( this.model, 'destroy', this.remove );
        },

        render: function() {
            // this.$el = $( this.template( this.model.toJSON() ) );
            var html = this.template( this.model.toJSON() );
            this.$el.html( html );
            return this;
        },

        on_remove_clicked: function() {
            this.model.destroy();
        }
    });
    return RemoveView;
})

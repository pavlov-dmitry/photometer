define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" );
    require( "template/add_to_timetable" );
    require( "jquery.datetimepicker");

    $.datetimepicker.setLocale('ru');

    var AddToTimetableView = Backbone.View.extend({
        template: Handlebars.templates.add_to_timetable,

        events: {
            "click .remove.button": "on_remove_clicked"
        },

        initialize: function() {
            this.listenTo( this.model, 'destroy', this.remove );
        },

        render: function() {
            var as_html = this.template( this.model.toJSON() );
            this.$el.html( as_html );
            this.$el.find(".datetimepicker-input").datetimepicker();
            return this;
        },

        on_remove_clicked: function() {
            this.model.destroy();
        }
    });

    return AddToTimetableView;
})

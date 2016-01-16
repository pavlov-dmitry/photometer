define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" ),
        moment = require( "moment" );
    require( "template/add_to_timetable" );
    require( "jquery.datetimepicker");

    var AddToTimetableView = Backbone.View.extend({
        template: Handlebars.templates.add_to_timetable,

        events: {
            "click .remove.button": "on_remove_clicked",
            "change .name-input": "on_name_changed",
            "change .datetimepicker-input": "on_datetime_changed"
        },

        initialize: function() {
            this.listenTo( this.model, 'destroy', this.remove );
        },

        render: function( idx, highlighted ) {
            var data = this.model.toJSON();
            data.idx = idx;
            this.idx = idx;
            this.$el.html( this.template( data ) );
            this.$datetimepicker = this.$el.find(".datetimepicker-input");
            this.$datetimepicker.datetimepicker({
                minDate: 0,
                weeks: true,
                dayOfWeekStart: 1,
                format: "H:i D, d M Y",
                highlightedDates: highlighted
            });
            this.$name_input = this.$el.find( ".name-input" );
            return this;
        },

        on_remove_clicked: function() {
            this.model.destroy();
        },

        on_name_changed: function() {
            var name = this.$name_input.val();
            this.model.set({ name: name });
        },

        on_datetime_changed: function() {
            var datetime = this.$datetimepicker.val();

            var datetime_value = moment( datetime, "HH:mm ddd, DD MMM YYYY" );
            if ( datetime_value.isValid() ) {
                this.model.set({ time: datetime_value.valueOf() });
            }
        }
    });

    return AddToTimetableView;
})

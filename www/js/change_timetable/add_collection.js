define( function(require) {
    var Backbone = require( "lib/backbone" ),
        AddModel = require( "change_timetable/add_model" );

    var AddToTimetableCollection = Backbone.Collection.extend({
        model: AddModel,

        add_one_more: function() {
            this.add( new AddModel() );
        }
    });

    return AddToTimetableCollection;
});

define( function(require) {
    var Backbone = require( "lib/backbone" ),
        RemoveModel = require( "change_timetable/remove_model" );

    var RemoveFromTimetableCollection = Backbone.Collection.extend({
        model: RemoveModel,

        try_add: function( data ) {
            var found = this.find( function( v ) {
                return v.get( "scheduled_id" ) === data.scheduled_id
            });
            if ( !found ) {
                this.add( data );
            }
        }
    });

    return RemoveFromTimetableCollection;
});

define( function(require) {
    var Backbone = require( "lib/backbone" ),
        AddCollection = require( "change_timetable/add_collection" ),
        request = require( "request" );

    var ChangeTimetableModel = Backbone.Model.extend({
        defaults: {
            group_id: 0,
            add: new AddCollection(),
            remove: [],
            description: ""
        },

        save: function( group_id ) {
            var url = "events/group/" + group_id + "/create/change_timetable";
            return request.post( url, this.toJSON() );
        }
    });

    return ChangeTimetableModel;
})

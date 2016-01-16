define( function(require) {
    var Backbone = require( "lib/backbone" ),
        AddCollection = require( "change_timetable/add_collection" ),
        RemoveCollection = require( "change_timetable/remove_collection" ),
        request = require( "request" );

    var ChangeTimetableModel = Backbone.Model.extend({
        defaults: {
            group_id: 0,
            add: new AddCollection(),
            remove: new RemoveCollection(),
            description: ""
        },

        save: function( group_id ) {
            var url = "events/group/" + group_id + "/create/change_timetable";
            var data = this.toJSON();
            data.remove = _.map( data.remove.models, function( m ) {
                return m.get("scheduled_id");
            });
            return request.post( url, data );
        }
    });

    return ChangeTimetableModel;
})

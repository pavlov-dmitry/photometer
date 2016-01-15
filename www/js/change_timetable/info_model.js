define( function(require) {
    var Backbone = require( "lib/backbone" ),
        request = require( "request" ),
        errors_handler = require( "errors_handler" );

    var ChangeTimetableInfoModel = Backbone.Model.extend({
        defaults: {
            group_name: ""
        },

        fetch: function( group_id ) {
            var url = "events/group/" + group_id + "/create/change_timetable";
            var handler = request.get( url );

            var self = this;
            handler.good = function( data ) {
                self.set( data );
            }
            return handler;
        }
    });

    return ChangeTimetableInfoModel;
})

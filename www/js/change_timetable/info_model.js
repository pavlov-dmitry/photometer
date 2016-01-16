define( function(require) {
    var Backbone = require( "lib/backbone" ),
        request = require( "request" ),
        errors_handler = require( "errors_handler" ),
        moment = require( "moment" );

    var ChangeTimetableInfoModel = Backbone.Model.extend({
        defaults: {
            group_name: ""
        },

        fetch: function( group_id ) {
            var url = "events/group/" + group_id + "/create/change_timetable";
            var handler = request.get( url );

            var self = this;
            handler.good = function( data ) {
                data.current = _.map( data.current, function( v ) {
                    v.date_str = moment( v.ending_time ).format( "YYYY/MM/DD" );
                    return v;
                });
                self.set( data );
            }
            return handler;
        }
    });

    return ChangeTimetableInfoModel;
})

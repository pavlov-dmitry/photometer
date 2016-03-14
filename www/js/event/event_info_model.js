define( function(require) {
    var Backbone = require( "lib/backbone" )

    var EventInfoModel = Backbone.Model.extend({
        defaults: {
            scheduled_id: 0
        },

        start_url: "/event/",

        fetch: function() {
            var self = this;
            var request = require( "request" );
            var url = this.start_url + this.scheduled_id();
            var handler = request.get( url, {});
            handler.good = function( data ) {
                self.set( data );
            }
            return handler;
        },

        scheduled_id: function() {
            return this.get("scheduled_id");
        }
    });


    return EventInfoModel;
})

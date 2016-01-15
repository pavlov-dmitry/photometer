define( function(require) {
    var Backbone = require( "lib/backbone" );

    var AddToTimetableModel = Backbone.Model.extend({
        defaults: {
            event_id: "Publication",
            name: "",
            time: "",
            params: ""
        }
    });

    return AddToTimetableModel;
})

define ( function( require ) {
    var Backbone = require( "lib/backbone" );

    var GroupModel = Backbone.Model.extend({
        defaults: {
            id: 0,
            name: "Имя группы"
        },

        fetch: function() {
            var request = require( "request" );
            var handler = request.get( "group/info", { group_id: this.get("id") });
            var self = this;
            handler.good = function( data ) {
                self.set( data );
            }

            return handler;
        }
    });

    return GroupModel;
})

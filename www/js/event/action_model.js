define( function(require) {
    var Backbone = require( "lib/backbone" );
    var ActionModel = Backbone.Model.extend ({
        defaults: {},

        finish: function() {
            this.trigger( "finished" );
        }
    });
    return ActionModel;
});

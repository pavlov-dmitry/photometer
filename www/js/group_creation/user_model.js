define ( function( require ) {
    var Backbone = require( "lib/backbone" );

    var UserModel = Backbone.Model.extend({
        defaults: {
            name: ""
        }
    });

    UserModel.empty_model = function() {
        return { name: "" };
    };

    return UserModel;
})

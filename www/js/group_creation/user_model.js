define ( function( require ) {
    var Backbone = require( "lib/backbone" );

    var UserModel = Backbone.Model.extend({
        defaults: {
            name: ""
        },

        set_removeable: function( is ) {
            this.trigger( "removable_changed", is );
        }
    });

    UserModel.empty_model = function() {
        return { name: "" };
    };

    return UserModel;
})

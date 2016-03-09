define ( function( require ) {
    var Backbone = require( "lib/backbone" );

    var UserModel = Backbone.Model.extend({
        defaults: {
            id: 0
        },

        set_removeable: function( is ) {
            this.trigger( "removable_changed", is );
        }
    });

    UserModel.empty_model = function() {
        return { id: 0 };
    };

    return UserModel;
})

define( function(require) {
    var Backbone = require( "lib/backbone" );

    var RegisterModel = Backbone.Model.extend({
        defaults: {
            'name': '',
            'password': '',
            'email': ''
        }
    });

    return RegisterModel;
})
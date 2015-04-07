define( function(require) {
    var Backbone = require( "lib/backbone" );

    var PhotoModel = Backbone.Model.extend({
        defaults: {
            'id': 0,
            'name': "нет имени"
        },
    })

    return PhotoModel;
})
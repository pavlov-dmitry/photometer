define( function(require) {
    var Backbone = require( "lib/backbone" );

    var MailModel = Backbone.Model.extend({
        defaults: {
            'id': 0,
            'creation_time': 0,
            'sender_name': "sender",
            'subject': "subject",
            'body': "the Mail",
            'readed': false
        },

        // fetch: function() {
        //     var self = this;
        //     var Request = require( "request" );
        //     var handler = Request.get( /photo_info/ + this.id );
        //     handler.good = function( data ) {
        //         self.set( data );
        //     }

        //     handler.bad = function( data ) {}
        // }
    })

    return MailModel;
})

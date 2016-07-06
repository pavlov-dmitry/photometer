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

        mark_as_readed: function( need_update ) {
            var self = this;
            var request = require( "request" );
            var handler = request.post( "/mailbox/mark_as_readed", { id: this.id } );
            handler.good = function( data ) {
                self.set({ readed: true });
                if ( need_update ) {
                    self.trigger( "marked" );
                }
            }
        }
    })

    return MailModel;
})

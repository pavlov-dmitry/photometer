define( function(require) {
    var Backbone = require( "lib/backbone" ),
        MailModel = require( "mailbox/mail_model" ),
        request = require( "request" );

    var MailsCollection = Backbone.Collection.extend({
        model: MailModel,
        is_only_unreaded: false,
        current_page: 0,

        fetch: function( page ) {
            var self = this;
            current_page = page;

            var url = "/mailbox";
            if ( this.is_only_unreaded ) {
                url += "/unreaded";
            }

            var handler = request.get( url, { page: page } );
            handler.good = function( data ) {
                self.reset();

                self.add( data.mails );

                self.trigger( "update" );
                self.trigger( "pages_changed", data.pagination );
            };

            handler.bad = function( data ) {
                var error_handler = require( "errors_handler" );
                error_handler.oops( "Не смог загрузить сообщения", JSON.stringify( data ) );
            }
        },
    });

    return MailsCollection;
})

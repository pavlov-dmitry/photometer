define( function(require) {
    var Backbone = require( "lib/backbone" ),
    MailModel = require( "mailbox/mail_model" ),
    Request = require( "request" );

    var MailsCollection = Backbone.Collection.extend({
        model: MailModel,

        fetch: function( page, is_unreaded ) {
            var self = this;
            var url = "/mailbox";
            if ( is_unreaded ) {
                url += "/unreaded";
            }

            var handler = Request.get( url, { page: page } );
            handler.good = function( data ) {
                self.reset();

                self.add( data.mails );

                self.trigger( "pages_changed", {
                    pages_count: data.pages_count,
                    current_page: data.current_page
                });
            };

            handler.bad = function( data ) {
                var errorHandler = require( "errors_handler" );
                errorHandler.oops( "Не смог загрузить сообщения", JSON.stringify( data ) );
            }
        }
    });

    return MailsCollection;
})

define( function(require) {
    var Backbone = require( "lib/backbone" ),
        MailModel = require( "mailbox/mail_model" ),
        Request = require( "request" ),
        markdown = require( 'showdown_converter' );

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

            var handler = Request.get( url, { page: page } );
            handler.good = function( data ) {
                self.reset();

                // преобзовываем письма из формата markdown в формат html
                _.each( data.mails, function(mail) {
                    mail.body = markdown.makeHtml( mail.body );
                });

                self.add( data.mails );

                self.trigger( "pages_changed", {
                    pages_count: data.pages_count,
                    current_page: data.current_page
                });
            };

            handler.bad = function( data ) {
                var error_handler = require( "errors_handler" );
                error_handler.oops( "Не смог загрузить сообщения", JSON.stringify( data ) );
            }
        },
    });

    return MailsCollection;
})

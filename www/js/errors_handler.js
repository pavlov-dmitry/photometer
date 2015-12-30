define( function( require ) {
    var $ = require( "jquery" );
    var Handlebars = require( "handlebars.runtime" );
    var growl = require( "growl" );

    var errorsHandler = {
        /// обработка ошибок сервера
        processInternalError: function( response, ajax ) {
            require( "template/dev_error" );

            $( "#workspace" ).html( Handlebars.templates.dev_error( {
	        ajax: ajax,
	        response: response
            }));
        },

        oops: function( header, text ) {
            require( "template/text_error" );

            var template = Handlebars.templates.text_error;
            $("#workspace").html( template( {
                header: header,
                text: text
            } ) );
        },

        error: function( text ) {
            growl({ header: "Ошибка",
                    msg: text,
                    negative: true
                  },
                  5000);

            // require( "template/closeable_error" );

            // var template = Handlebars.templates.closeable_error;
            // var newError = $( template({ text: text }) );
            // $("#errors").append( newError );
        },

    };

    return errorsHandler;
});

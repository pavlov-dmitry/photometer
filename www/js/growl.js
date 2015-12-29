define( function(require) {
    var Handlebars = require( "handlebars.runtime" );
    require( "template/growl_msg")
    var counter = 0;

    var growl = function( content, interval ) {

        counter += 1;
        content.id = "groul-msg-" + counter;

        var msg_html = Handlebars.templates.growl_msg( content );
        $("#growl").prepend( msg_html );

        var close = function( $el ) {
            $el.transition({
                animation: 'fly left',
                onComplete: function() {
                    $el.remove();
                }
            })
        }
        $('.message .close').on('click', function() {
            close( $(this).closest('.row') );
        });

        var msg_selector = "#" + content.id;
        if ( interval ) {
            window.setTimeout(
                function() {
                    close( $(msg_selector) );
                },
                interval
            );
        }
        $(msg_selector).transition('fly left');
    }
    return growl;
});

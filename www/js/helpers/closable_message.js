define( function(require) {
    var closable_message = function() {
        $('.message .close').on('click',
                                function() {
                                    $(this)
                                        .closest('.message')
                                        .transition('fade');
                                });
    };
    return closable_message;
});

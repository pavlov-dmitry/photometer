define( function(require) {
    var fit_image = function( options ) {
        var $photo = $(options.img);

        var max_width = $(options.container).width();
        var max_height = $(window).height() - options.top_offset - options.bottom_offset;
        var max_height_by_width = max_width * options.height_coeff;

        var width = max_width;
        var height = max_height_by_width;
        if ( max_height < max_height_by_width )
        {
            height = max_height;
            width = max_height / options.height_coeff;
        }

        $photo.height( height );
        $photo.width( width );
    };
    return fit_image;
});

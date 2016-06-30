define( function( require ) {
    var Showdown = require( 'showdown' );
    var emojify = require( 'emojify' );
    emojify.setConfig({
        img_dir: "i/emoji",
        ignored_tags     : {
            'SCRIPT'  : 1,
            'TEXTAREA': 1,
            'A'       : 1,
            'PRE'     : 1,
            'CODE'    : 1
        }
    });

    var converter = new Showdown.converter();
    var my_converter = {
        makeHtml: function( data ) {
            var result = data
            result = result.replace(/(@\w+)/g, '**$1**' );
            result = emojify.replace( result );
            result = converter.makeHtml( result );
            return result;
        }
    }
    return my_converter;
})

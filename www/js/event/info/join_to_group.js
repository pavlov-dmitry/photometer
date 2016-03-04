
define( function(require) {
    var Handlebars = require( "handlebars.runtime" );
    require( "template/join_to_group_info" );

    var change_timetable_info = {
        makeHtml: function( data ) {
            return Handlebars.templates.join_to_group_info( data );
        }
    };
    return change_timetable_info;
});

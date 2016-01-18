define( function(require) {
    var Handlebars = require( "handlebars.runtime" );
    require( "template/change_timetable_info" );
    require( "template/timetable_value_info" );

    var change_timetable_info = {
        caption: function( name ) {
            return "Изменение расписания в группе '" + name + "'";
        },
        makeHtml: function( data ) {
            return Handlebars.templates.change_timetable_info( data );
        }
    };
    return change_timetable_info;
});

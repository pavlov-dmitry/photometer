define( function(require) {
    Handlebars = require( "handlebars.runtime" );
    require( "handlebars_helpers" );
    require( "template/group_creation_info" );

    var group_creation_info = {
        caption: function( name ) {
            return "Создание группы \"" + name + "\"";
        },
        answer: "Вы согласны присоединиться к этой группе?",
        makeHtml: function( data ) {
            return Handlebars.templates.group_creation_info( data );
        }
    };

    return group_creation_info;
})

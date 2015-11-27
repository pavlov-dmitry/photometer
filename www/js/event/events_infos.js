define( function(require) {
    var Handlebars = require( "handlebars.runtime" );
        var markdown = require( "showdown_converter" );

    Handlebars.registerHelper( "markdown", function( data ) {
        return markdown.makeHtml( data );
    });

    var events_info_collection = function( id ) {
        var result  = {
            caption: function( name ) {
                return name;
            },
            template: function( data ) {
                return data;
            },
            answer: "Вы согласны?"
        }

        switch ( id ) {

        case 2:// Group creation
            var tmpl = require( "template/group_creation_info" );
            result.caption = function( name ) {
                return "Создание группы <b>" + name + "</b>";
            };
            result.template = Handlebars.templates.group_creation_info;
            result.answer = "Вы согласны присоедениться к этой группе?";
            break;

        }

        return result;
    }

    return events_info_collection;
})

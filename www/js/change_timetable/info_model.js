define( function(require) {
    var Backbone = require( "lib/backbone" ),
        request = require( "request" ),
        errors_handler = require( "errors_handler" );

    var ChangeTimetableInfoModel = Backbone.Model.extend({
        defaults: {
            group_name: ""
        },

        fetch: function( group_id ) {
            var url = "events/group/" + group_id + "/create/5";
            var handler = request.get( url );

            var self = this;
            handler.good = function( data ) {
                self.set( data );
            }
            handler.bad = function() {
                errors_handler.oops(
                    "Ошибка запроса информации о группе",
                    "Что-то группа которую вы запрашиваете, не найдена, то-ли что-то пошло не так, то-ли кто-то что-то не то запрашивает. В общем фиг его знает, если вы не мудрили с адресной строкой, то похоже стоит обратиться к разработчикам."
                )
            }
        }
    });

    return ChangeTimetableInfoModel;
})

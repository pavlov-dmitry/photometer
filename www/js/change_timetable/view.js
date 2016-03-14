define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" ),
        AddView = require( "change_timetable/add_view" ),
        RemoveView = require( "change_timetable/remove_view" ),
        ChangeTimetableInfoModel = require( "change_timetable/info_model" ),
        markdown = require( 'showdown_converter' ),
        growl = require( "growl" ),
        errors_handler = require( "errors_handler" ),
        moment = require( "moment" );
    require( "template/change_timetable" );


    var ChangeTimetableView = Backbone.View.extend({

        el: "#workspace",
        template: Handlebars.templates.change_timetable,

        events: {
            "click #new-add-btn": "on_need_new_add",
            "change #description-input": "description_changed",
            "keyup #description-input": "description_changed",
            "submit #change-timetable-form": "on_submit",
            "change #remove-datetimepicker": "on_try_add_remove"
        },

        initialize: function() {
            this.info_model = new ChangeTimetableInfoModel();
            this.listenTo( this.info_model, "change", this.on_info_fetched );

            this.add_collection = this.model.get( "add" );
            this.add_collection.reset();
            this.listenTo( this.add_collection, "add", this.on_new_add );

            this.remove_collection = this.model.get( "remove" );
            this.remove_collection.reset();
            this.listenTo( this.remove_collection, "add", this.on_new_remove );
        },

        close: function () {
            this.stopListening( this.add_collection );
            this.stopListening( this.remove_collection );
        },

        set_group_id: function( group_id ) {
            this.group_id = group_id;
            var handler = this.info_model.fetch( this.group_id );
            handler.not_found = this.not_found;
        },

        render: function() {
            this.$el.html( this.template( this.info_model.toJSON() ) );
            this.$desc_preview = $("#desc-preview")
            this.$form = $("#change-timetable-form");

            this.$remove_datetimepicker = $("#remove-datetimepicker");
            this.$remove_datetimepicker.datetimepicker({
                format: "Y/m/d",
                weeks: true,
                dayOfWeekStart: 1,
                inline: true,
                timepicker: false,
                highlightedDates: this.current_highlighted
            });
            $(".markdown").popup({
                hoverable: true,
                popup: "#markdown-help",
                lastResort: true
            });
            return this;
        },

        on_info_fetched: function() {
            var timetable = this.info_model.get( "current" );
            var highlighted = _.map( timetable, function( v ) {
                return v.date_str + "," + v.name;
            });
            this.current_highlighted = highlighted;

            this.render();
        },

        on_new_add: function( data ) {
            var idx = this.add_collection.length - 1;
            var view = new AddView({ model: data });
            $("#add-list").append( view.render( idx, this.current_highlighted ).$el );
        },

        on_new_remove: function( data ) {
            var view = new RemoveView({ model: data });
            $("#remove-list").append( view.render().$el );
        },

        on_need_new_add: function() {
            this.add_collection.add_one_more();
        },

        on_try_add_remove: function() {
            var date = this.$remove_datetimepicker.val();
            var current_timetable = this.info_model.get( "current" );
            var self = this;
            _.forEach( current_timetable, function( v ) {
                if ( date === v.date_str ) {
                    self.remove_collection.try_add( v );
                }
            });
        },

        description_changed: function() {
            var description = $( "#description-input" ).val();
            this.model.set({ description: description });
            this.$desc_preview.html( markdown.makeHtml( description ) );
        },

        on_submit: function() {
            this.close_all_errors();
            this.$form.addClass( "loading" );
            var handler = this.model.save( this.group_id );
            var self = this;
            handler.good = function() {
                growl(
                    {
                        header: "Запрос на изменение расписания создан.",
                        msg: "Теперь группа должна сказать своё слово. Вы получили сообщение с дальнейшими инструкциями",
                        positive: true
                    },
                    "long"
                );
                // обновляем состояние, чтоб увидеть что пришло новое письмо
                var app = require( "app" );
                // app.userState.fetch();
                var group = self.info_model.get("group");
                app.navGroup( group.id );
            }
            handler.not_found = this.not_found;
            handler.bad = function( data ) {
                self.show_errors( data );
            }
            handler.finish = function() {
                self.$form.removeClass( "loading" );
            }
        },

        show_errors: function( errors ) {
            var self = this;
            _.forEach( errors, function( error ) {
                var selector = null;
                if ( error.field_class === "Common" && error.field_type === "Description" ) {
                    selector = "#description";
                }
                if ( error.field_class === "ForAdd" ) {
                    if ( error.field_type === "Name" ) {
                        selector = "#name"
                    }
                    else if ( error.field_type === "Datetime" ) {
                        selector = "#datetime"
                    }
                    selector += "-" + error.idx;
                }
                if ( selector ) {
                    var $elem = $(selector)
                    var $elem_err = $(selector+"-err");
                    $elem.addClass( "error" );
                    $elem_err.html( self.reason_text( error.reason ) );
                    $elem_err.removeClass( "hidden" );
                }
            });
        },

        reason_text: function( reason ) {
            if ( reason === "Empty" ) {
                return "Надо бы заполнить";
            }
            else if ( reason === "TooLong") {
                return "Слишком длинное";
            }
            else if ( reason === "Invalid" ) {
                return "Некорректный формат";
            }
            else if ( reason === "TimeInPast" ) {
                return "Время указано в прошлое";
            }
            else if ( reason === "NotFound" ) {
                return "Такое не найдено";
            }
            return "Причина ошибки, известна только разработчикам. Обращайтесь к ним.";
        },

        close_all_errors: function() {
            var count = this.add_collection.length;
            var hide = function( selector ) {
                $(selector).removeClass( "error" );
                $(selector + "-err").addClass( "hidden" );
            }
            for ( i = 0; i < count; ++i ) {
                hide( "#name-" + i );
                hide( "#datetime-" + i );
            }
            hide( "#desctiprion" );
        },

        not_found: function() {
            errors_handler.oops(
                "Ошибка запроса информации о группе",
                "Что-то группа которую вы запрашиваете, не найдена, то-ли что-то пошло не так, то-ли кто-то что-то не то запрашивает. В общем фиг его знает, если вы не мудрили с адресной строкой, то похоже стоит обратиться к разработчикам."
            )
        }
    });

    return ChangeTimetableView;
})

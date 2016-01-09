define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" ),
        AddView = require( "change_timetable/add_view" ),
        ChangeTimetableInfoModel = require( "change_timetable/info_model" );
    require( "template/change_timetable" );

    var ChangeTimetableView = Backbone.View.extend({

        el: "#workspace",
        template: Handlebars.templates.change_timetable,

        events: {
            "click #new-add-btn": "on_need_new_add"
        },

        initialize: function() {
            this.info_model = new ChangeTimetableInfoModel();
            this.listenTo( this.info_model, "change", this.on_info_fetched );

            var add_collection = this.model.get( "add" );
            this.listenTo( add_collection, "add", this.on_new_add );
        },

        set_group_id: function( group_id ) {
            this.group_id = group_id;
            this.info_model.fetch( this.group_id );
        },

        render: function() {
            this.$el.html( this.template( this.info_model.toJSON() ) );
            return this;
        },

        on_info_fetched: function() {
            this.render();
        },

        on_new_add: function( data ) {
            var add_collection = this.model.get( "add" );
            var view = new AddView({ model: data });
            $("#add-list").append( view.render().$el );
        },

        on_need_new_add: function() {
            var add_collection = this.model.get( "add" );
            add_collection.add_one_more();
        },
    });

    return ChangeTimetableView;
})

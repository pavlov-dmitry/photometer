define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" ),
        tmpl = require( "template/none_action" );

    var NoneView = Backbone.View.extend({
        el: "#action",
        template: Handlebars.templates.none_action,

        initialize: function() {
            this.render();
        },

        render: function() {
            this.$el.html( this.template({}) );
        }
    });
    return NoneView;
})

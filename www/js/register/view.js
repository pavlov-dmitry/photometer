define( function(require){
    var Handlebars = require( "handlebars.runtime" ),
        Backbone = require( "lib/backbone" ),
        Config = require( "config" );
    require( "template/register" );

    var RegisterView = Backbone.View.extend({

        el: "#workspace",

        template: Handlebars.templates.register,

        events: {
            "submit #form-register": "submit"
        },

        initialize: function() {
            this.model.on( "change", this.render, this );
            this.render();
            $(this.el).children().hide();
            $(this.el).children().fadeIn( Config.showAniTime );
        },

        render: function() {
            $(this.el).html( this.template( this.model.toJSON() ) )
            return this;
        },

        submit: function() {
            console.log( "need register" );
        },

    });

    return RegisterView;
});
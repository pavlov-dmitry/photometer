define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" ),
        tmpl = require( "template/vote_action" ),
        growl = require( "growl" );

    var VoteView = Backbone.View.extend({
        el: "#action",
        template: Handlebars.templates.vote_action,

        events: {
            "click #yes-btn": "yes_clicked",
            "click #no-btn": "no_clicked"
        },

        initialize: function() {
            this.listenTo( this.model, "change", this.render );
        },

        render: function( data ) {
            this.$el.html( this.template( this.model.toJSON() ) );
        },

        yes_clicked: function() {
            this.send_vote( "yes" );
        },

        no_clicked:  function() {
            this.send_vote( "no" );
        },

        send_vote: function( vote ) {
            var request = require( "request" );
            var app = require( "app" );

            var id = this.model.get("id");
            var url = "/event/" + id;
            var handler = request.post( url, { vote: vote });

            var self = this;
            handler.good = function() {
                // self.$el.html( "<h2 class=\"ui center aligned basic segment\"><div class=\"content\"><font color=\"green\">Ваше мнение учтено.</font></div></h2>")
                growl({
                    header: "Голосование",
                    msg: "Ваше мнение учтено!",
                    positive: true
                }, "short");
            };
            handler.bad = function( data ) {
                // self.$el.html( "<h2 class=\"ui center aligned basic segment\"><div class=\"content\"><font color=\"red\">Похоже нельзя нам голосвать, уже.</font></div></h2>")
                growl({
                    header: "Голосование",
                    msg: "Похоже нельзя нам голосвать, уже.",
                    negative: true
                }, "short");
            };
            handler.finish = function() {
                self.model.finish();
            }
        }
    });

    return VoteView;
})

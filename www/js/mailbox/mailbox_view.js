define( function(require) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" ),
        mailbox_tmpl = require( "template/mailbox_view" ),
        pagination_tmpl = require( "template/pagination" ),
        make_pagination = require( "make_pagination" ),
        MailView = require( "mailbox/mail_view" );

    var MailboxView = Backbone.View.extend({

        el: $( "#workspace" ),

        template: Handlebars.templates.mailbox_view,
        pagination_tmpl: Handlebars.templates.pagination,

        initialize: function() {
            var self = this;

            this.listenTo( this.model, "add", this.addOne );
            this.listenTo( this.model, "reset", this.addAll );
            this.listenTo( this.model, "pages_changed", this.pagesChanged );

            this.render();
        },

        render: function() {
            this.$el.html( this.template({}) );
            return this;
        },

        addOne: function( data ) {
            var view = new MailView({
                model: data,
                id: "mail-" + data.id
            });
            this.$("#mail-list").append( view.render().$el );
        },

        addAll: function() {
            this.$("#mail-list").empty();
            this.model.each( this.addOne, this );
        },

        pagesChanged: function( data ) {
            if ( 1 < data.pages_count ) {
                var pagination = make_pagination( data.current_page, data.pages_count, "#mailbox/");

                var content = this.pagination_tmpl( pagination );
                $("#header-pagination").html( content );
                $("#footer-pagination").html( content );
            }
        },

    });

    return MailboxView;
})

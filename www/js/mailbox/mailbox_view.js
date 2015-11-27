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
            this.listenTo( this.model, "reset", this.onReset );
            this.listenTo( this.model, "update", this.onUpdate );
            this.listenTo( this.model, "pages_changed", this.pagesChanged );

            this.render();
        },

        render: function() {
            var state = {
                is_unreaded: this.model.is_only_unreaded
            };
            this.$el.html( this.template( state ) );
            return this;
        },

        addOne: function( data ) {
            this.listenTo( data, "marked", this.some_mail_marked );
            var view = new MailView({
                model: data,
                id: "mail-" + data.id
            });
            this.$("#mail-list").append( view.render().$el );
        },

        onReset: function() {
            this.$("#mail-list").empty();
        },

        onUpdate: function() {
            if ( this.model.length == 0 ) {
                $("#mail-list").html( "<h3 class=\"bulged text-center\"><strong>Здесь пусто</strong><h3>")
            }
        },


        pagesChanged: function( data ) {
            if ( 1 < data.pages_count ) {
                var link_prefix = "#mailbox/";
                if ( this.model.is_only_unreaded ) {
                    link_prefix += "unreaded/";
                }

                var pagination = make_pagination( data.current_page, data.pages_count, link_prefix );

                var content = this.pagination_tmpl( pagination );
                $("#header-pagination").html( content );
                $("#footer-pagination").html( content );
            }
        },

        some_mail_marked: function() {
            //TODO: надо бы переделать по человечески
            this.model.fetch( this.model.current_page );
            this.trigger( "some_mail_marked" );
        }

    });

    return MailboxView;
})

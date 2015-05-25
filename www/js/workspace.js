define( function(require) {

    var Backbone = require( "lib/backbone" );

    var Workspace = Backbone.Router.extend({
        routes: {
            'login': 'login',
            'register': 'register',
            'activate/:key': 'activate',
            'gallery': 'gallery',
            'gallery/:page': 'gallery_page',
            'edit_photo/:id': "edit_photo",
            'mailbox/unreaded': 'mailbox_unreaded',
            'mailbox/unreaded/:page': 'mailbox_unreaded_page',
            'mailbox': 'mailbox',
            'mailbox/:page': 'mailbox_page',
            'gallery_photo/:user_id/:photo_id': "gallery_photo"
        },

        clear_current: function() {
            if ( this.current ) {
                this.current.undelegateEvents();
                if ( this.current.close ) {
                    this.current.close();
                }
            }
        },

        nav: function( route ) {
            this.navigate( route, { trigger: true } );
        },

        login: function() {
            this.clear_current();

            var UserLoginView = require( "login/view" ),
            UserLoginModel = require( "login/model" );

            this.current = new UserLoginView( { model: new UserLoginModel } );
        },

        register: function() {
            this.clear_current();

            var RegisterView = require( "register/view" ),
            RegisterModel = require( "register/model" );

            this.current = new RegisterView( { model: new RegisterModel } );
        },

        activate: function( key ) {
            this.clear_current();

            var makeActivate = require( "activate" );
            makeActivate( key );
        },

        gallery: function() {
            this.gallery_page( 0 );
        },

        gallery_page: function( page ) {
            this.clear_current();

            var GalleryView = require( "gallery/gallery_view" ),
            GalleryCollection = require( "gallery/gallery_collection" );

            var model = new GalleryCollection;
            this.current = new GalleryView( { model: model } );
            model.fetch( page );

            require( ['app'], function( app ) {
                app.userState.navToGallery();
            })
        },

        mailbox: function() {
            this.mailbox_page( 0 );
        },

        mailbox_page: function( page ) {
            this.mailbox_show_page( page, false );
        },

        mailbox_unreaded: function() {
            this.mailbox_unreaded_page( 0 );
        },

        mailbox_unreaded_page: function( page ) {
            this.mailbox_show_page( page, true );
        },

        mailbox_show_page: function( page, is_only_unreaded ) {
            this.clear_current();

            var MailboxView = require( "mailbox/mailbox_view" ),
                MailsCollection = require( "mailbox/mails_collection" );

            var model = new MailsCollection;
            model.is_only_unreaded = is_only_unreaded;
            this.current = new MailboxView( { model: model } );

            model.fetch( page );

            var self = this;
            require( ['app'], function( app ) {
                app.userState.navToMessages();
                app.userState.listenTo( self.current, "some_mail_marked", app.userState.fetch );
            })
        },

        edit_photo: function( id ) {
            this.clear_current();

            var PhotoModel = require( "gallery/photo_model" );
            var PhotoEditView = require( "edit_photo/edit_view" );

            var model = new PhotoModel( {id: id} );
            model.photo_url = "gallery/photo_info";

            var self = this;
            require( ['app'], function( app ) {
                model.user_id = app.user_id();
                self.current = new PhotoEditView( { model: model } );
            } );
        },

        gallery_photo: function( user_id, photo_id ) {
            this.clear_current();



            // this.current = new PhotoView({model: new });
        }

    });

    return Workspace;

})

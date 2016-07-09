define( function(require) {

    var Backbone = require( "lib/backbone" );
    var app = require( 'app' );

    var Workspace = Backbone.Router.extend({
        routes: {
            "": 'root',
            'login': 'login',
            'logout': 'logout',
            'register': 'register',
            'activate/:key': 'activate',
            'gallery': 'current_gallery',
            'gallery/:user_id': 'gallery',
            'gallery/:user_id/:page': 'gallery_page',
            'edit_photo/:id': "edit_photo",
            'mailbox/unreaded': 'mailbox_unreaded',
            'mailbox/unreaded/:page': 'mailbox_unreaded_page',
            'mailbox': 'mailbox',
            'mailbox/:page': 'mailbox_page',
            'gallery_photo/:user_id/:photo_id': "gallery_photo",
            'group_creation': "group_creation",
            'event/:id': "event_info",
            'group/info/:id': "group_info",
            'group/feed/:id': "group_feed",
            'start': "start",
            'group/feed/element/:id': "group_feed_element",
            'change_timetable/:id': "change_timetable",
            'user_invite/:id' : "user_invite",
            'publication/:feed_id/photo/:photo_id': "publication_photo",
            "user/:id" : "user_description"
        },

        clear_current: function( is_next_photo ) {
            var view = this.current;
            if ( view ) {
                view.undelegateEvents();
                if ( view.close ) {
                    view.close( is_next_photo );
                }
            }
            app.userState.resetNav();
        },

        nav: function( route ) {
            this.navigate( route, { trigger: true } );
        },

        root: function() {
            this.start();
        },

        login: function() {
            if ( app.userState.is_logged_in() ) {
                this.start();
                return;
            }

            this.clear_current();

            var UserLoginView = require( "login/view" ),
            UserLoginModel = require( "login/model" );

            this.current = new UserLoginView( { model: new UserLoginModel } );
        },

        logout: function() {
            app.logout();
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

        current_gallery: function( user_id ) {
            var self = this;
            app.userState.on_ready( function() {
                self.gallery( app.userState.user_id() );
            });
        },

        gallery: function( user_id ) {
            this.gallery_page( user_id, 0 );
        },

        gallery_page: function( user_id, page ) {
            this.clear_current();

            var GalleryView = require( "gallery/gallery_view" );
            var GalleryModel = require( "gallery/gallery_model" );

            var model = new GalleryModel({
                owner: {
                    id: user_id
                }
            });
            this.current = new GalleryView( { model: model } );
            model.fetch( page );
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

            app.userState.navToMessages();
            app.userState.listenTo( this.current, "some_mail_marked", app.userState.fetch );
        },

        edit_photo: function( id ) {
            this.clear_current();

            var PhotoModel = require( "gallery/photo_model" );
            var PhotoEditView = require( "edit_photo/edit_view" );

            var model = new PhotoModel( {id: id} );
            model.photo_url = "gallery/photo_info";
            model.photo_data = {
                user: app.user_id(),
                photo: id
            };

            this.current = new PhotoEditView( { model: model } );
        },

        gallery_photo: function( user_id, photo_id ) {
            this.clear_current( true );

            var PhotoModel = require( "gallery/photo_model" );
            var PhotoView = require( "gallery/photo_view" );

            var model = new PhotoModel({ id: photo_id });
            model.photo_url = "gallery/photo_info";
            model.photo_data = {
                photo: photo_id,
                user: user_id
            }
            model.set({ context_url: "gallery_photo/" + user_id + "/" });

            this.current = new PhotoView({ model: model });
        },

        publication_photo: function( feed_id, photo_id ) {
            this.clear_current( true );

            var PhotoModel = require( "gallery/photo_model" );
            var PhotoView = require( "gallery/photo_view" );

            var model = new PhotoModel({ id: photo_id });
            model.photo_url = "publication/photo";
            model.photo_data = {
                feed_id: feed_id,
                photo_id: photo_id
            };
            model.set({ context_url: "publication/" + feed_id + "/photo/" });
            this.current = new PhotoView({ model: model });
        },

        group_creation: function() {
            this.clear_current();

            var GroupCreationModel = require( "group_creation/group_creation_model" );
            var GroupCreationView = require( "group_creation/group_creation_view" );

            var model = new GroupCreationModel();
            this.current = new GroupCreationView({ model: model });
        },

        event_info: function( id ) {
            this.clear_current();

            var EventInfoModel = require( "event/event_info_model" );
            var EventInfoView = require( "event/event_info_view" );

            var model = new EventInfoModel({ scheduled_id: id });
            this.current = new EventInfoView({ model: model });
        },

        group_info: function( id ) {
            this.clear_current();

            var GroupModel = require( "group/model" );
            var GroupView = require( "group/view" );

            var model = new GroupModel({ id: id });
            this.current = new GroupView({ model: model });
        },

        group_feed: function( id ) {
            this.clear_current();

            var GroupFeedModel = require( "group_feed/model" );
            var GroupFeedView = require( "group_feed/view" );

            var model = new GroupFeedModel({ id: id });
            this.current = new GroupFeedView({ model: model });
            app.userState.navToGroup();
        },

        start: function() {
            var self = this;
            app.userState.on_ready( function() {
                var group_id = app.userState.get_current_group_id();
                if ( group_id != 0 )
                {
                    app.navGroup( group_id );
                }
                else
                {
                    app.navMessages();
                }
            });
        },

        group_feed_element: function( id ) {
            this.clear_current();

            var GroupFeedElementModel = require( "group_feed/element_model" );
            var GroupFeedElementView = require( "group_feed/element_view" );

            var model = new GroupFeedElementModel({ id: id });
            this.current = new GroupFeedElementView({ model: model });
            app.userState.navToGroup();
        },

        change_timetable: function( group_id ) {
            this.clear_current();

            var ChangeTimetableModel = require( "change_timetable/model" );
            var ChangeTimetableView = require( "change_timetable/view" );

            var model = new ChangeTimetableModel();
            this.current = new ChangeTimetableView({model: model});
            this.current.set_group_id( group_id );
        },

        user_invite: function( group_id ) {
            this.clear_current();

            var UserInviteModel = require( "user_invite/model" );
            var UserInviteView = require( "user_invite/view" );

            var model = new UserInviteModel({group_id: group_id});
            this.current = new UserInviteView({model: model});
        },

        user_description: function( user_id ) {
            this.clear_current();

            var UserDescriptionModel = require( "user_description/model" );
            var UserDescriptionView = require( "user_description/view" );

            var model = new UserDescriptionModel({id: user_id});
            this.current = new UserDescriptionView({model: model});
        }

    });

    return Workspace;

})

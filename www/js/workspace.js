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
        },

        clearCurrent: function() {
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
            this.clearCurrent();

            var UserLoginView = require( "login/view" ),
            UserLoginModel = require( "login/model" );

            this.current = new UserLoginView( { model: new UserLoginModel } );
        },

        register: function() {
            this.clearCurrent();

            var RegisterView = require( "register/view" ),
            RegisterModel = require( "register/model" );

            this.current = new RegisterView( { model: new RegisterModel } );
        },

        activate: function( key ) {
            this.clearCurrent();

            var makeActivate = require( "activate" );
            makeActivate( key );
        },

        gallery: function() {
            this.gallery_page( 0 );
        },

        gallery_page: function( page ) {
            this.clearCurrent();

            var GalleryView = require( "gallery/gallery_view" ),
            GalleryCollection = require( "gallery/gallery_collection" );

            var model = new GalleryCollection;
            this.current = new GalleryView( { model: model } );
            model.fetch( page );

            require( ['app'], function( app ) {
                app.userState.navToGallery();
            })
        },

        edit_photo: function( id ) {
            this.clearCurrent();

            var PhotoModel = require( "gallery/photo_model" );
            var PhotoEditView = require( "edit_photo/edit_view" );

            this.current = new PhotoEditView( { model: new PhotoModel( {id: id} ) } );
        }

    });

    return Workspace;

})

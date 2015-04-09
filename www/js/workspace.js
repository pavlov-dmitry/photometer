define( function(require) {

    var Backbone = require( "lib/backbone" );

    var Workspace = Backbone.Router.extend({
        routes: {
            'login': 'login',
            'register': 'register',
            'activate/:key': 'activate',
            'gallery': 'gallery',
            'edit_photo/:id': "edit_photo",
        },

        nav: function( route ) {
            this.navigate( route, { trigger: true } );
        },

        login: function() {
            var UserLoginView = require( "login/view" ),
            UserLoginModel = require( "login/model" );

            this.current = new UserLoginView( { model: new UserLoginModel } );
        },

        register: function() {
            var RegisterView = require( "register/view" ),
            RegisterModel = require( "register/model" );

            this.current = new RegisterView( { model: new RegisterModel } );
        },

        activate: function( key ) {
            var makeActivate = require( "activate" );
            makeActivate( key );
        },

        gallery: function() {
            var GalleryView = require( "gallery/gallery_view" ),
            GalleryCollection = require( "gallery/gallery_collection" );

            this.current = new GalleryView( { model: new GalleryCollection } );

            require( ['app'], function( app ) {
                app.userState.navToGallery();
            })
        },

        edit_photo: function( id ) {
            var PhotoModel = require( "gallery/photo_model" );
            var PhotoEditView = require( "edit_photo/edit_view" );

            this.current = new PhotoEditView( { model: new PhotoModel( {id: id} ) } );
        }

    });

    return Workspace;

})

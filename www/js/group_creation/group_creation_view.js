define( function( require ) {
    var Backbone = require( "lib/backbone" ),
        Handlebars = require( "handlebars.runtime" ),
        UserView = require( "group_creation/user_view" ),
        group_creation_template = require( "template/group_creation" )

    var GroupCreationView = Backbone.View.extend({

        el: $( "#workspace" ),

        template: Handlebars.templates.group_creation,

        events: {
            "change #description-input": "description_changed",
            "keyup #description-input": "description_changed",
            "keyup #name-input": "description_changed",
            "click #add-member-btn": "add_user_clicked",
            "change #name-input": "name_changed"
        },

        initialize: function() {
            var members = this.model.get( "members" );
            this.listenTo( members, "add", this.user_added );
            this.listenTo( members, "remove", this.check_users_for_remove );

            this.render();
        },

        close: function() {
            var members = this.model.get( "members" );
            members.off( null, this.user_added );
            members.off( null, this.check_users_for_remove );
        },

        render: function() {
            this.$el.html( this.template( this.model.toJSON() ) );
            var members = this.model.get( "members" );
            members.each( this.user_added, this );
        },

        add_user_clicked: function() {
            this.model.add_new_member();
        },

        user_added: function( data ) {
            var members = this.model.get( "members" );
            var view = new UserView({
                model: data,
                id: "user-" + members.size()
            });
            var user_el = view.render().$el;
            this.$("#users-list").append( user_el );
            this.check_users_for_remove();
        },

        description_changed: function() {
            markdown = require( 'showdown_converter' );

            var group_name = $( "#name-input" ).val();
            var description = $( "#description-input" ).val();

            this.model.set({ description: description });

            var desc_html = markdown.makeHtml( description );
            var all_html = "<h1>" + group_name + "</h1>" + desc_html;

            $( "#info-preview").html( all_html );
        },

        name_changed: function() {
            this.model.set({ name: $("#name-input").val() });
        },

        check_users_for_remove: function() {
            var members = this.model.get( "members" );
            var is_removeable = members.size() !== 1;
            members.forEach( function( m ) {
                m.set_removeable( is_removeable );
            });
        }
    });

    return GroupCreationView;
})

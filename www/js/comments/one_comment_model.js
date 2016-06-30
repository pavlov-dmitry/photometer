define( function( require ) {
    var Backbone = require( "lib/backbone" ),
        request = require( "request" );

    var OneCommentModel = Backbone.Model.extend({
        defaults: {
            id: 0,
            cancelable: true
        },

        quote: function() {
            var text = this.get("text");
            var lines = text.split('\n');
            var creator = this.get("creator");
            var quote_text = "> #### @" + creator.name + "\n";
            _.forEach( lines, function( line ) {
                quote_text += "> " + line + "\n";
            })
            this.trigger( "quote", quote_text );
        },

        start_edit: function() {
            var save = this.get("text");
            this.set({ save: save });
        },

        cancel: function() {
            var save = this.get("save");
            this.set({ text: save });
            this.trigger("cancel");
        },

        save: function() {
            var handler = request.post( "/comment/edit", {
                id: this.get("id"),
                text: this.get("text")
            });
            var self = this;
            handler.good = function() {
                self.trigger( "edited" );
            }
            return handler;
        }
    });

    return OneCommentModel;
})

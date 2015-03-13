require.config({
	baseUrl: "js",
	shim : {
        "bootstrap" : { deps :['jquery'] }
    },
	paths: {
		jquery: "lib/jquery",
		bootstrap: "lib/bootstrap",
        underscore: "lib/underscore",
        template: "../template",
        'handlebars.runtime': 'lib/handlebars-runtime'
	}
});

require( [ 'bootstrap', 'app'], function () {} );

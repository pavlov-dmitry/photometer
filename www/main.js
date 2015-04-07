require.config({
    baseUrl: 'js',
    shim : {
        'bootstrap' : { deps :['jquery'] }
    },
    paths: {
	jquery: 'lib/jquery',
        'jquery.ui.widget': 'lib/jquery.ui.widget',
	bootstrap: 'lib/bootstrap',
        underscore: 'lib/underscore',
        template: '../template',
        'handlebars.runtime': 'lib/handlebars-runtime'
    }
});

require( [ 'bootstrap', 'app'], function () {} );

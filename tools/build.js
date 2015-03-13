{
    appDir: "../www",
    baseUrl: "js",
    dir: "../www-release",
    shim : {
        "bootstrap" : { deps :['jquery'] }
    },
    paths: {
        jquery: "lib/jquery",
        bootstrap: "lib/bootstrap",
        underscore: "lib/underscore",
        template: "../template",
        'handlebars.runtime': 'lib/handlebars-runtime'
    },
    modules: [
        {
            name: "../main"
        }
    ],
    optimize: "uglify2",
    preserveLicenseComments: false
}
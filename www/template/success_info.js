define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['success_info'] = template({"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function";

  return "<div class=\"container\">\n    <div class=\"jumbotron\">\n        <h2 class=\"text-success\">"
    + container.escapeExpression(((helper = (helper = helpers.caption || (depth0 != null ? depth0.caption : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"caption","hash":{},"data":data}) : helper)))
    + "</h2>\n        <p>"
    + ((stack1 = ((helper = (helper = helpers.text || (depth0 != null ? depth0.text : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"text","hash":{},"data":data}) : helper))) != null ? stack1 : "")
    + "</p>\n    </div>\n</div>\n";
},"useData":true});
});
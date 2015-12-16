define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['vote_action'] = template({"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var helper;

  return "<p>\n  <h3 class=\"text-center\"><i>"
    + container.escapeExpression(((helper = (helper = helpers.answer || (depth0 != null ? depth0.answer : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0 != null ? depth0 : {},{"name":"answer","hash":{},"data":data}) : helper)))
    + "</i></h3>\n</p>\n<div class=\"row\">\n  <div class=\"col-sm-4 col-sm-offset-4\">\n    <button id=\"yes-btn\" class=\"btn btn-lg btn-success fly\" type=\"button\"> Да</button>\n    <button id=\"no-btn\" class=\"btn btn-lg btn-danger fly\" type=\"button\"> Нет</button>\n  <div>\n</div>\n";
},"useData":true});
});
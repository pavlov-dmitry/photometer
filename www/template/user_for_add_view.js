define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['user_for_add_view'] = template({"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var helper;

  return "<p>\n  <div class=\"input-group\">\n    <input type=\"text\" class=\"form-control user-name\" placeholder=\"Имя пользователя\" value=\""
    + container.escapeExpression(((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0 != null ? depth0 : {},{"name":"name","hash":{},"data":data}) : helper)))
    + "\" required>\n    <span class=\"input-group-btn\">\n      <button class=\"btn btn-default remove-btn\" type=\"button\"><span class=\"glyphicon glyphicon-trash\"></span></button>\n    </span>\n  </div>\n</p>\n";
},"useData":true});
});
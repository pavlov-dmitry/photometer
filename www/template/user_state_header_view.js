define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['user_state_header_view'] = template({"1":function(depth0,helpers,partials,data) {
    var helper;

  return "  <li class=\"dropdown\">\n    <a href=\"#\" class=\"dropdown-toggle\" data-toggle=\"dropdown\" role=\"button\" aria-expanded=\"false\">\n      <span class=\"glyphicon glyphicon-user\" ></span> "
    + this.escapeExpression(((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0,{"name":"name","hash":{},"data":data}) : helper)))
    + " <span class=\"caret\"></span>\n    </a>\n    <ul class=\"dropdown-menu\" role=\"menu\">\n      <li class=\"devider\"></li>\n      <li><a id=\"logout-action\" href=\"#\" onClick=\"return false\"><span class=\"glyphicon glyphicon-off\"></span> Выйти</a></li>\n      <!-- <li><button type=\"button\" class=\"navbar-btn\"><span class=\"glyphicon glyphicon-off\"> Выйти</button></li> -->\n    </ul>\n  </li>\n";
},"3":function(depth0,helpers,partials,data) {
    return "  <a href=\"#login\"><button type=\"button\" class=\"btn btn-success navbar-btn\">Войти</button></a>\n  <div class=\"nav navbar-nav spacer-w-sm\"></div>\n";
},"compiler":[6,">= 2.0.0-beta.1"],"main":function(depth0,helpers,partials,data) {
    var stack1;

  return "<ul class=\"nav navbar-nav navbar-right\">\n"
    + ((stack1 = helpers['if'].call(depth0,(depth0 != null ? depth0.isLogged : depth0),{"name":"if","hash":{},"fn":this.program(1, data, 0),"inverse":this.program(3, data, 0),"data":data})) != null ? stack1 : "")
    + "</ul>\n";
},"useData":true});
});
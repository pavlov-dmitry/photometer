define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['user_state_header_view'] = template({"1":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {};

  return "<ul class=\"nav navbar-nav\">\n  <li "
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.isNavInMessages : depth0),{"name":"if","hash":{},"fn":container.program(2, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + ">\n    <a href=\"#mailbox/unreaded\"><span class=\"glyphicon glyphicon-inbox\"></span> Сообщения"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.unreaded_messages : depth0),{"name":"if","hash":{},"fn":container.program(4, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "</a>\n  </li>\n  <li "
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.isNavInGallery : depth0),{"name":"if","hash":{},"fn":container.program(2, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "><a href=\"#gallery\"><span class=\"glyphicon glyphicon-picture\"></span> Галлерея</a></li>\n</ul>\n<ul class=\"nav navbar-nav navbar-right\">\n  <li class=\"dropdown\">\n    <a href=\"#\" class=\"dropdown-toggle\" data-toggle=\"dropdown\" role=\"button\" aria-expanded=\"false\">\n      <span class=\"glyphicon glyphicon-user\" ></span> "
    + container.escapeExpression(((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(alias1,{"name":"name","hash":{},"data":data}) : helper)))
    + " <span class=\"caret\"></span>\n    </a>\n    <ul class=\"dropdown-menu inverse-dropdown\" role=\"menu\">\n      <li><a id=\"logout-action\" href=\"#\" onClick=\"return false\"><span class=\"glyphicon glyphicon-off\"></span> Выйти</a></li>\n    </ul>\n  </li>\n</ul>\n";
},"2":function(container,depth0,helpers,partials,data) {
    return "class=\"active\"";
},"4":function(container,depth0,helpers,partials,data) {
    var helper;

  return " <span class=\"badge\">"
    + container.escapeExpression(((helper = (helper = helpers.unreaded_messages || (depth0 != null ? depth0.unreaded_messages : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0 != null ? depth0 : {},{"name":"unreaded_messages","hash":{},"data":data}) : helper)))
    + "</span>";
},"6":function(container,depth0,helpers,partials,data) {
    return "<ul class=\"nav navbar-nav navbar-right\">\n  <a href=\"#login\"><button type=\"button\" class=\"btn btn-success navbar-btn\">Войти</button></a>\n  <div class=\"nav navbar-nav spacer-w-sm\"></div>\n</ul>\n";
},"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1;

  return ((stack1 = helpers["if"].call(depth0 != null ? depth0 : {},(depth0 != null ? depth0.isLogged : depth0),{"name":"if","hash":{},"fn":container.program(1, data, 0),"inverse":container.program(6, data, 0),"data":data})) != null ? stack1 : "");
},"useData":true});
});
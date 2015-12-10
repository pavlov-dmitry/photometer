define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['user_state_header_view'] = template({"1":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {};

  return "<ul class=\"nav navbar-nav\">\n  <li class=\""
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.isNavInMessages : depth0),{"name":"if","hash":{},"fn":container.program(2, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "caret-left\">\n    <a href=\"#mailbox/unreaded\"><span class=\"glyphicon glyphicon-inbox\"></span> Сообщения"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.unreaded_messages : depth0),{"name":"if","hash":{},"fn":container.program(4, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "</a>\n  </li>\n  <li class=\""
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.isNavInGallery : depth0),{"name":"if","hash":{},"fn":container.program(2, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "caret-left\"><a href=\"#gallery\"><span class=\"glyphicon glyphicon-picture\"></span> Галлерея</a></li>\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.is_groups : depth0),{"name":"if","hash":{},"fn":container.program(6, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "</ul>\n\n<ul class=\"nav navbar-nav navbar-right\">\n  <li class=\"dropdown caret-left\">\n    <a href=\"#\" class=\"dropdown-toggle\" data-toggle=\"dropdown\" role=\"button\" aria-expanded=\"false\">\n      <span class=\"glyphicon glyphicon-user\" ></span> "
    + container.escapeExpression(((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(alias1,{"name":"name","hash":{},"data":data}) : helper)))
    + " <span class=\"caret\"></span>\n    </a>\n    <ul class=\"dropdown-menu inverse-dropdown\" role=\"menu\">\n      <li><a id=\"group-create-action\" href=\"#group-creation\"><span class=\"glyphicon glyphicon-folder-open\"></span> Создать новую группу</li>\n      <li><a id=\"logout-action\" href=\"#\" onClick=\"return false\"><span class=\"glyphicon glyphicon-off\"></span> Выйти</a></li>\n    </ul>\n  </li>\n</ul>\n";
},"2":function(container,depth0,helpers,partials,data) {
    return "active ";
},"4":function(container,depth0,helpers,partials,data) {
    var helper;

  return " <span class=\"badge\">"
    + container.escapeExpression(((helper = (helper = helpers.unreaded_messages || (depth0 != null ? depth0.unreaded_messages : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0 != null ? depth0 : {},{"name":"unreaded_messages","hash":{},"data":data}) : helper)))
    + "</span>";
},"6":function(container,depth0,helpers,partials,data) {
    var stack1, alias1=depth0 != null ? depth0 : {}, alias2=container.lambda, alias3=container.escapeExpression;

  return "  <li class=\"caret-left"
    + ((stack1 = helpers.unless.call(alias1,(depth0 != null ? depth0.is_many_groups : depth0),{"name":"unless","hash":{},"fn":container.program(7, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "\">\n    <a href=\"#group/"
    + alias3(alias2(((stack1 = (depth0 != null ? depth0.current_group : depth0)) != null ? stack1.id : stack1), depth0))
    + "\">\n      <span class=\"glyphicon glyphicon-certificate\"></span> "
    + alias3(alias2(((stack1 = (depth0 != null ? depth0.current_group : depth0)) != null ? stack1.name : stack1), depth0))
    + ((stack1 = helpers["if"].call(alias1,((stack1 = (depth0 != null ? depth0.current_group : depth0)) != null ? stack1.unwatched_events : stack1),{"name":"if","hash":{},"fn":container.program(9, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "\n    </a>\n  </li>\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.is_many_groups : depth0),{"name":"if","hash":{},"fn":container.program(11, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "");
},"7":function(container,depth0,helpers,partials,data) {
    return " caret-right";
},"9":function(container,depth0,helpers,partials,data) {
    var stack1;

  return "<span class=\"badge\">"
    + container.escapeExpression(container.lambda(((stack1 = (depth0 != null ? depth0.current_group : depth0)) != null ? stack1.unwatched_events : stack1), depth0))
    + "</span>";
},"11":function(container,depth0,helpers,partials,data) {
    var stack1, alias1=depth0 != null ? depth0 : {};

  return "  <li class=\"dropdown pull-left\">\n    <a class=\"dropdown-toggle low-caret-left caret-right\" data-toggle=\"dropdown\">\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.has_unreaded_in_groups : depth0),{"name":"if","hash":{},"fn":container.program(12, data, 0),"inverse":container.program(14, data, 0),"data":data})) != null ? stack1 : "")
    + "    </a>\n    <ul class=\"dropdown-menu inverse-dropdown pull-right\">\n"
    + ((stack1 = helpers.each.call(alias1,(depth0 != null ? depth0.groups : depth0),{"name":"each","hash":{},"fn":container.program(16, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "    </ul>\n";
},"12":function(container,depth0,helpers,partials,data) {
    return "      <span class=\"glyphicon glyphicon-collapse-down\"></span>\n";
},"14":function(container,depth0,helpers,partials,data) {
    return "      <b class=\"caret\"></b>\n";
},"16":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return "      <li>\n        <a href=\"#group/"
    + alias4(((helper = (helper = helpers.id || (depth0 != null ? depth0.id : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"id","hash":{},"data":data}) : helper)))
    + "\">"
    + alias4(((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"name","hash":{},"data":data}) : helper)))
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.unwatched_events : depth0),{"name":"if","hash":{},"fn":container.program(17, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "</a>\n      </li>\n";
},"17":function(container,depth0,helpers,partials,data) {
    var helper;

  return "<span class=\"badge\">"
    + container.escapeExpression(((helper = (helper = helpers.unwatched_events || (depth0 != null ? depth0.unwatched_events : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0 != null ? depth0 : {},{"name":"unwatched_events","hash":{},"data":data}) : helper)))
    + "</span>";
},"19":function(container,depth0,helpers,partials,data) {
    return "<ul class=\"nav navbar-nav navbar-right\">\n  <a href=\"#login\"><button type=\"button\" class=\"btn btn-success navbar-btn\">Войти</button></a>\n  <div class=\"nav navbar-nav spacer-w-sm\"></div>\n</ul>\n";
},"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1;

  return ((stack1 = helpers["if"].call(depth0 != null ? depth0 : {},(depth0 != null ? depth0.isLogged : depth0),{"name":"if","hash":{},"fn":container.program(1, data, 0),"inverse":container.program(19, data, 0),"data":data})) != null ? stack1 : "");
},"useData":true});
});
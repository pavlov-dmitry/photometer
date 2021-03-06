define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['user_state_header_view'] = template({"1":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return "<div id=\"main-menu\" class=\"ui pointing secondary stackable main menu\">\n  <div class=\"ui container\">\n    <div class=\"header item\">\n      <img class=\"ui logo\" src=\"i/logo.png\" data-variation=\"wide\">\n    </div>\n    <a class=\"item"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.isNavInMessages : depth0),{"name":"if","hash":{},"fn":container.program(2, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "\" href=\"#mailbox/unreaded\">\n      <i class=\"icon inbox\">\n      </i>\n      Сообщения\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.unreaded_messages : depth0),{"name":"if","hash":{},"fn":container.program(4, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "    </a>\n    <a class=\"item"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.isNavInGallery : depth0),{"name":"if","hash":{},"fn":container.program(2, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "\" href=\"#gallery/"
    + alias4(((helper = (helper = helpers.id || (depth0 != null ? depth0.id : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"id","hash":{},"data":data}) : helper)))
    + "\">\n      <i class=\"icon film\"></i>\n      Галлерея\n    </a>\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.is_groups : depth0),{"name":"if","hash":{},"fn":container.program(6, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "    <div class=\"right menu\">\n      <div id=\"user_menu\" class=\"ui top right pointing dropdown item\">\n        "
    + alias4(((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"name","hash":{},"data":data}) : helper)))
    + "\n        <i class=\"dropdown icon\"></i>\n        <div class=\"menu\">\n          <a class=\"item\" href=\"#group_creation\">\n            <i class=\"child icon\"></i>\n            Создать новую группу\n          </a>\n          <a class=\"item\" href=\"#logout\">\n            <i class=\"sign out icon\"></i>\n            Выход\n          </a>\n        </div>\n      </div>\n    </div>\n  </div>\n</div>\n";
},"2":function(container,depth0,helpers,partials,data) {
    return " active";
},"4":function(container,depth0,helpers,partials,data) {
    var helper;

  return "      <div class=\"ui yellow label\">\n        "
    + container.escapeExpression(((helper = (helper = helpers.unreaded_messages || (depth0 != null ? depth0.unreaded_messages : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0 != null ? depth0 : {},{"name":"unreaded_messages","hash":{},"data":data}) : helper)))
    + "\n      </div>\n";
},"6":function(container,depth0,helpers,partials,data) {
    var stack1, alias1=depth0 != null ? depth0 : {}, alias2=container.lambda, alias3=container.escapeExpression;

  return "    <div class=\"ui buttons\">\n      <a class=\""
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.isNavInGroup : depth0),{"name":"if","hash":{},"fn":container.program(7, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "button item\" href=\"#group/feed/"
    + alias3(alias2(((stack1 = (depth0 != null ? depth0.current_group : depth0)) != null ? stack1.id : stack1), depth0))
    + "\">\n        <i class=\"icon users\"></i>\n        "
    + alias3(alias2(((stack1 = (depth0 != null ? depth0.current_group : depth0)) != null ? stack1.name : stack1), depth0))
    + "\n"
    + ((stack1 = helpers["if"].call(alias1,((stack1 = (depth0 != null ? depth0.current_group : depth0)) != null ? stack1.unwatched_events : stack1),{"name":"if","hash":{},"fn":container.program(9, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "      </a>\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.is_many_groups : depth0),{"name":"if","hash":{},"fn":container.program(11, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "    </div>\n";
},"7":function(container,depth0,helpers,partials,data) {
    return "active ";
},"9":function(container,depth0,helpers,partials,data) {
    var stack1;

  return "        <div class=\"ui yellow label\">\n          "
    + container.escapeExpression(container.lambda(((stack1 = (depth0 != null ? depth0.current_group : depth0)) != null ? stack1.unwatched_events : stack1), depth0))
    + "\n        </div>\n";
},"11":function(container,depth0,helpers,partials,data) {
    var stack1, alias1=depth0 != null ? depth0 : {};

  return "      <div id=\"groups_menu\" class=\"ui top right pointing floating dropdown icon button item\">\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.has_unreaded_in_other_groups : depth0),{"name":"if","hash":{},"fn":container.program(12, data, 0),"inverse":container.program(14, data, 0),"data":data})) != null ? stack1 : "")
    + "        <div class=\"menu\">\n"
    + ((stack1 = helpers.each.call(alias1,(depth0 != null ? depth0.groups : depth0),{"name":"each","hash":{},"fn":container.program(16, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "        </div>\n      </div>\n";
},"12":function(container,depth0,helpers,partials,data) {
    return "        <i class=\"yellow chevron circle down icon\"></i>\n";
},"14":function(container,depth0,helpers,partials,data) {
    return "        <i class=\"dropdown icon\"></i>\n";
},"16":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return "          <a class=\"item\" href=\"#group/feed/"
    + alias4(((helper = (helper = helpers.id || (depth0 != null ? depth0.id : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"id","hash":{},"data":data}) : helper)))
    + "\">\n            <i class=\"icon users\"></i>\n            "
    + alias4(((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"name","hash":{},"data":data}) : helper)))
    + "\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.unwatched_events : depth0),{"name":"if","hash":{},"fn":container.program(17, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "          </a>\n";
},"17":function(container,depth0,helpers,partials,data) {
    var helper;

  return "            <div class=\"ui yellow label\">\n              "
    + container.escapeExpression(((helper = (helper = helpers.unwatched_events || (depth0 != null ? depth0.unwatched_events : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0 != null ? depth0 : {},{"name":"unwatched_events","hash":{},"data":data}) : helper)))
    + "\n            </div>\n";
},"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1;

  return ((stack1 = helpers["if"].call(depth0 != null ? depth0 : {},(depth0 != null ? depth0.isLogged : depth0),{"name":"if","hash":{},"fn":container.program(1, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "");
},"useData":true});
});
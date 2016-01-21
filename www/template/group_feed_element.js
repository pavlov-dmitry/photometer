define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['group_feed_element'] = template({"1":function(container,depth0,helpers,partials,data) {
    var stack1, alias1=container.lambda, alias2=container.escapeExpression;

  return "    <a class=\"section\" href=\"#user/"
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.creator : depth0)) != null ? stack1.id : stack1), depth0))
    + "\">"
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.creator : depth0)) != null ? stack1.name : stack1), depth0))
    + "</a>\n    <i class=\"at divider icon\"></i>\n";
},"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {}, alias2=container.lambda, alias3=container.escapeExpression;

  return "<div class=\"ui container\">\n  <div class=\"ui huge breadcrumb\">\n    <i class=\"large users icon\"></i>\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.creator : depth0),{"name":"if","hash":{},"fn":container.program(1, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "    <a class=\"section\" href=\"#group/feed/"
    + alias3(alias2(((stack1 = (depth0 != null ? depth0.group : depth0)) != null ? stack1.id : stack1), depth0))
    + "\">"
    + alias3(alias2(((stack1 = (depth0 != null ? depth0.group : depth0)) != null ? stack1.name : stack1), depth0))
    + "</a>\n    <i class=\"right chevron divider icon\"></i>\n    <div class=\"active section\">"
    + alias3(((helper = (helper = helpers.event_name || (depth0 != null ? depth0.event_name : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(alias1,{"name":"event_name","hash":{},"data":data}) : helper)))
    + "</div>\n  </div>\n  <div id=\"feed-view\" class=\"ui large feed\">\n  </div>\n</div>\n";
},"useData":true});
});
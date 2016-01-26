define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['event_info'] = template({"1":function(container,depth0,helpers,partials,data) {
    return "    <i class=\"large announcement icon\"></i>\n";
},"3":function(container,depth0,helpers,partials,data) {
    var stack1;

  return ((stack1 = (helpers.if_equal || (depth0 && depth0.if_equal) || helpers.helperMissing).call(depth0 != null ? depth0 : {},(depth0 != null ? depth0.id : depth0),"GroupVoting",{"name":"if_equal","hash":{},"fn":container.program(4, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "");
},"4":function(container,depth0,helpers,partials,data) {
    return "    <i class=\"large alarm icon\"></i>\n    ";
},"6":function(container,depth0,helpers,partials,data) {
    var stack1, alias1=container.lambda, alias2=container.escapeExpression;

  return "    <a class=\"section\" href=\"#user/"
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.creator : depth0)) != null ? stack1.id : stack1), depth0))
    + "\">\n      "
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.creator : depth0)) != null ? stack1.name : stack1), depth0))
    + "\n    </a>\n    <i class=\"at icon divider\"></i>\n";
},"8":function(container,depth0,helpers,partials,data) {
    var stack1, alias1=container.lambda, alias2=container.escapeExpression;

  return "    <a class=\"section\" href=\"#/group/feed/"
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.group : depth0)) != null ? stack1.id : stack1), depth0))
    + "\">\n      "
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.group : depth0)) != null ? stack1.name : stack1), depth0))
    + "\n    </a>\n    <i class=\"right chevron icon divider\"></i>\n";
},"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.lambda, alias5=container.escapeExpression;

  return "<div class=\"ui container\">\n  <div class=\"ui huge event breadcrumb\">\n"
    + ((stack1 = (helpers.if_equal || (depth0 && depth0.if_equal) || alias2).call(alias1,(depth0 != null ? depth0.id : depth0),"Publication",{"name":"if_equal","hash":{},"fn":container.program(1, data, 0),"inverse":container.program(3, data, 0),"data":data})) != null ? stack1 : "")
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.creator : depth0),{"name":"if","hash":{},"fn":container.program(6, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.group : depth0),{"name":"if","hash":{},"fn":container.program(8, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "    <div class=\"active section\">\n      "
    + ((stack1 = ((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"name","hash":{},"data":data}) : helper))) != null ? stack1 : "")
    + "\n    </div>\n  </div>\n  <div class=\"ui small "
    + alias5(alias4(((stack1 = (depth0 != null ? depth0.state : depth0)) != null ? stack1.color : stack1), depth0))
    + " event label\">\n    "
    + alias5(alias4(((stack1 = (depth0 != null ? depth0.state : depth0)) != null ? stack1.text : stack1), depth0))
    + "\n  </div>\n  <p class=\"header-date\">\n    Создано\n    "
    + alias5((helpers.duration_from_now || (depth0 && depth0.duration_from_now) || alias2).call(alias1,(depth0 != null ? depth0.starting_time : depth0),{"name":"duration_from_now","hash":{},"data":data}))
    + "\n    "
    + alias5((helpers.time || (depth0 && depth0.time) || alias2).call(alias1,(depth0 != null ? depth0.starting_time : depth0),{"name":"time","hash":{},"data":data}))
    + ",\n    Окончание\n    "
    + alias5((helpers.duration_from_now || (depth0 && depth0.duration_from_now) || alias2).call(alias1,(depth0 != null ? depth0.ending_time : depth0),{"name":"duration_from_now","hash":{},"data":data}))
    + "\n    "
    + alias5((helpers.time || (depth0 && depth0.time) || alias2).call(alias1,(depth0 != null ? depth0.ending_time : depth0),{"name":"time","hash":{},"data":data}))
    + "\n  </p>\n  <div id=\"description\">\n    "
    + ((stack1 = ((helper = (helper = helpers.description || (depth0 != null ? depth0.description : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"description","hash":{},"data":data}) : helper))) != null ? stack1 : "")
    + "\n  </div>\n</div>\n<div id=\"action\">\n  "
    + ((stack1 = ((helper = (helper = helpers.action || (depth0 != null ? depth0.action : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"action","hash":{},"data":data}) : helper))) != null ? stack1 : "")
    + "\n</div>\n<div class=\"ui container\">\n  <div id=\"comments\"></div>\n  <div id=\"comment-editor\"></div>\n</div>\n";
},"useData":true});
});
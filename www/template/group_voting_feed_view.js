define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['group_voting_feed_view'] = template({"1":function(container,depth0,helpers,partials,data) {
    return "    <i class=\"alarm icon\"></i>\n";
},"3":function(container,depth0,helpers,partials,data) {
    var stack1;

  return ((stack1 = helpers["if"].call(depth0 != null ? depth0 : {},((stack1 = (depth0 != null ? depth0.data : depth0)) != null ? stack1.is_success : stack1),{"name":"if","hash":{},"fn":container.program(4, data, 0),"inverse":container.program(6, data, 0),"data":data})) != null ? stack1 : "");
},"4":function(container,depth0,helpers,partials,data) {
    return "      <i class=\"green checkmark icon\"></i>\n";
},"6":function(container,depth0,helpers,partials,data) {
    return "      <i class=\"red alarm slash icon\"></i>\n";
},"8":function(container,depth0,helpers,partials,data) {
    return "      <div class=\"ui yellow label\">новое</div>\n";
},"10":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.creator : depth0),{"name":"if","hash":{},"fn":container.program(11, data, 0),"inverse":container.program(13, data, 0),"data":data})) != null ? stack1 : "")
    + "        голосование за\n        <a href=\"#event/"
    + alias4(((helper = (helper = helpers.scheduled_id || (depth0 != null ? depth0.scheduled_id : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"scheduled_id","hash":{},"data":data}) : helper)))
    + "\">"
    + alias4(((helper = (helper = helpers.event_name || (depth0 != null ? depth0.event_name : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"event_name","hash":{},"data":data}) : helper)))
    + "</a>.\n";
},"11":function(container,depth0,helpers,partials,data) {
    var stack1, alias1=container.lambda, alias2=container.escapeExpression;

  return "        <a class=\"user\" href=\"#user/"
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.creator : depth0)) != null ? stack1.id : stack1), depth0))
    + "\">\n          "
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.creator : depth0)) != null ? stack1.name : stack1), depth0))
    + "\n        </a>\n        создал(а)\n";
},"13":function(container,depth0,helpers,partials,data) {
    return "        Началось\n";
},"15":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return "        Голосование за\n        <a href=\"#event/"
    + alias4(((helper = (helper = helpers.scheduled_id || (depth0 != null ? depth0.scheduled_id : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"scheduled_id","hash":{},"data":data}) : helper)))
    + "\">"
    + alias4(((helper = (helper = helpers.event_name || (depth0 != null ? depth0.event_name : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"event_name","hash":{},"data":data}) : helper)))
    + "</a>\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.creator : depth0),{"name":"if","hash":{},"fn":container.program(16, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "        завершилось\n"
    + ((stack1 = helpers["if"].call(alias1,((stack1 = (depth0 != null ? depth0.data : depth0)) != null ? stack1.is_success : stack1),{"name":"if","hash":{},"fn":container.program(18, data, 0),"inverse":container.program(20, data, 0),"data":data})) != null ? stack1 : "");
},"16":function(container,depth0,helpers,partials,data) {
    var stack1, alias1=container.lambda, alias2=container.escapeExpression;

  return "        созданное\n        <a class=\"user\" href=\"#user/"
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.creator : depth0)) != null ? stack1.id : stack1), depth0))
    + "\">\n          "
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.creator : depth0)) != null ? stack1.name : stack1), depth0))
    + "\n        </a>\n";
},"18":function(container,depth0,helpers,partials,data) {
    return "        положительно.\n";
},"20":function(container,depth0,helpers,partials,data) {
    return "        отрицательно.\n";
},"22":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {};

  return "    <div class=\"extra\">\n      <div class=\""
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.unreaded_comments : depth0),{"name":"if","hash":{},"fn":container.program(23, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "comments\">\n        <i class=\"comment outline icon\"></i> "
    + container.escapeExpression(((helper = (helper = helpers.comments_count || (depth0 != null ? depth0.comments_count : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(alias1,{"name":"comments_count","hash":{},"data":data}) : helper)))
    + "\n      </div>\n    </div>\n";
},"23":function(container,depth0,helpers,partials,data) {
    return "new ";
},"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing;

  return "<div class=\"event\">\n  <div class=\"label\">\n"
    + ((stack1 = (helpers.if_equal || (depth0 && depth0.if_equal) || alias2).call(alias1,(depth0 != null ? depth0.state : depth0),"Start",{"name":"if_equal","hash":{},"fn":container.program(1, data, 0),"inverse":container.program(3, data, 0),"data":data})) != null ? stack1 : "")
    + "  </div>\n  <div class=\"content\">\n    <div class=\"date\">\n      "
    + container.escapeExpression((helpers.duration_from_now || (depth0 && depth0.duration_from_now) || alias2).call(alias1,(depth0 != null ? depth0.creation_time : depth0),{"name":"duration_from_now","hash":{},"data":data}))
    + "\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.is_new : depth0),{"name":"if","hash":{},"fn":container.program(8, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "    </div>\n    <div class=\"summary\">\n"
    + ((stack1 = (helpers.if_equal || (depth0 && depth0.if_equal) || alias2).call(alias1,(depth0 != null ? depth0.state : depth0),"Start",{"name":"if_equal","hash":{},"fn":container.program(10, data, 0),"inverse":container.program(15, data, 0),"data":data})) != null ? stack1 : "")
    + "    </div>\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.comments_count : depth0),{"name":"if","hash":{},"fn":container.program(22, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "  </div>\n</div>\n";
},"useData":true});
});
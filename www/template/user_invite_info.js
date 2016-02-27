define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['user_invite_info'] = template({"1":function(container,depth0,helpers,partials,data) {
    var stack1;

  return ((stack1 = helpers["if"].call(depth0 != null ? depth0 : {},(depth0 != null ? depth0.success : depth0),{"name":"if","hash":{},"fn":container.program(2, data, 0),"inverse":container.program(4, data, 0),"data":data})) != null ? stack1 : "");
},"2":function(container,depth0,helpers,partials,data) {
    var stack1, alias1=container.lambda, alias2=container.escapeExpression;

  return "          <i class=\"thumbs up icon\"></i>\n        </div>\n        <div class=\"label\">\n          <a href=\"#user/"
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.user : depth0)) != null ? stack1.id : stack1), depth0))
    + "\">"
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.user : depth0)) != null ? stack1.name : stack1), depth0))
    + "</a> дал согласие\n        </div>\n";
},"4":function(container,depth0,helpers,partials,data) {
    var stack1, alias1=container.lambda, alias2=container.escapeExpression;

  return "          <i class=\"thumbs down icon\"></i>\n        </div>\n        <div class=\"label\">\n          <a href=\"#user/"
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.user : depth0)) != null ? stack1.id : stack1), depth0))
    + "\">"
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.user : depth0)) != null ? stack1.name : stack1), depth0))
    + "</a> отказался\n        </div>\n";
},"6":function(container,depth0,helpers,partials,data) {
    var stack1, alias1=container.lambda, alias2=container.escapeExpression;

  return "        <i class=\"help icon\"></i>\n      </div>\n      <div class=\"label\">\n        <a href=\"#user/"
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.user : depth0)) != null ? stack1.id : stack1), depth0))
    + "\">"
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.user : depth0)) != null ? stack1.name : stack1), depth0))
    + "</a> еще не принял решение\n      </div>\n";
},"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, alias1=depth0 != null ? depth0 : {};

  return "<h4 class=\"ui header\">\n  <div class=\"content\">\n    Описание группы:\n  </div>\n</h4>\n<div class=\"ui secondary segment zeromargin description\">\n  "
    + ((stack1 = (helpers.markdown || (depth0 && depth0.markdown) || helpers.helperMissing).call(alias1,(depth0 != null ? depth0.group_description : depth0),{"name":"markdown","hash":{},"data":data})) != null ? stack1 : "")
    + "\n</div>\n<div class=\"ui center aligned basic segment\">\n  <div class=\"ui one small statistics\">\n    <div class=\"statistic\">\n      <div class=\"value\">\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.is_voted : depth0),{"name":"if","hash":{},"fn":container.program(1, data, 0),"inverse":container.program(6, data, 0),"data":data})) != null ? stack1 : "")
    + "    </div>\n  </div>\n</div>\n";
},"useData":true});
});
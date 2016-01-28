define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['change_timetable_info'] = template({"1":function(container,depth0,helpers,partials,data) {
    var stack1;

  return "      "
    + ((stack1 = (helpers.include || (depth0 && depth0.include) || helpers.helperMissing).call(depth0 != null ? depth0 : {},"timetable_value_info",depth0,{"name":"include","hash":{},"data":data})) != null ? stack1 : "")
    + "\n";
},"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {};

  return "<div class=\"ui secondary segment description\">\n  "
    + ((stack1 = ((helper = (helper = helpers.description || (depth0 != null ? depth0.description : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(alias1,{"name":"description","hash":{},"data":data}) : helper))) != null ? stack1 : "")
    + "\n</div>\n<div class=\"ui two column stackable grid\">\n  <div class=\"column\">\n    <div class=\"ui horizontal divider green header\">\n      <i class=\"plus icon\"></i>\n      Добавляются:\n    </div>\n    <div class=\"ui divided items\">\n"
    + ((stack1 = helpers.each.call(alias1,(depth0 != null ? depth0.enable : depth0),{"name":"each","hash":{},"fn":container.program(1, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "    </div>\n  </div>\n  <div class=\"column\">\n    <div class=\"ui horizontal divider red header\">\n      <i class=\"minus icon\"></i>\n      Удаляются:\n    </div>\n    <div class=\"ui divided items\">\n"
    + ((stack1 = helpers.each.call(alias1,(depth0 != null ? depth0.disable : depth0),{"name":"each","hash":{},"fn":container.program(1, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "    </div>\n  </div>\n</div>\n";
},"useData":true});
});
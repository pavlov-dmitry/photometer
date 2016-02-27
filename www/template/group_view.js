define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['group_view'] = template({"1":function(container,depth0,helpers,partials,data) {
    var helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return "  <div id=\"edit-btn\" class=\"ui top right pointing dropdown basic icon button topright\">\n    <i class=\"configure icon\"></i>\n    <div class=\"menu\">\n      <a class=\"item\" href=\"#change_timetable/"
    + alias4(((helper = (helper = helpers.id || (depth0 != null ? depth0.id : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"id","hash":{},"data":data}) : helper)))
    + "\">\n        <i class=\"calendar icon\"></i>\n        Изменить расписание\n      </a>\n      <a class=\"item\" href=\"#user_invite/"
    + alias4(((helper = (helper = helpers.id || (depth0 != null ? depth0.id : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"id","hash":{},"data":data}) : helper)))
    + "\">\n        <i class=\"add user icon\"></i>\n        Пригласить пользователя\n      </a>\n    </div>\n  </div>\n";
},"3":function(container,depth0,helpers,partials,data) {
    var helper;

  return "    <div class=\"ui label\">\n      "
    + container.escapeExpression(((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0 != null ? depth0 : {},{"name":"name","hash":{},"data":data}) : helper)))
    + "\n    </div>\n";
},"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3=container.escapeExpression;

  return "<div class=\"ui relative container\">\n  <h2 class=\"ui dividing header\">\n    <div class=\"content\">\n      <i class=\"users icon\"></i>\n      "
    + alias3(((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : alias2),(typeof helper === "function" ? helper.call(alias1,{"name":"name","hash":{},"data":data}) : helper)))
    + "\n    </div>\n  </h2>\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.editable : depth0),{"name":"if","hash":{},"fn":container.program(1, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "  <div class=\"description\">\n    <p>\n      "
    + ((stack1 = (helpers.markdown || (depth0 && depth0.markdown) || alias2).call(alias1,(depth0 != null ? depth0.description : depth0),{"name":"markdown","hash":{},"data":data})) != null ? stack1 : "")
    + "\n    </p>\n  </div>\n  <p>\n    <strong>Создана: </strong>"
    + alias3((helpers.duration_from_now || (depth0 && depth0.duration_from_now) || alias2).call(alias1,(depth0 != null ? depth0.creation_time : depth0),{"name":"duration_from_now","hash":{},"data":data}))
    + "\n  </p>\n  <h4 class=\"ui header\">\n    <div class=\"content\">\n      Участники:\n    </div>\n  </h4>\n  <div class=\"ui basic blue labels\">\n"
    + ((stack1 = helpers.each.call(alias1,(depth0 != null ? depth0.members : depth0),{"name":"each","hash":{},"fn":container.program(3, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "  </div>\n</div>\n";
},"useData":true});
});
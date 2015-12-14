define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['group_view'] = template({"1":function(container,depth0,helpers,partials,data) {
    return "    <div class=\"btn-group pull-right\" role=\"group\">\n      <button type=\"button\" class=\"btn tr-btn gray-btn dropdown-toggle\" data-toggle=\"dropdown\" aria-haspopup=\"true\" aria-expanded=\"false\">\n        <span class=\"glyphicon glyphicon-cog\"></span>\n      </button>\n      <ul class=\"dropdown-menu inverse-dropdown\">\n        <li><a href=\"#\">Изменить расписание</a></li>\n        <li><a href=\"#\">Пригласить пользователя</a></li>\n      </ul>\n    </div>\n  </h1>\n  <div style=\"clear:both\"/>\n";
},"3":function(container,depth0,helpers,partials,data) {
    var helper;

  return "    <li>"
    + container.escapeExpression(((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0 != null ? depth0 : {},{"name":"name","hash":{},"data":data}) : helper)))
    + "</li>\n";
},"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing;

  return "<div class=\"container\">\n  <h1 class=\"full-width pull-left\">"
    + container.escapeExpression(((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : alias2),(typeof helper === "function" ? helper.call(alias1,{"name":"name","hash":{},"data":data}) : helper)))
    + "\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.editable : depth0),{"name":"if","hash":{},"fn":container.program(1, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "  <div class=\"text-preview\">\n    "
    + ((stack1 = (helpers.markdown || (depth0 && depth0.markdown) || alias2).call(alias1,(depth0 != null ? depth0.description : depth0),{"name":"markdown","hash":{},"data":data})) != null ? stack1 : "")
    + "\n  </div>\n  <h4>Участники:</h4>\n  <ul>\n"
    + ((stack1 = helpers.each.call(alias1,(depth0 != null ? depth0.members : depth0),{"name":"each","hash":{},"fn":container.program(3, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "  </ul>\n</div>\n";
},"useData":true});
});
define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['one_comment_view'] = template({"1":function(container,depth0,helpers,partials,data) {
    return "      <i class=\"write icon\"></i>\n";
},"3":function(container,depth0,helpers,partials,data) {
    return "      <div class=\"ui tiny yellow label\">новый</div>\n";
},"5":function(container,depth0,helpers,partials,data) {
    return "      <a class=\"edit action\">\n        <i class=\"edit icon\"></i>\n        Редактировать\n      </a>\n";
},"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, alias1=container.lambda, alias2=container.escapeExpression, alias3=depth0 != null ? depth0 : {}, alias4=helpers.helperMissing;

  return "<div class=\"comment\">\n  <div class=\"avatar\">\n    <i class=\"big horizontally flipped comment icon\"></i>\n  </div>\n  <div class=\"content\">\n    <a class=\"author\" href=\"#user/"
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.creator : depth0)) != null ? stack1.id : stack1), depth0))
    + "\">"
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.creator : depth0)) != null ? stack1.name : stack1), depth0))
    + "</a>\n    <div class=\"metadata\">\n      <div class=\"date\">"
    + alias2((helpers.duration_from_now || (depth0 && depth0.duration_from_now) || alias4).call(alias3,(depth0 != null ? depth0.creation_time : depth0),{"name":"duration_from_now","hash":{},"data":data}))
    + "</div>\n"
    + ((stack1 = helpers["if"].call(alias3,(depth0 != null ? depth0.edit_time : depth0),{"name":"if","hash":{},"fn":container.program(1, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + ((stack1 = helpers["if"].call(alias3,(depth0 != null ? depth0.is_new : depth0),{"name":"if","hash":{},"fn":container.program(3, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "    </div>\n    <div class=\"text\">\n      <div class=\"editor\"></div>\n      <div class=\"text-content\">\n        "
    + ((stack1 = (helpers.markdown || (depth0 && depth0.markdown) || alias4).call(alias3,(depth0 != null ? depth0.text : depth0),{"name":"markdown","hash":{},"data":data})) != null ? stack1 : "")
    + "\n      </div>\n    </div>\n    <div class=\"actions\">\n      <a class=\"quote action\">\n        <i class=\"reply icon\"></i>\n        Цитировать\n      </a>\n"
    + ((stack1 = helpers["if"].call(alias3,(depth0 != null ? depth0.is_editable : depth0),{"name":"if","hash":{},"fn":container.program(5, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "    </div>\n  </div>\n</div>\n";
},"useData":true});
});
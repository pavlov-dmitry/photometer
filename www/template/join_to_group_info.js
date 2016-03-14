define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['join_to_group_info'] = template({"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, alias1=container.lambda, alias2=container.escapeExpression;

  return "<div class=\"ui secondary segment description\">\n  <h2 class=\"ui header\">\n    <div class=\"content\">\n      <i class=\"add user icon\"></i>\n      <a href=\"#user/"
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.user : depth0)) != null ? stack1.id : stack1), depth0))
    + "\">"
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.user : depth0)) != null ? stack1.name : stack1), depth0))
    + "</a>\n    </div>\n  </h2>\n  "
    + ((stack1 = (helpers.markdown || (depth0 && depth0.markdown) || helpers.helperMissing).call(depth0 != null ? depth0 : {},(depth0 != null ? depth0.text : depth0),{"name":"markdown","hash":{},"data":data})) != null ? stack1 : "")
    + "\n</div>\n";
},"useData":true});
});
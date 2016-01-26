define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['comments_view'] = template({"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3=container.escapeExpression;

  return "<h3 class=\"ui dividing header\">\n  <i class=\"comments icon\"></i>\n  Комментарии\n</h3>\n"
    + alias3((helpers.pagination || (depth0 && depth0.pagination) || alias2).call(alias1,(depth0 != null ? depth0.pagination : depth0),"",{"name":"pagination","hash":{},"data":data}))
    + "\n<div id=\"comments-list\" class=\"ui comments\">\n</div>\n"
    + alias3((helpers.pagination || (depth0 && depth0.pagination) || alias2).call(alias1,(depth0 != null ? depth0.pagination : depth0),"",{"name":"pagination","hash":{},"data":data}))
    + "\n";
},"useData":true});
});
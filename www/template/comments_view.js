define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['comments_view'] = template({"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3=container.escapeExpression;

  return "<h3 class=\"ui dividing header\">\n  <i class=\"comments icon\"></i>\n  "
    + alias3(((helper = (helper = helpers.all_count || (depth0 != null ? depth0.all_count : depth0)) != null ? helper : alias2),(typeof helper === "function" ? helper.call(alias1,{"name":"all_count","hash":{},"data":data}) : helper)))
    + " "
    + alias3((helpers.humanize || (depth0 && depth0.humanize) || alias2).call(alias1,(depth0 != null ? depth0.all_count : depth0),"Комментарий","Комментария","Комментариев",{"name":"humanize","hash":{},"data":data}))
    + "\n</h3>\n"
    + ((stack1 = (helpers.pagination || (depth0 && depth0.pagination) || alias2).call(alias1,(depth0 != null ? depth0.pagination : depth0),"",{"name":"pagination","hash":{},"data":data})) != null ? stack1 : "")
    + "\n<div id=\"comments-list\" class=\"ui comments basic segment\">\n</div>\n"
    + ((stack1 = (helpers.pagination || (depth0 && depth0.pagination) || alias2).call(alias1,(depth0 != null ? depth0.pagination : depth0),"",{"name":"pagination","hash":{},"data":data})) != null ? stack1 : "")
    + "\n<div class=\"ui hidden divider\"></div>\n";
},"useData":true});
});
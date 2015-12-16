define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['photo_view'] = template({"1":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return "<li class=\"previous"
    + ((stack1 = helpers.unless.call(alias1,(depth0 != null ? depth0.next : depth0),{"name":"unless","hash":{},"fn":container.program(2, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "\"><a href=\"#"
    + alias4(((helper = (helper = helpers.context_url || (depth0 != null ? depth0.context_url : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"context_url","hash":{},"data":data}) : helper)))
    + alias4(((helper = (helper = helpers.next || (depth0 != null ? depth0.next : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"next","hash":{},"data":data}) : helper)))
    + "\"><span aria-hidden=\"true\">&larr;</span></a></li>";
},"2":function(container,depth0,helpers,partials,data) {
    return " disabled";
},"4":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return "<li class=\"next"
    + ((stack1 = helpers.unless.call(alias1,(depth0 != null ? depth0.prev : depth0),{"name":"unless","hash":{},"fn":container.program(2, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "\"><a href=\"#"
    + alias4(((helper = (helper = helpers.context_url || (depth0 != null ? depth0.context_url : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"context_url","hash":{},"data":data}) : helper)))
    + alias4(((helper = (helper = helpers.prev || (depth0 != null ? depth0.prev : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"prev","hash":{},"data":data}) : helper)))
    + "\"><span aria-hidden=\"true\">&rarr;</span></a></li>";
},"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {};

  return "<div class=\"container\">\n  <div class=\"\">\n    <img id=\"photo\" src=\"/photo/"
    + container.escapeExpression(((helper = (helper = helpers.id || (depth0 != null ? depth0.id : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(alias1,{"name":"id","hash":{},"data":data}) : helper)))
    + ".jpg\" class=\"center-block img-responsive\"/>\n  </div>\n  <nav>\n    <ul class=\"pager\">\n      "
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.next : depth0),{"name":"if","hash":{},"fn":container.program(1, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "\n      "
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.prev : depth0),{"name":"if","hash":{},"fn":container.program(4, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "\n    </ul>\n  </nav>\n</div>\n";
},"useData":true});
});
define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['photo_view'] = template({"1":function(depth0,helpers,partials,data) {
    var stack1, helper, alias1=helpers.helperMissing, alias2="function", alias3=this.escapeExpression;

  return "<li class=\"previous"
    + ((stack1 = helpers.unless.call(depth0,(depth0 != null ? depth0.next : depth0),{"name":"unless","hash":{},"fn":this.program(2, data, 0),"inverse":this.noop,"data":data})) != null ? stack1 : "")
    + "\"><a href=\"#"
    + alias3(((helper = (helper = helpers.context_url || (depth0 != null ? depth0.context_url : depth0)) != null ? helper : alias1),(typeof helper === alias2 ? helper.call(depth0,{"name":"context_url","hash":{},"data":data}) : helper)))
    + alias3(((helper = (helper = helpers.next || (depth0 != null ? depth0.next : depth0)) != null ? helper : alias1),(typeof helper === alias2 ? helper.call(depth0,{"name":"next","hash":{},"data":data}) : helper)))
    + "\"><span aria-hidden=\"true\">&larr;</span></a></li>";
},"2":function(depth0,helpers,partials,data) {
    return " disabled";
},"4":function(depth0,helpers,partials,data) {
    var stack1, helper, alias1=helpers.helperMissing, alias2="function", alias3=this.escapeExpression;

  return "<li class=\"next"
    + ((stack1 = helpers.unless.call(depth0,(depth0 != null ? depth0.prev : depth0),{"name":"unless","hash":{},"fn":this.program(2, data, 0),"inverse":this.noop,"data":data})) != null ? stack1 : "")
    + "\"><a href=\"#"
    + alias3(((helper = (helper = helpers.context_url || (depth0 != null ? depth0.context_url : depth0)) != null ? helper : alias1),(typeof helper === alias2 ? helper.call(depth0,{"name":"context_url","hash":{},"data":data}) : helper)))
    + alias3(((helper = (helper = helpers.prev || (depth0 != null ? depth0.prev : depth0)) != null ? helper : alias1),(typeof helper === alias2 ? helper.call(depth0,{"name":"prev","hash":{},"data":data}) : helper)))
    + "\"><span aria-hidden=\"true\">&rarr;</span></a></li>";
},"compiler":[6,">= 2.0.0-beta.1"],"main":function(depth0,helpers,partials,data) {
    var stack1, helper;

  return "<div class=\"container\">\n  <div class=\"fly\">\n    <img id=\"photo\" src=\"/photo/"
    + this.escapeExpression(((helper = (helper = helpers.id || (depth0 != null ? depth0.id : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0,{"name":"id","hash":{},"data":data}) : helper)))
    + ".jpg\" class=\"img-responsive\"/>\n  </div>\n  <nav>\n    <ul class=\"pager\">\n      "
    + ((stack1 = helpers['if'].call(depth0,(depth0 != null ? depth0.next : depth0),{"name":"if","hash":{},"fn":this.program(1, data, 0),"inverse":this.noop,"data":data})) != null ? stack1 : "")
    + "\n      "
    + ((stack1 = helpers['if'].call(depth0,(depth0 != null ? depth0.prev : depth0),{"name":"if","hash":{},"fn":this.program(4, data, 0),"inverse":this.noop,"data":data})) != null ? stack1 : "")
    + "\n    </ul>\n  </nav>\n</div>\n";
},"useData":true});
});
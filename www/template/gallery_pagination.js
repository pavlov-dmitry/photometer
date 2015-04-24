define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['gallery_pagination'] = template({"1":function(depth0,helpers,partials,data) {
    return "";
},"3":function(depth0,helpers,partials,data) {
    return "class=\"disabled\"";
},"5":function(depth0,helpers,partials,data) {
    var helper;

  return "href=\""
    + this.escapeExpression(((helper = (helper = helpers.prev || (depth0 != null ? depth0.prev : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0,{"name":"prev","hash":{},"data":data}) : helper)))
    + "\"";
},"7":function(depth0,helpers,partials,data) {
    var stack1, helper;

  return "    <li "
    + ((stack1 = helpers['if'].call(depth0,(depth0 != null ? depth0.active : depth0),{"name":"if","hash":{},"fn":this.program(8, data, 0),"inverse":this.noop,"data":data})) != null ? stack1 : "")
    + ((stack1 = helpers['if'].call(depth0,(depth0 != null ? depth0.disabled : depth0),{"name":"if","hash":{},"fn":this.program(3, data, 0),"inverse":this.noop,"data":data})) != null ? stack1 : "")
    + ">\n      <a "
    + ((stack1 = helpers['if'].call(depth0,(depth0 != null ? depth0.link : depth0),{"name":"if","hash":{},"fn":this.program(10, data, 0),"inverse":this.noop,"data":data})) != null ? stack1 : "")
    + " "
    + ((stack1 = helpers['if'].call(depth0,(depth0 != null ? depth0.next : depth0),{"name":"if","hash":{},"fn":this.program(12, data, 0),"inverse":this.noop,"data":data})) != null ? stack1 : "")
    + " "
    + ((stack1 = helpers['if'].call(depth0,(depth0 != null ? depth0.next : depth0),{"name":"if","hash":{},"fn":this.program(14, data, 0),"inverse":this.noop,"data":data})) != null ? stack1 : "")
    + ">\n        "
    + this.escapeExpression(((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0,{"name":"name","hash":{},"data":data}) : helper)))
    + "\n      </a>\n    </li>\n";
},"8":function(depth0,helpers,partials,data) {
    return "class=\"active\"";
},"10":function(depth0,helpers,partials,data) {
    var helper;

  return "href=\""
    + this.escapeExpression(((helper = (helper = helpers.link || (depth0 != null ? depth0.link : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0,{"name":"link","hash":{},"data":data}) : helper)))
    + "\"";
},"12":function(depth0,helpers,partials,data) {
    return "aria-label=\"\"";
},"14":function(depth0,helpers,partials,data) {
    return "aria-label=\"Слудующая\"";
},"16":function(depth0,helpers,partials,data) {
    var helper;

  return "href=\""
    + this.escapeExpression(((helper = (helper = helpers.next || (depth0 != null ? depth0.next : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0,{"name":"next","hash":{},"data":data}) : helper)))
    + "\"";
},"compiler":[6,">= 2.0.0-beta.1"],"main":function(depth0,helpers,partials,data) {
    var stack1;

  return "<nav>\n  <ul class=\"pagination\">\n    <li "
    + ((stack1 = helpers['if'].call(depth0,(depth0 != null ? depth0.prev : depth0),{"name":"if","hash":{},"fn":this.program(1, data, 0),"inverse":this.program(3, data, 0),"data":data})) != null ? stack1 : "")
    + ">\n      <a "
    + ((stack1 = helpers['if'].call(depth0,(depth0 != null ? depth0.prev : depth0),{"name":"if","hash":{},"fn":this.program(5, data, 0),"inverse":this.noop,"data":data})) != null ? stack1 : "")
    + " aria-label=\"Предыдущая\">\n        <span aria-hidden=\"true\">&laquo;</span>\n      </a>\n    </li>\n"
    + ((stack1 = helpers.each.call(depth0,(depth0 != null ? depth0.pages : depth0),{"name":"each","hash":{},"fn":this.program(7, data, 0),"inverse":this.noop,"data":data})) != null ? stack1 : "")
    + "    <li "
    + ((stack1 = helpers['if'].call(depth0,(depth0 != null ? depth0.next : depth0),{"name":"if","hash":{},"fn":this.program(1, data, 0),"inverse":this.program(3, data, 0),"data":data})) != null ? stack1 : "")
    + ">\n      <a "
    + ((stack1 = helpers['if'].call(depth0,(depth0 != null ? depth0.next : depth0),{"name":"if","hash":{},"fn":this.program(16, data, 0),"inverse":this.noop,"data":data})) != null ? stack1 : "")
    + " aria-label=\"Слудующая\">\n        <span aria-hidden=\"true\">&raquo;</span>\n      </a>\n    </li>\n  </ul>\n</nav>\n";
},"useData":true});
});
define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['photo_preview'] = template({"1":function(depth0,helpers,partials,data) {
    var helper;

  return this.escapeExpression(((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0,{"name":"name","hash":{},"data":data}) : helper)));
},"3":function(depth0,helpers,partials,data) {
    return "Без имени";
},"5":function(depth0,helpers,partials,data) {
    return "EDITING!!!";
},"compiler":[6,">= 2.0.0-beta.1"],"main":function(depth0,helpers,partials,data) {
    var stack1, helper;

  return "<div class=\"preview panel fly-sm\">\r\n  <img src=\"preview/"
    + this.escapeExpression(((helper = (helper = helpers.id || (depth0 != null ? depth0.id : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0,{"name":"id","hash":{},"data":data}) : helper)))
    + ".png\" alt=\"Нет картинки :(\"/>\r\n  <div class=\"caption\">\r\n      <h4 class=\"pull-left\">"
    + ((stack1 = helpers['if'].call(depth0,(depth0 != null ? depth0.name : depth0),{"name":"if","hash":{},"fn":this.program(1, data, 0),"inverse":this.program(3, data, 0),"data":data})) != null ? stack1 : "")
    + "</h4>\r\n      <!-- <span class=\"input-group-btn\"> -->\r\n\r\n      <button type=\"button\" class=\"btn btn-default pull-right rename-btn\" aria-label=\"Переименовать\">\r\n        <span class=\"glyphicon glyphicon-pencil\" aria-hidden=\"true\"></span>\r\n      </button>\r\n      <!-- </span> -->\r\n      <!-- <input type=\"text\" class=\"form-control\" placeholder=\"Search for...\"> -->\r\n      "
    + ((stack1 = helpers['if'].call(depth0,(depth0 != null ? depth0.editing : depth0),{"name":"if","hash":{},"fn":this.program(5, data, 0),"inverse":this.noop,"data":data})) != null ? stack1 : "")
    + "\r\n  </div>\r\n</div>\r\n";
},"useData":true});
});
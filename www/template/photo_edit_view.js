define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['photo_edit_view'] = template({"1":function(depth0,helpers,partials,data) {
    var helper;

  return "value=\""
    + this.escapeExpression(((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0,{"name":"name","hash":{},"data":data}) : helper)))
    + "\"";
},"compiler":[6,">= 2.0.0-beta.1"],"main":function(depth0,helpers,partials,data) {
    var stack1, helper;

  return "<div class=\"container-fluid my-container row\">\n  <form id=\"rename-photo-form\" class=\"form-horizontal\" onsubmit=\"return false;\">\n    <div class=\"form-group\">\n      <div class=\"input-group fly\">\n        <input type=\"text\" class=\"form-control\" id=\"new-name-input\" placeholder=\"Имя фотографии\" "
    + ((stack1 = helpers['if'].call(depth0,(depth0 != null ? depth0.name : depth0),{"name":"if","hash":{},"fn":this.program(1, data, 0),"inverse":this.noop,"data":data})) != null ? stack1 : "")
    + ">\n        <span class=\"input-group-btn\">\n          <button class=\"btn btn-success\" type=\"submit\">Переименовать</button>\n        </span>\n      </div>\n    </div>\n  </form>\n  <div class=\"fly\">\n    <img src=\"/photo/"
    + this.escapeExpression(((helper = (helper = helpers.id || (depth0 != null ? depth0.id : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0,{"name":"id","hash":{},"data":data}) : helper)))
    + ".jpg\" class=\"img-responsive\"/>\n  <div>\n</div>\n";
},"useData":true});
});
define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['photo_edit_view'] = template({"1":function(depth0,helpers,partials,data) {
    var helper;

  return "value=\""
    + this.escapeExpression(((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0,{"name":"name","hash":{},"data":data}) : helper)))
    + "\"";
},"compiler":[6,">= 2.0.0-beta.1"],"main":function(depth0,helpers,partials,data) {
    var stack1, helper;

  return "<div class=\"container panel fly\">\n  <div class=\"row panel-body\">\n    <form id=\"rename-photo-form\" class=\"form-horizontal\">\n      <div class=\"form-group col-sm-12\">\n        <div class=\"col-sm-10\">\n          <input type=\"text\" class=\"form-control\" id=\"new-name-input\" placeholder=\"Имя фотографии\" "
    + ((stack1 = helpers['if'].call(depth0,(depth0 != null ? depth0.name : depth0),{"name":"if","hash":{},"fn":this.program(1, data, 0),"inverse":this.noop,"data":data})) != null ? stack1 : "")
    + ">\n        </div>\n        <button type=\"submit\" class=\"col-sm-2 btn btn-success\">Переименовать</button>\n      </div>\n    </form>\n    <img src=\"/photo/"
    + this.escapeExpression(((helper = (helper = helpers.id || (depth0 != null ? depth0.id : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0,{"name":"id","hash":{},"data":data}) : helper)))
    + ".jpg\" class=\"col-sm-12\" />\n  </div>\n</div>\n";
},"useData":true});
});
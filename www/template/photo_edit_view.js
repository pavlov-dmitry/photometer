define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['photo_edit_view'] = template({"1":function(container,depth0,helpers,partials,data) {
    var helper;

  return "value=\""
    + container.escapeExpression(((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0 != null ? depth0 : {},{"name":"name","hash":{},"data":data}) : helper)))
    + "\"";
},"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {};

  return "<div class=\"container\">\n  <div class=\"well\">\n    <form id=\"rename-photo-form\" class=\"form-horizontal\" onsubmit=\"return false;\">\n      <h3 class=\"form-heading\">Редактирование</h3>\n      <p>\n        <div class=\"input-group\">\n          <div class=\"input-group full-width\">\n            <input type=\"text\" class=\"form-control\" id=\"new-name-input\" placeholder=\"Имя фотографии\" "
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.name : depth0),{"name":"if","hash":{},"fn":container.program(1, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + ">\n          </div>\n          <span class=\"input-group-btn\">\n            <button class=\"btn btn-success\" type=\"submit\">Переименовать</button>\n          </span>\n        </div>\n      </p>\n    </form>\n    <div class=\"\">\n      <img id=\"photo\" src=\"/photo/"
    + container.escapeExpression(((helper = (helper = helpers.id || (depth0 != null ? depth0.id : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(alias1,{"name":"id","hash":{},"data":data}) : helper)))
    + ".jpg\" class=\"center-block img-responsive\"/>\n    </div>\n    <br>\n    <button id=\"crop-btn\" class=\"btn btn-success center-block\" type=\"button\">Изменить миниатюру</button>\n    <br>\n  </div>\n</div>\n";
},"useData":true});
});
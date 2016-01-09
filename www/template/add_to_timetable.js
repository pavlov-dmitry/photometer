define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['add_to_timetable'] = template({"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var helper;

  return "<div id=\"add-"
    + container.escapeExpression(((helper = (helper = helpers.id || (depth0 != null ? depth0.id : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0 != null ? depth0 : {},{"name":"id","hash":{},"data":data}) : helper)))
    + "\" class=\"ui inline fields\">\n  <div class=\"ui field\">\n    <div class=\"ui left icon input\">\n      <i class=\"info icon\"></i>\n      <input class=\"name-input\" type=\"text\" placeholder=\"Название\" required/>\n    </div>\n  </div>\n  <div class=\"ui field\">\n    <div class=\"ui left icon input\">\n      <i class=\"calendar icon\"></i>\n      <input class=\"datetimepicker-input\" type=\"text\" placeholder=\"Дата и время события\" required/>\n    </div>\n  </div>\n  <div class=\"ui field\">\n    <button class=\"ui icon remove button\" type=\"button\">\n      <i class=\"trash icon\"></i>\n    </button>\n  </div>\n</div>\n";
},"useData":true});
});
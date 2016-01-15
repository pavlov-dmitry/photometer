define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['timetable_value_info'] = template({"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return "<div class=\"ui item\">\n  <div class=\"content\">\n    <div class=\"header\">"
    + alias4(((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"name","hash":{},"data":data}) : helper)))
    + "</div>\n    <div class=\"meta\">Публикация</div>\n    <div class=\"description\">\n      <div class=\"ui left icon disabled input\">\n        <input type=\"text\" value=\""
    + alias4(((helper = (helper = helpers.ending_time || (depth0 != null ? depth0.ending_time : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"ending_time","hash":{},"data":data}) : helper)))
    + "\">\n        <i class=\"calendar icon\"></i>\n      </div>\n    </div>\n  </div>\n</div>\n";
},"useData":true});
});
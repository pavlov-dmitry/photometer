define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['timetable_value_info'] = template({"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3=container.escapeExpression;

  return "<div class=\"ui item\">\n  <div class=\"content\">\n    <div class=\"header\">"
    + alias3(((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : alias2),(typeof helper === "function" ? helper.call(alias1,{"name":"name","hash":{},"data":data}) : helper)))
    + "</div>\n    <div class=\"meta\">Публикация</div>\n    <div class=\"description\">\n      <div class=\"ui left icon disabled fluid input\">\n        <input type=\"text\" value=\""
    + alias3((helpers.time || (depth0 && depth0.time) || alias2).call(alias1,(depth0 != null ? depth0.ending_time : depth0),{"name":"time","hash":{},"data":data}))
    + "\">\n        <i class=\"calendar icon\"></i>\n      </div>\n    </div>\n  </div>\n</div>\n";
},"useData":true});
});
define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['remove_from_timetable'] = template({"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3=container.escapeExpression;

  return "<div class=\"ui segment zeromargin\">\n  <div class=\"ui header\">\n    <div class=\"content\">\n      "
    + alias3(((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : alias2),(typeof helper === "function" ? helper.call(alias1,{"name":"name","hash":{},"data":data}) : helper)))
    + "\n      <div class=\"sub header\">Публикация</div>\n    </div>\n  </div>\n  <div class=\"ui left icon disabled fluid input\">\n    <input type=\"text\" value=\""
    + alias3((helpers.time || (depth0 && depth0.time) || alias2).call(alias1,(depth0 != null ? depth0.ending_time : depth0),{"name":"time","hash":{},"data":data}))
    + "\">\n    <i class=\"calendar icon\"></i>\n  </div>\n  <button class=\"ui icon basic remove button topright\" type=\"button\">\n    <i class=\"close icon\"></i>\n  </button>\n</div>\n";
},"useData":true});
});
define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['add_to_timetable'] = template({"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return "<div class=\"ui inline fields\">\n  <div id=\"name-"
    + alias4(((helper = (helper = helpers.idx || (depth0 != null ? depth0.idx : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"idx","hash":{},"data":data}) : helper)))
    + "\" class=\"ui field\">\n    <div class=\"ui left icon input\">\n      <i class=\"info icon\"></i>\n      <input class=\"name-input\" type=\"text\" placeholder=\"Название\" maxlength=\"64\" required/>\n    </div>\n    <div id=\"name-"
    + alias4(((helper = (helper = helpers.idx || (depth0 != null ? depth0.idx : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"idx","hash":{},"data":data}) : helper)))
    + "-err\" class=\"ui hidden pointing red basic label\">\n    </div>\n  </div>\n  <div id=\"datetime-"
    + alias4(((helper = (helper = helpers.idx || (depth0 != null ? depth0.idx : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"idx","hash":{},"data":data}) : helper)))
    + "\" class=\"ui field\">\n    <div class=\"ui left icon input\">\n      <i class=\"calendar icon\"></i>\n      <input class=\"datetimepicker-input\" type=\"text\" placeholder=\"Дата и время события\" required/>\n    </div>\n    <div id=\"datetime-"
    + alias4(((helper = (helper = helpers.idx || (depth0 != null ? depth0.idx : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"idx","hash":{},"data":data}) : helper)))
    + "-err\" class=\"ui hidden pointing red basic label\">\n    </div>\n  </div>\n  <div class=\"ui field\">\n    <button class=\"ui icon remove button\" type=\"button\">\n      <i class=\"trash icon\"></i>\n    </button>\n  </div>\n</div>\n";
},"useData":true});
});
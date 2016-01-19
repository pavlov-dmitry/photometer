define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['publication_info'] = template({"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return "<div class=\"ui center aligned basic segment\">\n  <div class=\"ui blue basic label\">\n    Должны опубликоваться: "
    + alias4(((helper = (helper = helpers.all_count || (depth0 != null ? depth0.all_count : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"all_count","hash":{},"data":data}) : helper)))
    + "\n  </div>\n  <div class=\"ui green basic label\">\n    Опубликовалось: "
    + alias4(((helper = (helper = helpers.published || (depth0 != null ? depth0.published : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"published","hash":{},"data":data}) : helper)))
    + "\n  </div>\n</div>\n";
},"useData":true});
});
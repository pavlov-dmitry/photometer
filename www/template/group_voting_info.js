define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['group_voting_info'] = template({"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return ((stack1 = ((helper = (helper = helpers.internal_html || (depth0 != null ? depth0.internal_html : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"internal_html","hash":{},"data":data}) : helper))) != null ? stack1 : "")
    + "\n<div class=\"ui right aligned basic segment zeromargin\">\n  <div class=\"ui basic blue label\">\n    Голосуют: "
    + alias4(((helper = (helper = helpers.all_count || (depth0 != null ? depth0.all_count : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"all_count","hash":{},"data":data}) : helper)))
    + "\n  </div>\n  <div class=\"ui basic green label\">\n    За: "
    + alias4(((helper = (helper = helpers.yes || (depth0 != null ? depth0.yes : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"yes","hash":{},"data":data}) : helper)))
    + "\n  </div>\n  <div class=\"ui basic red label\">\n    Против: "
    + alias4(((helper = (helper = helpers.no || (depth0 != null ? depth0.no : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"no","hash":{},"data":data}) : helper)))
    + "\n  </div>\n  <div class=\"ui basic yellow label\">\n    Необходимо для принятия: "
    + alias4(((helper = (helper = helpers.min_success_count || (depth0 != null ? depth0.min_success_count : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"min_success_count","hash":{},"data":data}) : helper)))
    + "\n  </div>\n</div>\n";
},"useData":true});
});
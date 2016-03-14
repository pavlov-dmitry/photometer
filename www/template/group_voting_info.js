define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['group_voting_info'] = template({"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return ((stack1 = ((helper = (helper = helpers.internal_html || (depth0 != null ? depth0.internal_html : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"internal_html","hash":{},"data":data}) : helper))) != null ? stack1 : "")
    + "\n<div class=\"ui center aligned basic segment zeromargin\">\n  <div class=\"ui four small statistics\">\n    <div class=\"statistic\">\n      <div class=\"value\">\n        <i class=\"user icon\"></i>\n        "
    + alias4(((helper = (helper = helpers.all_count || (depth0 != null ? depth0.all_count : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"all_count","hash":{},"data":data}) : helper)))
    + "\n      </div>\n      <div class=\"label\">\n        Голосуют\n      </div>\n    </div>\n    <div class=\"statistic\">\n      <div class=\"value\">\n        <i class=\"thumbs up icon\"></i>\n        "
    + alias4(((helper = (helper = helpers.yes || (depth0 != null ? depth0.yes : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"yes","hash":{},"data":data}) : helper)))
    + "\n      </div>\n      <div class=\"label\">\n        За\n      </div>\n    </div>\n    <div class=\"statistic\">\n      <div class=\"value\">\n        <i class=\"thumbs down icon\"></i>\n        "
    + alias4(((helper = (helper = helpers.no || (depth0 != null ? depth0.no : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"no","hash":{},"data":data}) : helper)))
    + "\n      </div>\n      <div class=\"label\">\n        Против\n      </div>\n    </div>\n    <div class=\"statistic\">\n      <div class=\"value\">\n        <i class=\"flag icon\"></i>\n        "
    + alias4((helpers.as_percent || (depth0 && depth0.as_percent) || alias2).call(alias1,(depth0 != null ? depth0.success_coeff : depth0),{"name":"as_percent","hash":{},"data":data}))
    + "%\n      </div>\n      <div class=\"label\">\n        Необходимо для принятия\n      </div>\n    </div>\n  </div>\n</div>\n";
},"useData":true});
});
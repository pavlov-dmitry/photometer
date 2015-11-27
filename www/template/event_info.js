define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['event_info'] = template({"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return "<div class=\"container\">\n  <div class=\"\">\n    <h1 class=\"text-center text-fly\">"
    + ((stack1 = ((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"name","hash":{},"data":data}) : helper))) != null ? stack1 : "")
    + "  <small>"
    + ((stack1 = ((helper = (helper = helpers.state || (depth0 != null ? depth0.state : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"state","hash":{},"data":data}) : helper))) != null ? stack1 : "")
    + "</small></h1>\n    <p class=\"text-right\">\n      <i>\n        <strong>Начало:</strong> "
    + alias4(((helper = (helper = helpers.starting_time || (depth0 != null ? depth0.starting_time : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"starting_time","hash":{},"data":data}) : helper)))
    + "  <strong>Окончание:</strong> "
    + alias4(((helper = (helper = helpers.ending_time || (depth0 != null ? depth0.ending_time : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"ending_time","hash":{},"data":data}) : helper)))
    + "\n      </i>\n    </p>\n    <div id=\"description\" class=\"text-preview\">\n      "
    + ((stack1 = ((helper = (helper = helpers.description || (depth0 != null ? depth0.description : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"description","hash":{},"data":data}) : helper))) != null ? stack1 : "")
    + "\n    </div>\n  </div>\n  <div id=\"action\">\n    "
    + ((stack1 = ((helper = (helper = helpers.action || (depth0 != null ? depth0.action : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"action","hash":{},"data":data}) : helper))) != null ? stack1 : "")
    + "\n  </div>\n</div>\n";
},"useData":true});
});
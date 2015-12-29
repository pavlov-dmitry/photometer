define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['photo_view'] = template({"1":function(container,depth0,helpers,partials,data) {
    var helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return "    <a href=\"#"
    + alias4(((helper = (helper = helpers.context_url || (depth0 != null ? depth0.context_url : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"context_url","hash":{},"data":data}) : helper)))
    + alias4(((helper = (helper = helpers.next || (depth0 != null ? depth0.next : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"next","hash":{},"data":data}) : helper)))
    + "\">\n      <div class=\"next pager\">\n        <div class=\"inside\">\n          <i style=\"right: 20%\" class=\"massive angle right next icon\"></i>\n        </div>\n      </div>\n    </a>\n";
},"3":function(container,depth0,helpers,partials,data) {
    var helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return "    <a href=\"#"
    + alias4(((helper = (helper = helpers.context_url || (depth0 != null ? depth0.context_url : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"context_url","hash":{},"data":data}) : helper)))
    + alias4(((helper = (helper = helpers.prev || (depth0 != null ? depth0.prev : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"prev","hash":{},"data":data}) : helper)))
    + "\">\n      <div class=\"prev pager\">\n        <div class=\"inside\">\n          <i style=\"left: 20%\" class=\"massive angle left prev icon\"></i>\n        </div>\n      </div>\n    </a>\n";
},"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {};

  return "<div id=\"photo-container\">\n  <div id=\"loader\" class=\"ui active dimmer\">\n    <div class=\"ui text loader\">Загрузка</div>\n  </div>\n    <img id=\"photo\" src=\"/photo/"
    + container.escapeExpression(((helper = (helper = helpers.id || (depth0 != null ? depth0.id : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(alias1,{"name":"id","hash":{},"data":data}) : helper)))
    + ".jpg\" class=\"incenter\"/>\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.next : depth0),{"name":"if","hash":{},"fn":container.program(1, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.prev : depth0),{"name":"if","hash":{},"fn":container.program(3, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "</div>\n";
},"useData":true});
});
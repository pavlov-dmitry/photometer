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
},"5":function(container,depth0,helpers,partials,data) {
    var stack1, alias1=depth0 != null ? depth0 : {};

  return ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.camera_model : depth0),{"name":"if","hash":{},"fn":container.program(6, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.focal_length : depth0),{"name":"if","hash":{},"fn":container.program(8, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.iso : depth0),{"name":"if","hash":{},"fn":container.program(10, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.shutter_speed : depth0),{"name":"if","hash":{},"fn":container.program(12, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.aperture : depth0),{"name":"if","hash":{},"fn":container.program(14, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "");
},"6":function(container,depth0,helpers,partials,data) {
    var helper;

  return "            "
    + container.escapeExpression(((helper = (helper = helpers.camera_model || (depth0 != null ? depth0.camera_model : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0 != null ? depth0 : {},{"name":"camera_model","hash":{},"data":data}) : helper)))
    + "\n";
},"8":function(container,depth0,helpers,partials,data) {
    var helper;

  return "            "
    + container.escapeExpression(((helper = (helper = helpers.focal_length || (depth0 != null ? depth0.focal_length : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0 != null ? depth0 : {},{"name":"focal_length","hash":{},"data":data}) : helper)))
    + "mm\n";
},"10":function(container,depth0,helpers,partials,data) {
    var helper;

  return "            ISO:"
    + container.escapeExpression(((helper = (helper = helpers.iso || (depth0 != null ? depth0.iso : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0 != null ? depth0 : {},{"name":"iso","hash":{},"data":data}) : helper)))
    + "\n";
},"12":function(container,depth0,helpers,partials,data) {
    return "            "
    + container.escapeExpression((helpers.shutter || (depth0 && depth0.shutter) || helpers.helperMissing).call(depth0 != null ? depth0 : {},(depth0 != null ? depth0.shutter_speed : depth0),{"name":"shutter","hash":{},"data":data}))
    + "с\n";
},"14":function(container,depth0,helpers,partials,data) {
    return "            f"
    + container.escapeExpression((helpers.aperture || (depth0 && depth0.aperture) || helpers.helperMissing).call(depth0 != null ? depth0 : {},(depth0 != null ? depth0.aperture : depth0),{"name":"aperture","hash":{},"data":data}))
    + "\n";
},"16":function(container,depth0,helpers,partials,data) {
    return "            Данные о параметрах съёмки отсутствуют.\n";
},"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return "<div id=\"photo-container\">\n  <div id=\"loader\" class=\"ui active dimmer\">\n    <div class=\"ui text loader\">Загрузка</div>\n  </div>\n    <img id=\"photo\" src=\"/photo/"
    + alias4(((helper = (helper = helpers.id || (depth0 != null ? depth0.id : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"id","hash":{},"data":data}) : helper)))
    + ".jpg\" class=\"incenter\"/>\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.next : depth0),{"name":"if","hash":{},"fn":container.program(1, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.prev : depth0),{"name":"if","hash":{},"fn":container.program(3, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "    <div class=\"exifview\">\n      <div class=\"ui very padded basic segment\">\n        <h2 class=\"ui center aligned header\">\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.shutter_speed : depth0),{"name":"if","hash":{},"fn":container.program(5, data, 0),"inverse":container.program(16, data, 0),"data":data})) != null ? stack1 : "")
    + "        </h2>\n      </div>\n    </div>\n    <div class=\"nameviewer\">\n      <div class=\"ui very padded basic segment\">\n        <h2 class=\"ui center aligned header\">\n          "
    + alias4(((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"name","hash":{},"data":data}) : helper)))
    + "@"
    + alias4(((helper = (helper = helpers.owner_name || (depth0 != null ? depth0.owner_name : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"owner_name","hash":{},"data":data}) : helper)))
    + "\n        </h2>\n      </div>\n    </div>\n</div>\n";
},"useData":true});
});
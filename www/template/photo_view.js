define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['photo_view'] = template({"1":function(container,depth0,helpers,partials,data) {
    var stack1, alias1=container.lambda, alias2=container.escapeExpression;

  return "<div class=\"ui container\">\n  <div class=\"ui huge breadcrumb\">\n    <i class=\"big photo icon\"></i>\n    <a class=\"section\" href=\"#user/"
    + alias2(alias1(((stack1 = ((stack1 = (depth0 != null ? depth0.photo : depth0)) != null ? stack1.owner : stack1)) != null ? stack1.id : stack1), depth0))
    + "\">"
    + alias2(alias1(((stack1 = ((stack1 = (depth0 != null ? depth0.photo : depth0)) != null ? stack1.owner : stack1)) != null ? stack1.name : stack1), depth0))
    + "</a>\n    <i class=\"at divider icon\"></i>\n    <a class=\"section\" href=\"#group/feed/"
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.group : depth0)) != null ? stack1.id : stack1), depth0))
    + "\">"
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.group : depth0)) != null ? stack1.name : stack1), depth0))
    + "</a>\n    <i class=\"right chevron divider icon\"></i>\n    <a class=\"section\" href=\"#group/feed/element/"
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.feed : depth0)) != null ? stack1.id : stack1), depth0))
    + "\">"
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.feed : depth0)) != null ? stack1.name : stack1), depth0))
    + "</a>\n    <i class=\"right chevron divider icon\"></i>\n    <div class=\"active section\">"
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.photo : depth0)) != null ? stack1.name : stack1), depth0))
    + "</div>\n  </div>\n</div>\n";
},"3":function(container,depth0,helpers,partials,data) {
    var helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return "    <a href=\"#"
    + alias4(((helper = (helper = helpers.context_url || (depth0 != null ? depth0.context_url : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"context_url","hash":{},"data":data}) : helper)))
    + alias4(((helper = (helper = helpers.next || (depth0 != null ? depth0.next : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"next","hash":{},"data":data}) : helper)))
    + "\">\n      <div class=\"next pager\">\n        <div class=\"inside\">\n          <i style=\"right: 20%\" class=\"massive angle right next icon\"></i>\n        </div>\n      </div>\n    </a>\n";
},"5":function(container,depth0,helpers,partials,data) {
    var helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return "    <a href=\"#"
    + alias4(((helper = (helper = helpers.context_url || (depth0 != null ? depth0.context_url : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"context_url","hash":{},"data":data}) : helper)))
    + alias4(((helper = (helper = helpers.prev || (depth0 != null ? depth0.prev : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"prev","hash":{},"data":data}) : helper)))
    + "\">\n      <div class=\"prev pager\">\n        <div class=\"inside\">\n          <i style=\"left: 20%\" class=\"massive angle left prev icon\"></i>\n        </div>\n      </div>\n    </a>\n";
},"7":function(container,depth0,helpers,partials,data) {
    var stack1, alias1=depth0 != null ? depth0 : {};

  return ((stack1 = helpers["if"].call(alias1,((stack1 = (depth0 != null ? depth0.photo : depth0)) != null ? stack1.camera_model : stack1),{"name":"if","hash":{},"fn":container.program(8, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + ((stack1 = helpers["if"].call(alias1,((stack1 = (depth0 != null ? depth0.photo : depth0)) != null ? stack1.focal_length : stack1),{"name":"if","hash":{},"fn":container.program(10, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + ((stack1 = helpers["if"].call(alias1,((stack1 = (depth0 != null ? depth0.photo : depth0)) != null ? stack1.iso : stack1),{"name":"if","hash":{},"fn":container.program(12, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + ((stack1 = helpers["if"].call(alias1,((stack1 = (depth0 != null ? depth0.photo : depth0)) != null ? stack1.shutter_speed : stack1),{"name":"if","hash":{},"fn":container.program(14, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + ((stack1 = helpers["if"].call(alias1,((stack1 = (depth0 != null ? depth0.photo : depth0)) != null ? stack1.aperture : stack1),{"name":"if","hash":{},"fn":container.program(16, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "");
},"8":function(container,depth0,helpers,partials,data) {
    var stack1;

  return "            <i class=\"camera retro icon\"></i>\n            "
    + container.escapeExpression(container.lambda(((stack1 = (depth0 != null ? depth0.photo : depth0)) != null ? stack1.camera_model : stack1), depth0))
    + "\n";
},"10":function(container,depth0,helpers,partials,data) {
    var stack1;

  return "            "
    + container.escapeExpression(container.lambda(((stack1 = (depth0 != null ? depth0.photo : depth0)) != null ? stack1.focal_length : stack1), depth0))
    + "mm\n";
},"12":function(container,depth0,helpers,partials,data) {
    var stack1;

  return "            ISO:"
    + container.escapeExpression(container.lambda(((stack1 = (depth0 != null ? depth0.photo : depth0)) != null ? stack1.iso : stack1), depth0))
    + "\n";
},"14":function(container,depth0,helpers,partials,data) {
    var stack1;

  return "            "
    + container.escapeExpression((helpers.shutter || (depth0 && depth0.shutter) || helpers.helperMissing).call(depth0 != null ? depth0 : {},((stack1 = (depth0 != null ? depth0.photo : depth0)) != null ? stack1.shutter_speed : stack1),{"name":"shutter","hash":{},"data":data}))
    + "с\n";
},"16":function(container,depth0,helpers,partials,data) {
    var stack1;

  return "            f"
    + container.escapeExpression((helpers.aperture || (depth0 && depth0.aperture) || helpers.helperMissing).call(depth0 != null ? depth0 : {},((stack1 = (depth0 != null ? depth0.photo : depth0)) != null ? stack1.aperture : stack1),{"name":"aperture","hash":{},"data":data}))
    + "\n";
},"18":function(container,depth0,helpers,partials,data) {
    return "            Данные о параметрах съёмки отсутствуют.\n";
},"20":function(container,depth0,helpers,partials,data) {
    var stack1, alias1=container.lambda, alias2=container.escapeExpression;

  return "            <i class=\"big photo icon\"></i>\n            <a class=\"section\" href=\"#user/"
    + alias2(alias1(((stack1 = ((stack1 = (depth0 != null ? depth0.photo : depth0)) != null ? stack1.owner : stack1)) != null ? stack1.id : stack1), depth0))
    + "\">"
    + alias2(alias1(((stack1 = ((stack1 = (depth0 != null ? depth0.photo : depth0)) != null ? stack1.owner : stack1)) != null ? stack1.name : stack1), depth0))
    + "</a>\n            <i class=\"at divider icon\"></i>\n            <a class=\"section\" href=\"#group/feed/"
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.group : depth0)) != null ? stack1.id : stack1), depth0))
    + "\">"
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.group : depth0)) != null ? stack1.name : stack1), depth0))
    + "</a>\n            <i class=\"right chevron divider icon\"></i>\n            <a class=\"section\" href=\"#group/feed/element/"
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.feed : depth0)) != null ? stack1.id : stack1), depth0))
    + "\">"
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.feed : depth0)) != null ? stack1.name : stack1), depth0))
    + "</a>\n            <i class=\"right chevron divider icon\"></i>\n";
},"22":function(container,depth0,helpers,partials,data) {
    var stack1;

  return "              "
    + container.escapeExpression(container.lambda(((stack1 = (depth0 != null ? depth0.photo : depth0)) != null ? stack1.name : stack1), depth0))
    + "\n";
},"24":function(container,depth0,helpers,partials,data) {
    return "              <i>Без имени</i>\n";
},"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, alias1=depth0 != null ? depth0 : {};

  return ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.group : depth0),{"name":"if","hash":{},"fn":container.program(1, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "<div id=\"photo-container\">\n  <div id=\"loader\" class=\"ui active dimmer\">\n    <div class=\"ui text loader\">Загрузка</div>\n  </div>\n    <img id=\"photo\" src=\"/photo/"
    + container.escapeExpression(container.lambda(((stack1 = (depth0 != null ? depth0.photo : depth0)) != null ? stack1.id : stack1), depth0))
    + ".jpg\" class=\"incenter\"/>\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.next : depth0),{"name":"if","hash":{},"fn":container.program(3, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.prev : depth0),{"name":"if","hash":{},"fn":container.program(5, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "    <div class=\"exifview\">\n      <div class=\"ui very padded basic segment\">\n        <h2 class=\"ui center aligned header\">\n"
    + ((stack1 = helpers["if"].call(alias1,((stack1 = (depth0 != null ? depth0.photo : depth0)) != null ? stack1.shutter_speed : stack1),{"name":"if","hash":{},"fn":container.program(7, data, 0),"inverse":container.program(18, data, 0),"data":data})) != null ? stack1 : "")
    + "        </h2>\n      </div>\n    </div>\n    <div class=\"nameviewer\">\n      <div class=\"ui very padded center aligned basic segment\">\n          <div class=\"ui huge breadcrumb\">\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.group : depth0),{"name":"if","hash":{},"fn":container.program(20, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "            <div class=\"active section\">\n"
    + ((stack1 = helpers["if"].call(alias1,((stack1 = (depth0 != null ? depth0.photo : depth0)) != null ? stack1.name : stack1),{"name":"if","hash":{},"fn":container.program(22, data, 0),"inverse":container.program(24, data, 0),"data":data})) != null ? stack1 : "")
    + "            </div>\n          </div>\n      </div>\n    </div>\n</div>\n";
},"useData":true});
});
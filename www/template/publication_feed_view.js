define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['publication_feed_view'] = template({"1":function(container,depth0,helpers,partials,data) {
    return "    <i class=\"announcement icon\"></i>\n";
},"3":function(container,depth0,helpers,partials,data) {
    return "    <i class=\"film icon\"></i>\n";
},"5":function(container,depth0,helpers,partials,data) {
    return "      <div class=\"ui yellow label\">новое</div>\n";
},"7":function(container,depth0,helpers,partials,data) {
    var helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return "      Можно начинать выкладываться\n      <a href=\"#event/"
    + alias4(((helper = (helper = helpers.scheduled_id || (depth0 != null ? depth0.scheduled_id : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"scheduled_id","hash":{},"data":data}) : helper)))
    + "\">"
    + alias4(((helper = (helper = helpers.event_name || (depth0 != null ? depth0.event_name : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"event_name","hash":{},"data":data}) : helper)))
    + "</a>\n";
},"9":function(container,depth0,helpers,partials,data) {
    var helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return "      Фотографии с <a href=\"#event/"
    + alias4(((helper = (helper = helpers.scheduled_id || (depth0 != null ? depth0.scheduled_id : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"scheduled_id","hash":{},"data":data}) : helper)))
    + "\">"
    + alias4(((helper = (helper = helpers.event_name || (depth0 != null ? depth0.event_name : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"event_name","hash":{},"data":data}) : helper)))
    + "</a>\n";
},"11":function(container,depth0,helpers,partials,data) {
    var stack1;

  return "    <div class=\"extra\">\n      <div class=\"ui stackable cards\">\n"
    + ((stack1 = helpers.each.call(depth0 != null ? depth0 : {},(depth0 != null ? depth0.photos : depth0),{"name":"each","hash":{},"fn":container.program(12, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "      </div>\n    </div>\n";
},"12":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return "        <div class=\"ui card mw300\">\n          <div class=\"content\">\n            <a href=\"#user/"
    + alias4(((helper = (helper = helpers.owner_id || (depth0 != null ? depth0.owner_id : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"owner_id","hash":{},"data":data}) : helper)))
    + "\" class=\"right floated author\">\n              @"
    + alias4(((helper = (helper = helpers.owner_name || (depth0 != null ? depth0.owner_name : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"owner_name","hash":{},"data":data}) : helper)))
    + "\n            </a>\n          </div>\n          <div class=\"image\">\n            <img exify_intitialized=\"true\" src=\"preview/"
    + alias4(((helper = (helper = helpers.id || (depth0 != null ? depth0.id : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"id","hash":{},"data":data}) : helper)))
    + ".png\" alt=\"Загружается ...\">\n          </div>\n          <div class=\"content\">\n            <div class=\"ui "
    + ((stack1 = helpers.unless.call(alias1,(depth0 != null ? depth0.name : depth0),{"name":"unless","hash":{},"fn":container.program(13, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "header\">\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.name : depth0),{"name":"if","hash":{},"fn":container.program(15, data, 0),"inverse":container.program(17, data, 0),"data":data})) != null ? stack1 : "")
    + "            </div>\n          </div>\n        </div>\n";
},"13":function(container,depth0,helpers,partials,data) {
    return "disabled ";
},"15":function(container,depth0,helpers,partials,data) {
    var helper;

  return "              "
    + container.escapeExpression(((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0 != null ? depth0 : {},{"name":"name","hash":{},"data":data}) : helper)))
    + "\n";
},"17":function(container,depth0,helpers,partials,data) {
    return "              <i>Без имени</i>\n";
},"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing;

  return "<div class=\"event\">\n  <div class=\"label\">\n"
    + ((stack1 = (helpers.if_equal || (depth0 && depth0.if_equal) || alias2).call(alias1,(depth0 != null ? depth0.state : depth0),"Start",{"name":"if_equal","hash":{},"fn":container.program(1, data, 0),"inverse":container.program(3, data, 0),"data":data})) != null ? stack1 : "")
    + "  </div>\n  <div class=\"content\">\n    <div class=\"date\">\n      "
    + container.escapeExpression((helpers.duration_from_now || (depth0 && depth0.duration_from_now) || alias2).call(alias1,(depth0 != null ? depth0.creation_time : depth0),{"name":"duration_from_now","hash":{},"data":data}))
    + "\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.is_new : depth0),{"name":"if","hash":{},"fn":container.program(5, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "    </div>\n    <div class=\"summary\">\n"
    + ((stack1 = (helpers.if_equal || (depth0 && depth0.if_equal) || alias2).call(alias1,(depth0 != null ? depth0.state : depth0),"Start",{"name":"if_equal","hash":{},"fn":container.program(7, data, 0),"inverse":container.program(9, data, 0),"data":data})) != null ? stack1 : "")
    + "    </div>\n"
    + ((stack1 = (helpers.if_equal || (depth0 && depth0.if_equal) || alias2).call(alias1,(depth0 != null ? depth0.state : depth0),"Finish",{"name":"if_equal","hash":{},"fn":container.program(11, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "</div>\n";
},"useData":true});
});
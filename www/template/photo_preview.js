define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['photo_preview'] = template({"1":function(container,depth0,helpers,partials,data) {
    return "disabled ";
},"3":function(container,depth0,helpers,partials,data) {
    var helper;

  return "                "
    + container.escapeExpression(((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0 != null ? depth0 : {},{"name":"name","hash":{},"data":data}) : helper)))
    + "\r\n";
},"5":function(container,depth0,helpers,partials,data) {
    return "                <i>Без имени</i>\r\n";
},"7":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {};

  return "            <div class=\"right floated "
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.unreaded_comments : depth0),{"name":"if","hash":{},"fn":container.program(8, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "comments\">\r\n                    <i class=\"comment "
    + ((stack1 = helpers.unless.call(alias1,(depth0 != null ? depth0.unreaded_comments : depth0),{"name":"unless","hash":{},"fn":container.program(10, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "icon\"></i> "
    + container.escapeExpression(((helper = (helper = helpers.comments_count || (depth0 != null ? depth0.comments_count : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(alias1,{"name":"comments_count","hash":{},"data":data}) : helper)))
    + "\r\n                    </div>\r\n";
},"8":function(container,depth0,helpers,partials,data) {
    return "new ";
},"10":function(container,depth0,helpers,partials,data) {
    return "outline ";
},"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return "<div id=\""
    + alias4(((helper = (helper = helpers.id || (depth0 != null ? depth0.id : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"id","hash":{},"data":data}) : helper)))
    + "\" class=\"centered card mw300\">\r\n    <a class=\"image\" href=\"#"
    + alias4(((helper = (helper = helpers.url || (depth0 != null ? depth0.url : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"url","hash":{},"data":data}) : helper)))
    + "\">\r\n        <img exify_intitialized=\"true\" src=\"i/dummy_preview.png\" data-src=\"preview/"
    + alias4(((helper = (helper = helpers.id || (depth0 != null ? depth0.id : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"id","hash":{},"data":data}) : helper)))
    + ".png\" alt=\"Нет картинки :(\">\r\n    </a>\r\n    <div class=\"content\">\r\n        <div class=\"ui "
    + ((stack1 = helpers.unless.call(alias1,(depth0 != null ? depth0.name : depth0),{"name":"unless","hash":{},"fn":container.program(1, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "header\">\r\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.name : depth0),{"name":"if","hash":{},"fn":container.program(3, data, 0),"inverse":container.program(5, data, 0),"data":data})) != null ? stack1 : "")
    + "        </div>\r\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.comments_count : depth0),{"name":"if","hash":{},"fn":container.program(7, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "                <div class=\"meta\">\r\n                    <span class=\"date\">Загружено "
    + alias4((helpers.duration_from_now || (depth0 && depth0.duration_from_now) || alias2).call(alias1,(depth0 != null ? depth0.upload_time : depth0),{"name":"duration_from_now","hash":{},"data":data}))
    + "</span>\r\n                </div>\r\n            </div>\r\n            <div class=\"extra content\">\r\n                <a href=\"#edit_photo/"
    + alias4(((helper = (helper = helpers.id || (depth0 != null ? depth0.id : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"id","hash":{},"data":data}) : helper)))
    + "\">\r\n                    <i class=\"edit icon\"></i>\r\n                    Редактировать\r\n                </a>\r\n            </div>\r\n        </div>\r\n";
},"useData":true});
});
define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['gallery_view'] = template({"1":function(container,depth0,helpers,partials,data) {
    return "    <span id=\"upload-btn\" class=\"ui green button fileinput-button\">\r\n        <i class=\"add icon\"></i>\r\n        <span>Добавить</span>\r\n        <input id=\"upload-file\" type=\"file\" name=\"file\" accept=\"image/jpeg,image/png\">\r\n    </span>\r\n\r\n    <div id=\"upload-progress\" class=\"ui indicating progress\">\r\n        <div class=\"bar\"></div>\r\n        <div class=\"label\">Загружено</div>\r\n    </div>\r\n";
},"3":function(container,depth0,helpers,partials,data) {
    var stack1, alias1=container.lambda, alias2=container.escapeExpression;

  return "    <h2 class=\"ui block header\">\r\n        <i class=\"camera icon\"></i>\r\n        <a class=\"content\" href=\"#user/"
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.owner : depth0)) != null ? stack1.id : stack1), depth0))
    + "\">\r\n            "
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.owner : depth0)) != null ? stack1.name : stack1), depth0))
    + "\r\n        </a>\r\n    </h2>\r\n";
},"5":function(container,depth0,helpers,partials,data,blockParams,depths) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return "        <div id=\""
    + alias4(((helper = (helper = helpers.id || (depth0 != null ? depth0.id : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"id","hash":{},"data":data}) : helper)))
    + "\" class=\"centered card mw300\">\r\n            <a class=\"image\" href=\"#gallery_photo/"
    + alias4(container.lambda(((stack1 = (depths[1] != null ? depths[1].owner : depths[1])) != null ? stack1.id : stack1), depth0))
    + "/"
    + alias4(((helper = (helper = helpers.id || (depth0 != null ? depth0.id : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"id","hash":{},"data":data}) : helper)))
    + "\">\r\n                <img exify_intitialized=\"true\" src=\"i/dummy_preview.png\" data-src=\"preview/"
    + alias4(((helper = (helper = helpers.id || (depth0 != null ? depth0.id : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"id","hash":{},"data":data}) : helper)))
    + ".png\" alt=\"Нет картинки :(\">\r\n            </a>\r\n            <div class=\"content\">\r\n                <div class=\"ui "
    + ((stack1 = helpers.unless.call(alias1,(depth0 != null ? depth0.name : depth0),{"name":"unless","hash":{},"fn":container.program(6, data, 0, blockParams, depths),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + " header\">\r\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.name : depth0),{"name":"if","hash":{},"fn":container.program(8, data, 0, blockParams, depths),"inverse":container.program(10, data, 0, blockParams, depths),"data":data})) != null ? stack1 : "")
    + "                </div>\r\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.comments_count : depth0),{"name":"if","hash":{},"fn":container.program(12, data, 0, blockParams, depths),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "                <div class=\"meta\">\r\n                    <span class=\"date\">Загружено "
    + alias4((helpers.duration_from_now || (depth0 && depth0.duration_from_now) || alias2).call(alias1,(depth0 != null ? depth0.upload_time : depth0),{"name":"duration_from_now","hash":{},"data":data}))
    + "</span>\r\n                </div>\r\n            </div>\r\n"
    + ((stack1 = helpers["if"].call(alias1,(depths[1] != null ? depths[1].is_own : depths[1]),{"name":"if","hash":{},"fn":container.program(17, data, 0, blockParams, depths),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "        </div>\r\n";
},"6":function(container,depth0,helpers,partials,data) {
    return " disabled ";
},"8":function(container,depth0,helpers,partials,data) {
    var helper;

  return "                        "
    + container.escapeExpression(((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0 != null ? depth0 : {},{"name":"name","hash":{},"data":data}) : helper)))
    + "\r\n";
},"10":function(container,depth0,helpers,partials,data) {
    return "                        <i>Без имени</i>\r\n";
},"12":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {};

  return "                <div class=\"right floated "
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.unreaded_comments : depth0),{"name":"if","hash":{},"fn":container.program(13, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + " comments\">\r\n                    <i class=\"comment "
    + ((stack1 = helpers.unless.call(alias1,(depth0 != null ? depth0.unreaded_comments : depth0),{"name":"unless","hash":{},"fn":container.program(15, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + " icon\"></i>\r\n                    "
    + container.escapeExpression(((helper = (helper = helpers.comments_count || (depth0 != null ? depth0.comments_count : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(alias1,{"name":"comments_count","hash":{},"data":data}) : helper)))
    + "\r\n                </div>\r\n";
},"13":function(container,depth0,helpers,partials,data) {
    return " new ";
},"15":function(container,depth0,helpers,partials,data) {
    return " outline ";
},"17":function(container,depth0,helpers,partials,data) {
    var helper;

  return "            <div class=\"extra content\">\r\n                <a href=\"#edit_photo/"
    + container.escapeExpression(((helper = (helper = helpers.id || (depth0 != null ? depth0.id : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0 != null ? depth0 : {},{"name":"id","hash":{},"data":data}) : helper)))
    + "\">\r\n                    <i class=\"edit icon\"></i>\r\n                    Редактировать\r\n                </a>\r\n            </div>\r\n";
},"19":function(container,depth0,helpers,partials,data) {
    var stack1;

  return "        </div>\r\n        <div class=\"ui center aligned very padded basic segment\">\r\n            <h1 class=\"ui disabled icon header\">\r\n                <i class=\"film icon\"></i>\r\n                Галлерея пуста.\r\n            </h1>\r\n"
    + ((stack1 = helpers["if"].call(depth0 != null ? depth0 : {},(depth0 != null ? depth0.is_own : depth0),{"name":"if","hash":{},"fn":container.program(20, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "        </div>\r\n";
},"20":function(container,depth0,helpers,partials,data) {
    return "            <h3 class=\"ui disabled header\">Начните её заполнять, нажав на кнопку \"Добавить\", в левом верхнем углу.</h3>\r\n";
},"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data,blockParams,depths) {
    var stack1, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing;

  return "<div class=\"ui container\">\r\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.is_own : depth0),{"name":"if","hash":{},"fn":container.program(1, data, 0, blockParams, depths),"inverse":container.program(3, data, 0, blockParams, depths),"data":data})) != null ? stack1 : "")
    + "    <p>\r\n    "
    + ((stack1 = (helpers.pagination || (depth0 && depth0.pagination) || alias2).call(alias1,(depth0 != null ? depth0.pagination : depth0),(depth0 != null ? depth0.prefix_url : depth0),{"name":"pagination","hash":{},"data":data})) != null ? stack1 : "")
    + "\r\n    </p>\r\n    <div id=\"preview-list\" class=\"ui stackable cards\">\r\n"
    + ((stack1 = helpers.each.call(alias1,(depth0 != null ? depth0.photos : depth0),{"name":"each","hash":{},"fn":container.program(5, data, 0, blockParams, depths),"inverse":container.program(19, data, 0, blockParams, depths),"data":data})) != null ? stack1 : "")
    + "    </div>\r\n    <p>\r\n    "
    + ((stack1 = (helpers.pagination || (depth0 && depth0.pagination) || alias2).call(alias1,(depth0 != null ? depth0.pagination : depth0),(depth0 != null ? depth0.prefix_url : depth0),{"name":"pagination","hash":{},"data":data})) != null ? stack1 : "")
    + "\r\n    </p>\r\n</div>\r\n";
},"useData":true,"useDepths":true});
});
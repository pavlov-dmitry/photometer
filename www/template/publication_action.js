define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['publication_action'] = template({"1":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return "    <div id=\"card-"
    + alias4(((helper = (helper = helpers.id || (depth0 != null ? depth0.id : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"id","hash":{},"data":data}) : helper)))
    + "\" class=\"centered card mw300\">\n      <div class=\"blurring dimmable image\">\n        <div class=\"ui dimmer\">\n          <div class=\"content\">\n            <div class=\"center\">\n              <div class=\"ui primary publish button\" data=\""
    + alias4(((helper = (helper = helpers.id || (depth0 != null ? depth0.id : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"id","hash":{},"data":data}) : helper)))
    + "\">Опубликовать</div>\n            </div>\n          </div>\n        </div>\n        <img exify_intitialized=\"true\" src=\"preview/"
    + alias4(((helper = (helper = helpers.id || (depth0 != null ? depth0.id : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"id","hash":{},"data":data}) : helper)))
    + ".png\" alt=\"Нет картинки :(\">\n      </div>\n      <div class=\"content\">\n        <div class=\"ui "
    + ((stack1 = helpers.unless.call(alias1,(depth0 != null ? depth0.name : depth0),{"name":"unless","hash":{},"fn":container.program(2, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "header\">\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.name : depth0),{"name":"if","hash":{},"fn":container.program(4, data, 0),"inverse":container.program(6, data, 0),"data":data})) != null ? stack1 : "")
    + "        </div>\n        <div class=\"meta\">\n          <span class=\"date\">Загружено "
    + alias4((helpers.duration_from_now || (depth0 && depth0.duration_from_now) || alias2).call(alias1,(depth0 != null ? depth0.upload_time : depth0),{"name":"duration_from_now","hash":{},"data":data}))
    + "</span>\n        </div>\n      </div>\n    </div>\n";
},"2":function(container,depth0,helpers,partials,data) {
    return "disabled ";
},"4":function(container,depth0,helpers,partials,data) {
    var helper;

  return "          "
    + container.escapeExpression(((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0 != null ? depth0 : {},{"name":"name","hash":{},"data":data}) : helper)))
    + "\n";
},"6":function(container,depth0,helpers,partials,data) {
    return "          <i>Без имени</i>\n";
},"8":function(container,depth0,helpers,partials,data) {
    return "    </div>\n    <div class=\"ui center aligned very padded basic segment\">\n      <h1 class=\"ui disabled icon header\">\n        <i class=\"film icon\"></i>\n        У вам нет неопубликованных фотографии.\n      </h1>\n      <h3 class=\"ui disabled header\">\n        Попробуйте загрузить новое фото в свою <a href=\"#gallery\">Галлерею</a>\n      </h3>\n    </div>\n";
},"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing;

  return "<div id=\"photos-container\" class=\"ui container basic segment nopadmar\">\n  <p>\n    "
    + ((stack1 = (helpers.pagination || (depth0 && depth0.pagination) || alias2).call(alias1,(depth0 != null ? depth0.pagination : depth0),"",{"name":"pagination","hash":{},"data":data})) != null ? stack1 : "")
    + "\n  </p>\n  <div class=\"ui stackable cards\">\n"
    + ((stack1 = helpers.each.call(alias1,(depth0 != null ? depth0.photos : depth0),{"name":"each","hash":{},"fn":container.program(1, data, 0),"inverse":container.program(8, data, 0),"data":data})) != null ? stack1 : "")
    + "  </div>\n  <p>\n    "
    + ((stack1 = (helpers.pagination || (depth0 && depth0.pagination) || alias2).call(alias1,(depth0 != null ? depth0.pagination : depth0),"",{"name":"pagination","hash":{},"data":data})) != null ? stack1 : "")
    + "\n  </p>\n</div>\n";
},"useData":true});
});
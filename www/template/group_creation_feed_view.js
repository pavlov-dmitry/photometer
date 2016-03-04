define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['group_creation_feed_view'] = template({"1":function(container,depth0,helpers,partials,data) {
    return "      <div class=\"ui yellow label\">новое</div>\n";
},"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, alias1=depth0 != null ? depth0 : {}, alias2=container.escapeExpression, alias3=container.lambda;

  return "<div class=\"event\">\n  <div class=\"label\">\n    <i class=\"child icon\"></i>\n  </div>\n  <div class=\"content\">\n    <div class=\"date\">\n      "
    + alias2((helpers.duration_from_now || (depth0 && depth0.duration_from_now) || helpers.helperMissing).call(alias1,(depth0 != null ? depth0.creation_time : depth0),{"name":"duration_from_now","hash":{},"data":data}))
    + "\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.is_new : depth0),{"name":"if","hash":{},"fn":container.program(1, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "    </div>\n    <div class=\"summary\">\n      Группа <a href=\"#group/info/"
    + alias2(alias3(((stack1 = (depth0 != null ? depth0.group : depth0)) != null ? stack1.id : stack1), depth0))
    + "\">"
    + alias2(alias3(((stack1 = (depth0 != null ? depth0.group : depth0)) != null ? stack1.name : stack1), depth0))
    + "</a> создана!\n    </div>\n    <div class=\"fluid extra text\">\n      Это лента событий группы. Выше этого сообщения будут появляться разные события группы. Для того что-бы начать, стоит добавить в расписание группы несколько событий. Это можно сделать нажав на <i class=\"configure icon\"></i>\"гаечный ключ\" на странице информации о группе <a href=\"#group/info/"
    + alias2(alias3(((stack1 = (depth0 != null ? depth0.group : depth0)) != null ? stack1.id : stack1), depth0))
    + "\">"
    + alias2(alias3(((stack1 = (depth0 != null ? depth0.group : depth0)) != null ? stack1.name : stack1), depth0))
    + "</a>.\n    </div>\n</div>\n";
},"useData":true});
});
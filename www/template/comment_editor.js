define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['comment_editor'] = template({"1":function(container,depth0,helpers,partials,data) {
    return "active ";
},"3":function(container,depth0,helpers,partials,data) {
    var stack1;

  return ((stack1 = helpers["if"].call(depth0 != null ? depth0 : {},(depth0 != null ? depth0.text : depth0),{"name":"if","hash":{},"fn":container.program(4, data, 0),"inverse":container.program(6, data, 0),"data":data})) != null ? stack1 : "");
},"4":function(container,depth0,helpers,partials,data) {
    var stack1;

  return "    "
    + ((stack1 = (helpers.markdown || (depth0 && depth0.markdown) || helpers.helperMissing).call(depth0 != null ? depth0 : {},(depth0 != null ? depth0.text : depth0),{"name":"markdown","hash":{},"data":data})) != null ? stack1 : "")
    + "\n";
},"6":function(container,depth0,helpers,partials,data) {
    return "    <i>Комментарий пуст</i>\n";
},"8":function(container,depth0,helpers,partials,data) {
    var helper;

  return "    <div class=\"ui field\">\n      <div class=\"ui basic right aligned segment nopadmar\">\n        <a class=\"thumb markdown\">markdown?</a>\n      </div>\n      <textarea class=\"comment-body\" maxlength=\"1024\" placeholder=\"Оставьте здесь комментарий ... (поддерживается форматирование markdown)\" required>"
    + container.escapeExpression(((helper = (helper = helpers.text || (depth0 != null ? depth0.text : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0 != null ? depth0 : {},{"name":"text","hash":{},"data":data}) : helper)))
    + "</textarea>\n    </div>\n";
},"10":function(container,depth0,helpers,partials,data) {
    return "    Редактировать\n";
},"12":function(container,depth0,helpers,partials,data) {
    return "    Комментировать\n";
},"14":function(container,depth0,helpers,partials,data) {
    return "  <button class=\"ui cancel button\" type=\"button\">\n    Отменить\n  </button>\n";
},"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, alias1=depth0 != null ? depth0 : {};

  return "<form class=\"ui form\">\n  <div class=\"ui top attached tabular menu\">\n    <a class=\""
    + ((stack1 = helpers.unless.call(alias1,(depth0 != null ? depth0.is_preview : depth0),{"name":"unless","hash":{},"fn":container.program(1, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "edit tab item\">Редактировать</a>\n    <a class=\""
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.is_preview : depth0),{"name":"if","hash":{},"fn":container.program(1, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "preview tab item\">Предпросмотр</a>\n  </div>\n  <div class=\"ui bottom attached tab segment active\">\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.is_preview : depth0),{"name":"if","hash":{},"fn":container.program(3, data, 0),"inverse":container.program(8, data, 0),"data":data})) != null ? stack1 : "")
    + "  </div>\n  <div class=\"ui tiny hidden divider\"></div>\n  <button class=\"ui primary submit labeled icon button\" type=\"submit\">\n    <i class=\"edit icon\"></i>\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.cancelable : depth0),{"name":"if","hash":{},"fn":container.program(10, data, 0),"inverse":container.program(12, data, 0),"data":data})) != null ? stack1 : "")
    + "  </button>\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.cancelable : depth0),{"name":"if","hash":{},"fn":container.program(14, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "</form>\n"
    + ((stack1 = (helpers.include || (depth0 && depth0.include) || helpers.helperMissing).call(alias1,"markdown_help",{"name":"include","hash":{},"data":data})) != null ? stack1 : "")
    + "\n";
},"useData":true});
});
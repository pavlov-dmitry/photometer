define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['group_creation'] = template({"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return "<div class=\"container\">\n  <form id=\"form-group-creation\" onsubmit=\"return false;\" role=\"group-creation\">\n    <h3>Создание новой группы</h3>\n    <div class=\"row\">\n      <div class=\"col-sm-9\">\n        <p>\n          <input type=\"text\" class=\"form-control\" id=\"name-input\" placeholder=\"Имя группы\" value=\""
    + alias4(((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"name","hash":{},"data":data}) : helper)))
    + "\" required autofocus>\n        </p>\n        <p>\n          <textarea type=\"text\" class=\"form-control\" id=\"description-input\" placeholder=\"Подробное описание, с поддержкой формата markdown.\" rows=\"10\" value=\""
    + alias4(((helper = (helper = helpers.description || (depth0 != null ? depth0.description : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"description","hash":{},"data":data}) : helper)))
    + "\" required/>\n          <h4>Предпросмотр описания:</h4>\n          <div class=\"text-preview full-width\">\n            <p>\n              <div id=\"info-preview\" >\n              </div>\n            </p>\n          </div>\n        </p>\n      </div>\n      <div class=\"col-sm-3\">\n        <button id=\"add-member-btn\" class=\"btn btn-primary\" type=\"button\">Добавить пользователя</button>\n        <h4>Пригласить:</h4>\n        <div id=\"users-list\">\n        </div>\n      </div>\n    </div>\n    <p>\n      <button id=\"create-btn\" class=\"btn btn-success fly\" type=\"submit\">Создать</button>\n    </p>\n  </form>\n</div>\n";
},"useData":true});
});
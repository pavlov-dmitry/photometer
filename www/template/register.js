define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['register'] = template({"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return "<div class=\"container panel small fly\">\r\n  <p>\r\n    <form id=\"form-register\" onsubmit=\"return false;\" role=\"register\">\r\n      <h3 class=\"form-heading\">Регистрация</h3>\r\n      <p><input id=\"form-reg-name\" type=\"text\" class=\"form-control\" placeholder=\"Имя\" value=\""
    + alias4(((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"name","hash":{},"data":data}) : helper)))
    + "\" required autofocus /></p>\r\n      <p><input id=\"form-reg-pasw\" type=\"password\" class=\"form-control\" placeholder=\"Пароль\" required /></p>\r\n      <p><input id=\"form-reg-mail\" type=\"email\" class=\"form-control\" placeholder=\"Почта\" value=\""
    + alias4(((helper = (helper = helpers.email || (depth0 != null ? depth0.email : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"email","hash":{},"data":data}) : helper)))
    + "\" required autofocus /></p>\r\n      <div id=\"form-reg-error\" class=\"alert alert-danger hidden\"><strong>Ошибка:</strong> "
    + alias4(((helper = (helper = helpers.error || (depth0 != null ? depth0.error : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"error","hash":{},"data":data}) : helper)))
    + "</div>\r\n      <div id=\"form-reg-info\" class=\"alert alert-success hidden\">Регистрация прошла успешно, проверьте почту что-бы завершить регистрацию</div>\r\n      <button id=\"form-reg-btn\" class=\"btn btn-lg btn-primary btn-block\" type=\"submit\">Регистрация</button>\r\n    </form>\r\n  </p>\r\n</div>\r\n";
},"useData":true});
});
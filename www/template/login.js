define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['login'] = template({"compiler":[6,">= 2.0.0-beta.1"],"main":function(depth0,helpers,partials,data) {
    var helper, alias1=helpers.helperMissing, alias2="function", alias3=this.escapeExpression;

  return "<div id=\"login-container\" class=\"container panel small fly\">\r\n    <p>\r\n      <form id=\"form-login\" onsubmit=\"return false;\" role=\"login\">\r\n            <h3 class=\"form-heading\">Вход</h3>\r\n            <p><input id=\"login-name\" type=\"text\" class=\"form-control\" placeholder=\"Имя\" value=\""
    + alias3(((helper = (helper = helpers.user || (depth0 != null ? depth0.user : depth0)) != null ? helper : alias1),(typeof helper === alias2 ? helper.call(depth0,{"name":"user","hash":{},"data":data}) : helper)))
    + "\"required autofocus /></p>\r\n            <p><input id=\"login-pasw\" type=\"password\" class=\"form-control\" placeholder=\"Пароль\" required /></p>\r\n        <!-- div class=\"checkbox\">\r\n            <label class=\"checkbox\">\r\n              <input type=\"checkbox\" value=\"remember-me\">Запомнить меня\r\n            </label>\r\n        </div -->\r\n        <div id=\"login-error\" class=\"alert alert-danger hidden\"><strong>Ошибка:</strong> "
    + alias3(((helper = (helper = helpers.error || (depth0 != null ? depth0.error : depth0)) != null ? helper : alias1),(typeof helper === alias2 ? helper.call(depth0,{"name":"error","hash":{},"data":data}) : helper)))
    + "</div>\r\n        <button id=\"login-btn\" class=\"btn btn-lg btn-success btn-block\" type=\"submit\">ВХОД</button>\r\n        <p><br>Еще не с нами? <a href=\"#register\">Пройди регистрацию</a>, чтобы присоединиться.</p>\r\n      </form>\r\n    </p>\r\n</div>";
},"useData":true});
});
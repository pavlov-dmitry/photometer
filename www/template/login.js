define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['login'] = template({"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return "<div class=\"container\">\r\n  <div class=\"panel panel-default small center-block\">\r\n    <div class=\"panel-body\">\r\n      <form id=\"form-login\" onsubmit=\"return false;\" role=\"login\">\r\n        <h3 class=\"form-heading\">Вход</h3>\r\n        <div class=\"form-group\">\r\n          <input id=\"login-name\" type=\"text\" class=\"form-control\" placeholder=\"Имя\" value=\""
    + alias4(((helper = (helper = helpers.user || (depth0 != null ? depth0.user : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"user","hash":{},"data":data}) : helper)))
    + "\"required autofocus />\r\n        </div>\r\n        <div class=\"form-group\">\r\n          <input id=\"login-pasw\" type=\"password\" class=\"form-control\" placeholder=\"Пароль\" required />\r\n        </div>\r\n        <div id=\"login-error\" class=\"alert alert-danger hidden\"><strong>Ошибка:</strong> "
    + alias4(((helper = (helper = helpers.error || (depth0 != null ? depth0.error : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"error","hash":{},"data":data}) : helper)))
    + "</div>\r\n        <div class=\"form-group\">\r\n          <button id=\"login-btn\" class=\"btn btn-lg btn-success btn-block\" type=\"submit\">ВХОД</button>\r\n        </div>\r\n        <p>Еще не с нами? <a href=\"#register\">Пройди регистрацию</a>, чтобы присоединиться.</p>\r\n      </form>\r\n    </div>\r\n  </div>\r\n</div>\r\n";
},"useData":true});
});
define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['registered'] = template({"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var helper;

  return "<div class=\"ui container middle aligned one column doubling stackable center aligned grid full-height\">\n  <div class=\"column\">\n    <div class=\"ui massive message\">\n      <h1 class=\"ui green header\">Регистрация прошла успешно!</h1>\n      <p>Осталось совсем чуть-чуть, всего лишь активировать вашу учётную запись, и Вы сможете сможете пользоватся фотометром по полной программе.</p>\n      <p>Сейчас наши сутрудники высылают, на Ваш электронный адрес <strong>"
    + container.escapeExpression(((helper = (helper = helpers.email || (depth0 != null ? depth0.email : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0 != null ? depth0 : {},{"name":"email","hash":{},"data":data}) : helper)))
    + "</strong> письмо для активации вашей учётной записи.\n        Следуйте инструкция в письме, что-бы завершить регистрацию.</p>\n    </div>\n  </div>\n</div>\n";
},"useData":true});
});
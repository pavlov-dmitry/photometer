define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['dev_error'] = template({"compiler":[6,">= 2.0.0-beta.1"],"main":function(depth0,helpers,partials,data) {
    var helper;

  return "<div class=\"container\">\n    <div class=\"jumbotron\">\n        <h2>Произошла ошибка взаимодействия с сервером.</h2>\n        <div class=\"panel panel-danger\">\n            <div class=\"panel-heading\">\n                 <h3 class=\"panel-title\">Подробная информация:</h3>\n            </div>\n            <div class=\"panel-body\">"
    + this.escapeExpression(((helper = (helper = helpers.error_msg || (depth0 != null ? depth0.error_msg : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0,{"name":"error_msg","hash":{},"data":data}) : helper)))
    + "</div>\n            <div class=\"panel-footer\"><em>Будьте добры, передайте её разработчикам</em></div>\n        </div>\n    </div>\n</div>";
},"useData":true});
});
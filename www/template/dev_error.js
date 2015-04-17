define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['dev_error'] = template({"compiler":[6,">= 2.0.0-beta.1"],"main":function(depth0,helpers,partials,data) {
    var stack1, alias1=this.lambda, alias2=this.escapeExpression;

  return "<div class=\"container\">\n    <div class=\"jumbotron\">\n        <h2>Ой, что-то пошло не так :(</h2>\n        <div class=\"panel panel-danger\">\n            <div class=\"panel-heading\">\n                 <h3 class=\"panel-title\">Подробная информация:</h3>\n            </div>\n            <table class=\"table table-striped table-first-column-bold\">\n                <tr>\n                    <td>method</td><td>"
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.ajax : depth0)) != null ? stack1.method : stack1), depth0))
    + "</td>\n                </tr>\n                <tr>\n                    <td>url</td><td>"
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.ajax : depth0)) != null ? stack1.url : stack1), depth0))
    + "</td>\n                </tr>\n                <tr>\n                    <td>data</td><td>"
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.ajax : depth0)) != null ? stack1.data : stack1), depth0))
    + "</td>\n                </tr>\n                <tr>\n                    <td>response</td><td>"
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.response : depth0)) != null ? stack1.status : stack1), depth0))
    + " - "
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.response : depth0)) != null ? stack1.statusText : stack1), depth0))
    + "</td>\n                </tr>\n                <tr>\n                    <td>text</td><td>"
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.response : depth0)) != null ? stack1.responseText : stack1), depth0))
    + "</td>\n                </tr>\n            </table>\n        </div>\n    </div>\n</div>\n";
},"useData":true});
});
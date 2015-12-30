define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['dev_error'] = template({"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, alias1=container.lambda, alias2=container.escapeExpression;

  return "<div class=\"ui negative message container\">\n  <h1 class=\"header\">\n    Ой, что-то пошло не так :(\n  </h1>\n  <div class=\"content\">\n    <div class=\"ui hidden divider\"></div>\n    <h4 class=\"ui black header\">\n      <div class=\"content\">\n        Подробная информация:\n      </div>\n    </h4>\n    <table class=\"ui very basic collapsing celled stripped table\">\n      <tr>\n        <td>\n          <h4 class=\"ui header\">\n            <div class=\"content\">\n              method\n            </div>\n            <h4>\n        </td>\n        <td>\n          "
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.ajax : depth0)) != null ? stack1.method : stack1), depth0))
    + "\n        </td>\n      </tr>\n      <tr>\n        <td>\n          <h4 class=\"ui header\">\n            <div class=\"content\">\n              url\n            </div>\n          </h4>\n        </td>\n        <td>\n          "
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.ajax : depth0)) != null ? stack1.url : stack1), depth0))
    + "\n        </td>\n      </tr>\n      <tr>\n        <td>\n          <h4 class=\"ui header\">\n            <div class=\"content\">\n              data\n            </div>\n          </h4>\n        </td>\n        <td>\n          "
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.ajax : depth0)) != null ? stack1.data : stack1), depth0))
    + "\n        </td>\n      </tr>\n      <tr>\n        <td>\n          <h4 class=\"ui header\">\n            <div class=\"content\">\n              response\n            </div>\n          </h4>\n        </td>\n        <td>\n          "
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.response : depth0)) != null ? stack1.status : stack1), depth0))
    + " - "
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.response : depth0)) != null ? stack1.statusText : stack1), depth0))
    + "\n        </td>\n      </tr>\n      <tr>\n        <td>\n          <h4 class=\"ui header\">\n            <div class=\"content\">\n              text\n            </div>\n          </h4>\n        </td>\n        <td>\n          "
    + alias2(alias1(((stack1 = (depth0 != null ? depth0.response : depth0)) != null ? stack1.responseText : stack1), depth0))
    + "\n        </td>\n      </tr>\n    </table>\n  </div>\n</div>\n";
},"useData":true});
});
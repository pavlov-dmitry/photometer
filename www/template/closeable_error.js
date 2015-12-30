define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['closeable_error'] = template({"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var helper;

  return "<div class=\"ui negative message\">\n  <i class=\"close icon\"></i>\n  <div class=\"header\">\n    <div class=\"content\">\n      Ошибка!\n    </div>\n  </div>\n  <div class=\"content\">\n    "
    + container.escapeExpression(((helper = (helper = helpers.text || (depth0 != null ? depth0.text : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0 != null ? depth0 : {},{"name":"text","hash":{},"data":data}) : helper)))
    + "\n  </div>\n</div>\n";
},"useData":true});
});
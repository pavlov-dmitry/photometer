define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['vote_action'] = template({"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var helper;

  return "<div class=\"ui center aligned basic segment\">\n  <h3 class=\"ui header\">\n    <div class=\"content\">\n      <i>"
    + container.escapeExpression(((helper = (helper = helpers.answer || (depth0 != null ? depth0.answer : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0 != null ? depth0 : {},{"name":"answer","hash":{},"data":data}) : helper)))
    + "</i>\n    </div>\n  </h3>\n  <button id=\"yes-btn\" class=\"positive ui button\">Да</button>\n  <button id=\"no-btn\" class=\"negative ui button\">Нет</button>\n</div>\n";
},"useData":true});
});
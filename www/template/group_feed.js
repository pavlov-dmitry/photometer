define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['group_feed'] = template({"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return "<div class=\"ui container\">\n  <h1 class=\"ui block header\">\n    <i class=\"users icon\"></i>\n    <div class=\"content\">\n      <a href=\"#group/info/"
    + alias4(((helper = (helper = helpers.group_id || (depth0 != null ? depth0.group_id : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"group_id","hash":{},"data":data}) : helper)))
    + "\">"
    + alias4(((helper = (helper = helpers.group_name || (depth0 != null ? depth0.group_name : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"group_name","hash":{},"data":data}) : helper)))
    + "</a>\n    </div>\n  </h1>\n  <div id=\"feeds\" class=\"ui large feed\">\n  </div>\n  <div id=\"feeds-loader\" class=\"ui centered inline text loader\">\n    Подгружаем дальше ...\n  </div>\n</div>\n";
},"useData":true});
});
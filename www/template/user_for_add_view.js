define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['user_for_add_view'] = template({"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return "<div class=\"ui action input\">\n  <input type=\"text\" class=\"form-control user-name\" placeholder=\"Имя пользователя\" value=\""
    + alias4(((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"name","hash":{},"data":data}) : helper)))
    + "\" required>\n  <div class=\"ui icon button remove-btn\">\n    <i class=\"minus icon\"></i>\n  </div>\n</div>\n\n<!-- <p> -->\n<!--   <div class=\"input-group\"> -->\n<!--     <input type=\"text\" class=\"form-control user-name\" placeholder=\"Имя пользователя\" value=\""
    + alias4(((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"name","hash":{},"data":data}) : helper)))
    + "\" required> -->\n<!--     <span class=\"input-group-btn\"> -->\n<!--       <button class=\"btn btn-default remove-btn\" type=\"button\"><span class=\"glyphicon glyphicon-trash\"></span></button> -->\n<!--     </span> -->\n<!--   </div> -->\n<!-- </p> -->\n";
},"useData":true});
});
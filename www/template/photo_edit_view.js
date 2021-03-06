define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['photo_edit_view'] = template({"1":function(container,depth0,helpers,partials,data) {
    var stack1;

  return " value=\""
    + container.escapeExpression(container.lambda(((stack1 = (depth0 != null ? depth0.photo : depth0)) != null ? stack1.name : stack1), depth0))
    + "\"";
},"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {};

  return "<div class=\"ui container\">\n  <h2 class=\"ui header\">\n    <div class=\"content\">\n      <i class=\"edit icon\"></i>\n      Редактирование\n    </div>\n  </h2>\n  <form id=\"rename-photo-form\" class=\"ui form\" onsubmit=\"return false;\">\n    <div class=\"fields\">\n      <div class=\"twelve wide icon field\">\n        <div class=\"ui input\">\n          <input type=\"text\" class=\"has-success\" maxlength=\"64\" id=\"new-name-input\" placeholder=\"Имя фотографии\" "
    + ((stack1 = helpers["if"].call(alias1,((stack1 = (depth0 != null ? depth0.photo : depth0)) != null ? stack1.name : stack1),{"name":"if","hash":{},"fn":container.program(1, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + " required>\n        </div>\n      </div>\n      <div class=\"four wide field\">\n        <button class=\"ui primary button\" type=\"submit\">\n          <i class=\"edit icon\"></i>\n          Переименовать\n        </button>\n      </div>\n    </div>\n  </form>\n  <div class=\"ui divider\"></div>\n  <div id=\"img-container\">\n    <img class=\"fit incenter\" id=\"photo\" src=\"/photo/"
    + container.escapeExpression(((helper = (helper = helpers.id || (depth0 != null ? depth0.id : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(alias1,{"name":"id","hash":{},"data":data}) : helper)))
    + ".jpg\"/>\n  </div>\n  <button id=\"crop-btn\" style=\"width: 100%\" class=\"ui bottom attached primary button\" type=\"button\">\n    <i class=\"crop icon\"></i>\n    Изменить миниатюру\n  </button>\n</div>\n";
},"useData":true});
});
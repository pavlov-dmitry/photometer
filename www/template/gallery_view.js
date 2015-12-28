define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['gallery_view'] = template({"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    return "<div class=\"ui basic segment container\">\r\n  <span id=\"upload-btn\" class=\"ui green button fileinput-button\">\r\n    <i class=\"add icon\"></i>\r\n    <span>Добавить</span>\r\n    <input id=\"upload-file\" type=\"file\" name=\"file\" accept=\"image/jpeg,image/png\">\r\n  </span>\r\n\r\n  <div id=\"upload-progress\" class=\"ui indicating progress\">\r\n    <div class=\"bar\"></div>\r\n    <div class=\"label\">Загружено</div>\r\n  </div>\r\n\r\n  <p>\r\n    <div id=\"header-pagination\">\r\n    </div>\r\n  </p>\r\n  <div id=\"preview-list\" class=\"ui stackable cards\">\r\n  </div>\r\n  <br>\r\n  <p>\r\n    <div id=\"footer-pagination\">\r\n    </div>\r\n  </p>\r\n</div>\r\n";
},"useData":true});
});
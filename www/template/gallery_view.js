define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['gallery_view'] = template({"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    return "<div class=\"container\">\r\n  <p>\r\n  <span id=\"upload-btn\" class=\"btn btn-success fileinput-button\">\r\n    <i class=\"glyphicon glyphicon-plus\"></i>\r\n    <span>Добавить</span>\r\n    <input id=\"upload-file\" type=\"file\" name=\"file\" accept=\"image/jpeg,image/png\">\r\n  </span>\r\n  </p>\r\n  <div id=\"upload-progress\" class=\"progress\">\r\n    <div class=\"progress-bar progress-bar-success progress-bar-striped active\"></div>\r\n  </div>\r\n  <div id=\"header-pagination\">\r\n  </div>\r\n  <div id=\"preview-list\" class=\"clearfix\">\r\n  </div>\r\n  <br>\r\n  <div id=\"footer-pagination\">\r\n  </div>\r\n</div>\r\n";
},"useData":true});
});
define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['mailbox_view'] = template({"1":function(container,depth0,helpers,partials,data) {
    return "active";
},"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, alias1=depth0 != null ? depth0 : {};

  return "<div class=\"container page-btns-place\">\n  <div class=\"btn-group btn-group-justified\" role=\"group\">\n    <a href=\"#mailbox/unreaded\" role=\"button\" class=\"btn btn-default "
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.is_unreaded : depth0),{"name":"if","hash":{},"fn":container.program(1, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "\">Непрочитанные</a>\n    <a href=\"#mailbox\" role=\"button\" class=\"btn btn-default "
    + ((stack1 = helpers.unless.call(alias1,(depth0 != null ? depth0.is_unreaded : depth0),{"name":"unless","hash":{},"fn":container.program(1, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "\">Все</a>\n  </div>\n</div>\n<div class=\"container\">\n  <div id=\"header-pagination\">\n  </div>\n  <div id=\"mail-list\">\n  </div>\n  <div id=\"footer-pagination\" style=\"clear:both\">\n  </div>\n</div>\n";
},"useData":true});
});
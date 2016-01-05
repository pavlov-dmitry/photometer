define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['mailbox_view'] = template({"1":function(container,depth0,helpers,partials,data) {
    return " active";
},"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, alias1=depth0 != null ? depth0 : {};

  return "<div class=\"ui container\">\n  <div class=\"ui centered doubling stackable grid basic segment\">\n    <div class=\"three column row\">\n      <div class=\"column\">\n        <div class=\"ui two item menu\">\n          <a class=\"item"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.is_unreaded : depth0),{"name":"if","hash":{},"fn":container.program(1, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "\" href=\"#mailbox/unreaded\">Непрочитанные</a>\n          <a class=\"item"
    + ((stack1 = helpers.unless.call(alias1,(depth0 != null ? depth0.is_unreaded : depth0),{"name":"unless","hash":{},"fn":container.program(1, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "\" href=\"#mailbox\">Все</a>\n        </div>\n      </div>\n    </div>\n  </div>\n  <div id=\"header-pagination\">\n  </div>\n  <div class=\"ui centered doubling stackable grid\">\n    <div class=\"twelve wide column\">\n      <div id=\"mail-list\" class=\"ui relaxed divided items\">\n      </div>\n    </div>\n  </div>\n  <div id=\"footer-pagination\" style=\"clear:both\">\n  </div>\n</div>\n";
},"useData":true});
});
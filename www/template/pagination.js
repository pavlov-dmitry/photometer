define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['pagination'] = template({"1":function(container,depth0,helpers,partials,data) {
    var helper;

  return "  <a class=\"ui button\" href=\""
    + container.escapeExpression(((helper = (helper = helpers.prev || (depth0 != null ? depth0.prev : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0 != null ? depth0 : {},{"name":"prev","hash":{},"data":data}) : helper)))
    + "\" aria-label=\"Предыдущая\">\n    <i class=\"angle double left icon\"></i>\n  </a>\n";
},"3":function(container,depth0,helpers,partials,data) {
    return "  <div class=\"ui button disabled\">\n    <i class=\"angle double left icon\"></i>\n  </div>\n";
},"5":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {};

  return "  <a class=\"ui button "
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.active : depth0),{"name":"if","hash":{},"fn":container.program(6, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.disabled : depth0),{"name":"if","hash":{},"fn":container.program(8, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "\" "
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.link : depth0),{"name":"if","hash":{},"fn":container.program(10, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + ">\n    "
    + container.escapeExpression(((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(alias1,{"name":"name","hash":{},"data":data}) : helper)))
    + "\n  </a>\n";
},"6":function(container,depth0,helpers,partials,data) {
    return " active";
},"8":function(container,depth0,helpers,partials,data) {
    return " disabled";
},"10":function(container,depth0,helpers,partials,data) {
    var helper;

  return "href=\""
    + container.escapeExpression(((helper = (helper = helpers.link || (depth0 != null ? depth0.link : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0 != null ? depth0 : {},{"name":"link","hash":{},"data":data}) : helper)))
    + "\"";
},"12":function(container,depth0,helpers,partials,data) {
    var helper;

  return "  <a class=\"ui button\" href=\""
    + container.escapeExpression(((helper = (helper = helpers.next || (depth0 != null ? depth0.next : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0 != null ? depth0 : {},{"name":"next","hash":{},"data":data}) : helper)))
    + "\" aria-label=\"Слудующая\">\n    <i class=\"angle double right icon\"></i>\n  </a>\n";
},"14":function(container,depth0,helpers,partials,data) {
    return "  <div class=\"ui button disabled\">\n    <i class=\"angle double right icon\"></i>\n  </div>\n";
},"16":function(container,depth0,helpers,partials,data) {
    return "";
},"18":function(container,depth0,helpers,partials,data) {
    return "class=\"disabled\"";
},"20":function(container,depth0,helpers,partials,data) {
    var helper;

  return "href=\""
    + container.escapeExpression(((helper = (helper = helpers.prev || (depth0 != null ? depth0.prev : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0 != null ? depth0 : {},{"name":"prev","hash":{},"data":data}) : helper)))
    + "\"";
},"22":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {};

  return " -->\n<!--     <li "
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.active : depth0),{"name":"if","hash":{},"fn":container.program(23, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.disabled : depth0),{"name":"if","hash":{},"fn":container.program(18, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "> -->\n<!--       <a "
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.link : depth0),{"name":"if","hash":{},"fn":container.program(10, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + " "
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.next : depth0),{"name":"if","hash":{},"fn":container.program(25, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + " "
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.next : depth0),{"name":"if","hash":{},"fn":container.program(27, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "> -->\n<!--         "
    + container.escapeExpression(((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(alias1,{"name":"name","hash":{},"data":data}) : helper)))
    + " -->\n<!--       </a> -->\n<!--     </li> -->\n<!--     ";
},"23":function(container,depth0,helpers,partials,data) {
    return "class=\"active\"";
},"25":function(container,depth0,helpers,partials,data) {
    return "aria-label=\"\"";
},"27":function(container,depth0,helpers,partials,data) {
    return "aria-label=\"Слудующая\"";
},"29":function(container,depth0,helpers,partials,data) {
    var helper;

  return "href=\""
    + container.escapeExpression(((helper = (helper = helpers.next || (depth0 != null ? depth0.next : depth0)) != null ? helper : helpers.helperMissing),(typeof helper === "function" ? helper.call(depth0 != null ? depth0 : {},{"name":"next","hash":{},"data":data}) : helper)))
    + "\"";
},"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, alias1=depth0 != null ? depth0 : {};

  return "<div class=\"ui small basic icon buttons\">\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.prev : depth0),{"name":"if","hash":{},"fn":container.program(1, data, 0),"inverse":container.program(3, data, 0),"data":data})) != null ? stack1 : "")
    + "\n"
    + ((stack1 = helpers.each.call(alias1,(depth0 != null ? depth0.pages : depth0),{"name":"each","hash":{},"fn":container.program(5, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.next : depth0),{"name":"if","hash":{},"fn":container.program(12, data, 0),"inverse":container.program(14, data, 0),"data":data})) != null ? stack1 : "")
    + "</div>\n\n<!-- <nav> -->\n<!--   <ul class=\"pagination\"> -->\n<!--     <li "
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.prev : depth0),{"name":"if","hash":{},"fn":container.program(16, data, 0),"inverse":container.program(18, data, 0),"data":data})) != null ? stack1 : "")
    + "> -->\n<!--       <a "
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.prev : depth0),{"name":"if","hash":{},"fn":container.program(20, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + " aria-label=\"Предыдущая\"> -->\n<!--         <span aria-hidden=\"true\">&laquo;</span> -->\n<!--       </a> -->\n<!--     </li> -->\n<!--     "
    + ((stack1 = helpers.each.call(alias1,(depth0 != null ? depth0.pages : depth0),{"name":"each","hash":{},"fn":container.program(22, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + " -->\n<!--     <li "
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.next : depth0),{"name":"if","hash":{},"fn":container.program(16, data, 0),"inverse":container.program(18, data, 0),"data":data})) != null ? stack1 : "")
    + "> -->\n<!--       <a "
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.next : depth0),{"name":"if","hash":{},"fn":container.program(29, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + " aria-label=\"Слудующая\"> -->\n<!--         <span aria-hidden=\"true\">&raquo;</span> -->\n<!--       </a> -->\n<!--     </li> -->\n<!--   </ul> -->\n<!-- </nav> -->\n";
},"useData":true});
});
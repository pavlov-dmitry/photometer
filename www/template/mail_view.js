define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['mail_view'] = template({"compiler":[6,">= 2.0.0-beta.1"],"main":function(depth0,helpers,partials,data) {
    var stack1, helper, alias1=helpers.helperMissing, alias2="function", alias3=this.escapeExpression;

  return "<div class=\"fly\">\n  <div class=\"blue-glass mail-header\">\n    <h4><strong>"
    + alias3(((helper = (helper = helpers.sender_name || (depth0 != null ? depth0.sender_name : depth0)) != null ? helper : alias1),(typeof helper === alias2 ? helper.call(depth0,{"name":"sender_name","hash":{},"data":data}) : helper)))
    + "</strong>: "
    + alias3(((helper = (helper = helpers.subject || (depth0 != null ? depth0.subject : depth0)) != null ? helper : alias1),(typeof helper === alias2 ? helper.call(depth0,{"name":"subject","hash":{},"data":data}) : helper)))
    + "</h4>\n  </div>\n  <div class=\"mail-body\">\n    <p>"
    + ((stack1 = ((helper = (helper = helpers.body || (depth0 != null ? depth0.body : depth0)) != null ? helper : alias1),(typeof helper === alias2 ? helper.call(depth0,{"name":"body","hash":{},"data":data}) : helper))) != null ? stack1 : "")
    + "</p>\n  </div>\n</div>\n";
},"useData":true});
});
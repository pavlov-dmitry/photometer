define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['mail_view'] = template({"1":function(container,depth0,helpers,partials,data) {
    return "      <button type=\"button\" class=\"btn btn-success pull-right\">Прочитал</button>\n";
},"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return "<div class=\"panel panel-default\">\n  <div class=\"panel-heading clearfix\">\n"
    + ((stack1 = helpers.unless.call(alias1,(depth0 != null ? depth0.readed : depth0),{"name":"unless","hash":{},"fn":container.program(1, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "      <h4>\n        <strong>"
    + alias4(((helper = (helper = helpers.sender_name || (depth0 != null ? depth0.sender_name : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"sender_name","hash":{},"data":data}) : helper)))
    + "</strong>: "
    + alias4(((helper = (helper = helpers.subject || (depth0 != null ? depth0.subject : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"subject","hash":{},"data":data}) : helper)))
    + "\n      </h4>\n  </div>\n  <div class=\"panel-body\">\n    <p>"
    + ((stack1 = (helpers.markdown || (depth0 && depth0.markdown) || alias2).call(alias1,(depth0 != null ? depth0.body : depth0),{"name":"markdown","hash":{},"data":data})) != null ? stack1 : "")
    + "</p>\n  </div>\n</div>\n";
},"useData":true});
});
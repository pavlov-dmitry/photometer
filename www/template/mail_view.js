define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['mail_view'] = template({"1":function(container,depth0,helpers,partials,data) {
    return "      <button class=\"ui topright icon basic tiny button readed-btn\" data-tooltip=\"Пометить как прочитанное.\" data-position=\"left center\" data-variation=\"inverted\">\n        <i class=\"close icon\"></i>\n      </button>\n";
},"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.escapeExpression;

  return "<div id=\""
    + alias4(((helper = (helper = helpers.id || (depth0 != null ? depth0.id : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"id","hash":{},"data":data}) : helper)))
    + "\" class=\"relative item\">\n  <div class=\"content\">\n    <div class=\"header\">\n      <strong>"
    + alias4(((helper = (helper = helpers.sender_name || (depth0 != null ? depth0.sender_name : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"sender_name","hash":{},"data":data}) : helper)))
    + ":</strong> "
    + alias4(((helper = (helper = helpers.subject || (depth0 != null ? depth0.subject : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"subject","hash":{},"data":data}) : helper)))
    + "\n"
    + ((stack1 = helpers.unless.call(alias1,(depth0 != null ? depth0.readed : depth0),{"name":"unless","hash":{},"fn":container.program(1, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "    </div>\n    <div class=\"meta\">Создано "
    + alias4((helpers.duration_from_now || (depth0 && depth0.duration_from_now) || alias2).call(alias1,(depth0 != null ? depth0.creation_time : depth0),{"name":"duration_from_now","hash":{},"data":data}))
    + "</div>\n    <div class=\"description\">\n      <p>\n      "
    + ((stack1 = (helpers.markdown || (depth0 && depth0.markdown) || alias2).call(alias1,(depth0 != null ? depth0.body : depth0),{"name":"markdown","hash":{},"data":data})) != null ? stack1 : "")
    + "\n      </p>\n    </div>\n  </div>\n</div>\n";
},"useData":true});
});
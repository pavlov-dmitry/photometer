define(['handlebars.runtime'], function(Handlebars) {
  Handlebars = Handlebars["default"];  var template = Handlebars.template, templates = Handlebars.templates = Handlebars.templates || {};
return templates['event_info'] = template({"1":function(container,depth0,helpers,partials,data) {
    var stack1;

  return "        <div class=\"ui blue label\">\n          "
    + container.escapeExpression(container.lambda(((stack1 = (depth0 != null ? depth0.creator : depth0)) != null ? stack1.name : stack1), depth0))
    + "\n        </div>\n";
},"compiler":[7,">= 4.0.0"],"main":function(container,depth0,helpers,partials,data) {
    var stack1, helper, alias1=depth0 != null ? depth0 : {}, alias2=helpers.helperMissing, alias3="function", alias4=container.lambda, alias5=container.escapeExpression;

  return "<div class=\"ui basic padded segment container\">\n  <h2 class=\"ui dividing header\">\n    <div class=\"content\">\n      "
    + ((stack1 = ((helper = (helper = helpers.name || (depth0 != null ? depth0.name : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"name","hash":{},"data":data}) : helper))) != null ? stack1 : "")
    + "\n      <div class=\"ui small "
    + alias5(alias4(((stack1 = (depth0 != null ? depth0.state : depth0)) != null ? stack1.color : stack1), depth0))
    + " label\">\n        "
    + alias5(alias4(((stack1 = (depth0 != null ? depth0.state : depth0)) != null ? stack1.text : stack1), depth0))
    + "\n      </div>\n    </div>\n  </h2>\n  <div class=\"ui right aligned basic segment nopadmar\">\n    <small>\n    <i>\n      <strong>\n        Создано\n"
    + ((stack1 = helpers["if"].call(alias1,(depth0 != null ? depth0.creator : depth0),{"name":"if","hash":{},"fn":container.program(1, data, 0),"inverse":container.noop,"data":data})) != null ? stack1 : "")
    + "        "
    + alias5((helpers.duration_from_now || (depth0 && depth0.duration_from_now) || alias2).call(alias1,(depth0 != null ? depth0.starting_time : depth0),{"name":"duration_from_now","hash":{},"data":data}))
    + "\n      </strong>\n      ("
    + alias5((helpers.time || (depth0 && depth0.time) || alias2).call(alias1,(depth0 != null ? depth0.starting_time : depth0),{"name":"time","hash":{},"data":data}))
    + ")\n      <strong>\n        Окончание\n        "
    + alias5((helpers.duration_from_now || (depth0 && depth0.duration_from_now) || alias2).call(alias1,(depth0 != null ? depth0.ending_time : depth0),{"name":"duration_from_now","hash":{},"data":data}))
    + "\n      </strong>\n      ("
    + alias5((helpers.time || (depth0 && depth0.time) || alias2).call(alias1,(depth0 != null ? depth0.ending_time : depth0),{"name":"time","hash":{},"data":data}))
    + ")\n    </i>\n    </small>\n  </div>\n  <div id=\"description\">\n    "
    + ((stack1 = ((helper = (helper = helpers.description || (depth0 != null ? depth0.description : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"description","hash":{},"data":data}) : helper))) != null ? stack1 : "")
    + "\n  </div>\n</div>\n<div id=\"action\">\n  "
    + ((stack1 = ((helper = (helper = helpers.action || (depth0 != null ? depth0.action : depth0)) != null ? helper : alias2),(typeof helper === alias3 ? helper.call(alias1,{"name":"action","hash":{},"data":data}) : helper))) != null ? stack1 : "")
    + "\n</div>\n";
},"useData":true});
});
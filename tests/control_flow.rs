use std::collections::HashMap;

use kitamura::render_template;

#[test]
#[should_panic]
fn control_flow_does_not_exist() {
    let html = "<html><ul>{#forrgfeqwv fruit in fruits#}<ul><li>${fruit.name}</li><li>${fruit.colour}</li><li>${fruit.weight}</li></ul>{#endfor#}</ul></html>";
    let params: HashMap<String, serde_json::Value> = HashMap::new();
    let _rendered_html = render_template(html.to_string(), params);
}

#[test]
#[should_panic]
fn control_flow_syntax_error_extra_brace() {
    let html = "<html><ul>{{#for fruit in fruits#}<ul><li>${fruit.name}</li><li>${fruit.colour}</li><li>${fruit.weight}</li></ul>{#endfor#}</ul></html>";
    let params: HashMap<String, serde_json::Value> = HashMap::new();
    let _rendered_html = render_template(html.to_string(), params);
}

#[test]
#[should_panic]
fn control_flow_syntax_error_extra_pound() {
    let html = "<html><ul>{##for fruit in fruits#}<ul><li>${fruit.name}</li><li>${fruit.colour}</li><li>${fruit.weight}</li></ul>{#endfor#}</ul></html>";
    let params: HashMap<String, serde_json::Value> = HashMap::new();
    let _rendered_html = render_template(html.to_string(), params);
}

#[test]
#[should_panic]
fn control_flow_missing_end() {
    let html = "<html>{#for fruit in fruits#}</html>";
    let params: HashMap<String, serde_json::Value> = HashMap::new();
    let _rendered_html = render_template(html.to_string(), params);
}

#[test]
#[should_panic]
fn nested_control_flow_missing_end() {
    let html = "<html>{#for fruit in fruits#}{#for thing in something#}{#endfor#}</html>";
    let params: HashMap<String, serde_json::Value> = HashMap::new();
    let _rendered_html = render_template(html.to_string(), params);
}

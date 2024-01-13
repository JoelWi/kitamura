use std::collections::HashMap;

use kitamura::render_template;

#[test]
fn control_flow_does_not_exist() {
    let html = "<html><ul>{#forrgfeqwv fruit of fruits#}<ul><li>${fruit.name}</li><li>${fruit.colour}</li><li>${fruit.weight}</li></ul>{#endfor#}</ul></html>";
    let params = HashMap::new();
    let rendered_html = render_template(html.to_string(), params);
    assert!(rendered_html.is_err())
}

#[test]
fn control_flow_syntax_error_extra_brace() {
    let html = "<html><ul>{{#for fruit of fruits#}<ul><li>${fruit.name}</li><li>${fruit.colour}</li><li>${fruit.weight}</li></ul>{#endfor#}</ul></html>";
    let params = HashMap::new();
    let rendered_html = render_template(html.to_string(), params);
    assert!(rendered_html.is_err())
}

#[test]
fn control_flow_syntax_error_extra_pound() {
    let html = "<html><ul>{##for fruit of fruits#}<ul><li>${fruit.name}</li><li>${fruit.colour}</li><li>${fruit.weight}</li></ul>{#endfor#}</ul></html>";
    let params = HashMap::new();
    let rendered_html = render_template(html.to_string(), params);
    assert!(rendered_html.is_err())
}

#[test]
fn control_flow_missing_end() {
    let html = "<html>{#for fruit of fruits#}</html>";
    let params = HashMap::new();
    let rendered_html = render_template(html.to_string(), params);
    assert!(rendered_html.is_err())
}

#[test]
fn nested_control_flow_missing_end() {
    let html = "<html>{#for fruit of fruits#}{#for thing of something#}{#endfor#}</html>";
    let params = HashMap::new();
    let rendered_html = render_template(html.to_string(), params);
    assert!(rendered_html.is_err())
}

use std::collections::HashMap;

use kitamura::render_template;
use serde_json::json;

#[test]
fn variable_renders_successfully() {
    let html = "<html>${first_name}</html>";
    let mut params = HashMap::new();
    params.insert("first_name".to_string(), json!("Joel"));
    let rendered_html = render_template(html.to_string(), params);
    assert_eq!(rendered_html, "<html>Joel</html>");
}

#[test]
#[should_panic]
fn variable_key_data_missing() {
    let html = "<html>${first_name}</html>";
    let params = HashMap::new();
    let _rendered_html = render_template(html.to_string(), params);
}

#[test]
#[should_panic]
fn variable_extra_open_brace() {
    let html = "<html>${{first_name}</html>";
    let params = HashMap::new();
    let _rendered_html = render_template(html.to_string(), params);
}

#[test]
#[should_panic]
fn variable_extra_closing_brace_shortens_name() {
    let html = "<html>${first_na}me}</html>";
    let params = HashMap::new();
    let _rendered_html = render_template(html.to_string(), params);
}

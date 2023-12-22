use std::collections::HashMap;

#[test]
fn condition_present_with_no_valid_param_provided() {
    let html = "Hello{#if first_name?? && first_name?not_empty#}${first_name}{#endif#}!".to_owned();
    let mut params = HashMap::new();

    params.insert("first_name".to_owned(), serde_json::json!({}));

    let expeccted_output = "Hello!".to_owned();
    let rendered_html = kitamura::render_template(html, params).unwrap();
    assert_eq!(rendered_html, expeccted_output);
}

#[test]
fn nested_condition_present_with_no_valid_param_provided() {
    let html = "Hello{#if first_name?? && first_name?not_empty#}${first_name}{#if last_name?? && last_name?not_empty#}${last_name}{#endif#}{#endif#}!".to_owned();
    let mut params = HashMap::new();

    params.insert("first_name".to_owned(), serde_json::json!({}));

    let expeccted_output = "Hello!".to_owned();
    let rendered_html = kitamura::render_template(html, params).unwrap();
    assert_eq!(rendered_html, expeccted_output);
}

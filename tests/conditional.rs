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

#[test]
fn exists() {
    let html =
        "Hello{#if first_name?exists && first_name?not_empty#} ${first_name}{#endif#}!".to_owned();
    let params = HashMap::from([("first_name".to_owned(), serde_json::json!("Joel"))]);

    let expected_output = "Hello Joel!";
    let rendered_html = kitamura::render_template(html.clone(), params).unwrap();
    assert_eq!(rendered_html, expected_output);
}

#[test]
fn not_empty() {
    let html =
        "Hello{#if first_name?exists && first_name?not_empty#} ${first_name}{#endif#}!".to_owned();
    let params = HashMap::from([("first_name".to_owned(), serde_json::json!(""))]);

    let expected_output = "Hello!";
    let rendered_html = kitamura::render_template(html.clone(), params).unwrap();
    assert_eq!(rendered_html, expected_output);
}

#[test]
fn not_empty_but_param_is_missing() {
    let html =
        "Hello{#if first_name?exists && first_name?not_empty#} ${first_name}{#endif#}!".to_owned();
    let params = HashMap::new();

    let expected_output = "Hello!";
    let rendered_html = kitamura::render_template(html.clone(), params).unwrap();
    assert_eq!(rendered_html, expected_output);
}

#[test]
fn combination_of_present_missing_not_empty_empty() {
    let html = "Hello{#if first_name?exists && first_name?not_empty#} ${first_name}{#endif#}!
{#if this_doesnt_exist?exists#}
  <p>This won't appear in render ${this_doesnt_exist}</p>
{#endif#}
{#if something?exists && something?not_empty#}
 <p>${something}</p>
{#endif#}"
        .to_owned();
    let params = HashMap::from([
        ("first_name".to_owned(), serde_json::json!("Joel")),
        ("something".to_owned(), serde_json::json!("")),
    ]);

    let expected_output = "Hello Joel!".to_owned();
    let rendered_html = kitamura::render_template(html.clone(), params)
        .unwrap()
        .replace('\n', "");
    assert_eq!(rendered_html, expected_output);
}

#[test]
fn invalid_data_inside_condition() {
    let html =
        "Hello{#if first_name?exists && first_name?not_empty#} ${this_isnt_provided}{#endif#}!"
            .to_owned();
    let params = HashMap::from([("first_name".to_owned(), serde_json::json!("Joel"))]);

    let rendered_html = kitamura::render_template(html.clone(), params);
    assert!(rendered_html.is_err());
}

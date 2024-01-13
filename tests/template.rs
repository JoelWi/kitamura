use std::collections::HashMap;

use kitamura::render_template;
use serde_json::json;

#[test]
fn new_line_char_appended_successfully() {
    let html = "<html>
    ${first_name}

</html>";
    let expected_rendered_html = "<html>
    Joel

</html>";
    let mut params = HashMap::new();
    params.insert("first_name".to_string(), json!("Joel"));
    let rendered_html = render_template(html.to_string(), params);
    assert_eq!(rendered_html.unwrap(), expected_rendered_html);
}

#[test]
fn data_mapping_key_is_also_object_in_data() {
    let html = "<html>{#for person of persons#}${person.first_name}{#endfor#}</html>";
    let expected_rendered_html = "<html>Joel</html>";
    let mut params = HashMap::new();
    params.insert(
        "persons".to_string(),
        json!({"persons":[{"first_name": "Joel"}]}),
    );
    let rendered_html = render_template(html.to_string(), params);
    assert_eq!(rendered_html.unwrap(), expected_rendered_html);
}

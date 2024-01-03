use std::collections::HashMap;

use kitamura::render_template;
use serde_json::json;

#[test]
fn variable_renders_successfully() {
    let html = "<html> order reference #: ${first_name}</html>";
    let mut params = HashMap::new();
    params.insert("first_name".to_string(), json!("Joel"));
    let rendered_html = render_template(html.to_string(), params);
    assert_eq!(
        rendered_html.unwrap(),
        "<html> order reference #: Joel</html>"
    );
}

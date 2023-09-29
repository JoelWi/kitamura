use kitamura::render_template;
use serde_json::json;
use std::collections::HashMap;

#[test]
fn variable_formatting_incorrect() {
    let html = "<html>${{first_name}</html>";
    let params = HashMap::new();
    let rendered_html = render_template(html.to_string(), params);
    assert!(rendered_html.is_err())
}

#[test]
fn variable_extra_open_brace() {
    let html = "<html>${{first_name}</html>";
    let params = HashMap::new();
    let rendered_html = render_template(html.to_string(), params);
    assert!(rendered_html.is_err())
}

#[test]
fn nested_loops_correctly_get_children_nodes() {
    let html = "<html>
    <ul>
      {#for continent in continents#}
        <li>${continent.name}</li>
        <ul>
          {#for country in countries#}
            <li>${country.name}</li>
          {#endfor#}
        </ul>
      {#endfor#}
    </ul>
  </html>";
    let expected_rendered_html = "<html>
    <ul>
        <li>Oceania</li>
        <ul>
            <li>Australia</li>
            <li>New Zealand</li>
        </ul>
    </ul>
  </html>";
    let mut params = HashMap::new();
    params.insert(
        "continents".to_string(),
        json!([
          {
            "countries": [
              {
                "name": "Australia"
              },
              {
                "name": "New Zealand"
              }
            ],
            "name": "Oceania"
          }
        ]
        ),
    );
    let rendered_html = render_template(html.to_string(), params).unwrap();
    assert_eq!(rendered_html, expected_rendered_html);
}

#[test]
fn whitespace_after_control_flow_statement_removed() {
    let html = "<html>
    <ul>
    {#for person in persons#}
        <li>${person.first_name}</li>
    {#endfor#}
    </ul>
</html>";
    let expected_rendered_html = "<html>
    <ul>
        <li>Joel</li>
    </ul>
</html>";
    let mut params = HashMap::new();
    params.insert(
        "persons".to_string(),
        json!({"persons":[{"first_name": "Joel"}]}),
    );
    let rendered_html = render_template(html.to_string(), params);
    assert_eq!(rendered_html.unwrap(), expected_rendered_html);
}

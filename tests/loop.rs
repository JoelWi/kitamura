use std::collections::HashMap;

use kitamura::render_template;
use serde_json::json;

#[test]
fn loop_data_renders_successfully() {
    let html = "<html><ul>{#for fruit in fruits#}<ul><li>${fruit.name}</li><li>${fruit.colour}</li><li>${fruit.weight}</li></ul>{#endfor#}</ul></html>";
    let expected_rendered_html = "<html>
    <ul>
        <ul>
        <li>Lemon</li>
        <li>Yellow</li>
        <li>150g</li>
        </ul>
        <ul>
        <li>shiikuwasha</li>
        <li>Green</li>
        <li>80g</li>
        </ul>
        <ul>
        <li>Lychee</li>
        <li>Red</li>
        <li>50g</li>
        </ul>
    </ul>
</html>"
        .replace([' ', '\n'], "");
    let mut params = HashMap::new();
    params.insert(
        "fruits".to_string(),
        json!([{"name": "Lemon", "colour": "Yellow", "weight": "150g"},
    {"name": "shiikuwasha", "colour": "Green", "weight": "80g"},
    {"name": "Lychee", "colour": "Red", "weight": "50g"}]),
    );
    let rendered_html = render_template(html.to_string(), params);
    assert_eq!(rendered_html.unwrap(), expected_rendered_html);
}

#[test]
fn loop_data_missing() {
    let html = "<html><ul>{#for fruit in fruits#}<ul><li>${fruit.name}</li><li>${fruit.colour}</li><li>${fruit.weight}</li></ul>{#endfor#}</ul></html>";
    let params = HashMap::new();
    let rendered_html = render_template(html.to_string(), params);
    assert!(rendered_html.is_err())
}

#[test]
fn loop_variable_not_in_scope() {
    let html = "<html><ul>{#for fruitt in fruits#}<ul><li>${fruit.name}</li><li>${fruit.colour}</li><li>${fruit.weight}</li></ul>{#endfor#}</ul></html>";
    let mut params = HashMap::new();
    params.insert(
        "fruits".to_string(),
        json!([{"name": "Lemon", "colour": "Yellow", "weight": "150g"},
    {"name": "shiikuwasha", "colour": "Green", "weight": "80g"},
    {"name": "Lychee", "colour": "Red", "weight": "50g"}]),
    );
    let rendered_html = render_template(html.to_string(), params);
    assert!(rendered_html.is_err())
}

#[test]
fn loop_variable_property_exists() {
    let html = "<html>{#for person in persons#}${person.first_name}{#endfor#}</html>";
    let mut params = HashMap::new();
    params.insert(
        "persons".to_string(),
        json!([{"first_name": "Joel"}, {"first_name": "Joel"}]),
    );
    let rendered_html = render_template(html.to_string(), params);
    assert_eq!(rendered_html.unwrap(), "<html>JoelJoel</html>");
}

#[test]
fn loop_variable_property_missing() {
    let html = "<html>{#for person in persons#}${person.first_name}{#endfor#}</html>";
    let mut params = HashMap::new();
    params.insert("persons".to_string(), json!([{"name": "Joel"}]));
    let rendered_html = render_template(html.to_string(), params);
    assert!(rendered_html.is_err())
}

#[test]
fn loop_can_be_in_or_of() {
    let html = "<html>{#for person of persons#}${person.first_name}{#endfor#}</html>";
    let mut params = HashMap::new();
    params.insert(
        "persons".to_string(),
        json!([{"first_name": "Joel"}, {"first_name": "Joel"}]),
    );
    let rendered_html = render_template(html.to_string(), params);
    assert_eq!(rendered_html.unwrap(), "<html>JoelJoel</html>");
}

#[test]
fn loop_data_not_an_object() {
    let html = "<html>{#for person in persons#}${person}{#endfor#}</html>";
    let mut params = HashMap::new();
    params.insert("persons".to_string(), json!(["Joel"]));
    let rendered_html = render_template(html.to_string(), params);
    assert!(rendered_html.is_err());
}

use std::collections::HashMap;

use kitamura::render_template;
use serde_json::json;

#[test]
fn loop_data_renders_successfully() {
    let html = "<html><ul>{#for fruit of fruits#}<ul><li>${fruit.name}</li><li>${fruit.colour}</li><li>${fruit.weight}</li></ul>{#endfor#}</ul></html>";
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
    let html = "<html><ul>{#for fruit of fruits#}<ul><li>${fruit.name}</li><li>${fruit.colour}</li><li>${fruit.weight}</li></ul>{#endfor#}</ul></html>";
    let params = HashMap::new();
    let rendered_html = render_template(html.to_string(), params);
    assert!(rendered_html.is_err())
}

#[test]
fn loop_variable_not_in_scope() {
    let html = "<html><ul>{#for fruitt of fruits#}<ul><li>${fruit.name}</li><li>${fruit.colour}</li><li>${fruit.weight}</li></ul>{#endfor#}</ul></html>";
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
fn loop_variable_property_missing() {
    let html = "<html>{#for person of persons#}${person.first_name}{#endfor#}</html>";
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
    let html = "<html>{#for person of persons#}${person}{#endfor#}</html>";
    let mut params = HashMap::new();
    params.insert("persons".to_string(), json!(["Joel"]));
    let rendered_html = render_template(html.to_string(), params);
    assert!(rendered_html.is_err());
}

#[test]
fn nested_loops_referencing_the_same_dataset_holds_same_references() {
    let html = "<html>
{#for continent of continents#}
    ${continent.name}
    {#for country of continent.countries#}
        ${country.name}
        {#for city of country.cities#}
            ${city.name}
            {#for continent of continents#}
                ${continent.name}
                {#for country of continent.countries#}
                    ${country.name}
                    {#for city of country.cities#}
                        ${city.name}
                    {#endfor#}
                {#endfor#}
            {#endfor#}
        {#endfor#}
    {#endfor#}
{#endfor#}
</html>";
    let mut params = HashMap::new();
    params.insert(
        "continents".to_string(),
        json!({
            "continents": [
                {
                    "name": "Oceania",
                    "countries": [
                        {
                            "name": "Australia",
                            "cities": [
                                {
                                    "name": "Brisbane",
                                    "time": "9:26PM",
                                    "tempurature": "14C"
                                },
                                {
                                    "name": "Melbourne",
                                    "time": "9:26PM",
                                    "tempurature": "14C"
                                },
                                {
                                    "name": "Adelaide",
                                    "time": "8:56PM",
                                    "tempurature": "15C"
                                }
                            ]
                        },
                        {
                            "name": "New Zealand",
                            "cities": [
                                {
                                    "name": "Wellington",
                                    "time": "11:26PM",
                                    "tempurature": "12C"
                                }
                            ]
                        }
                    ]
                },
                {
                    "name": "Europe",
                    "countries": [
                        {
                            "name": "England",
                            "cities": [
                                {
                                    "name": "Manchester",
                                    "time": "12:26PM",
                                    "tempurature": "16C"
                                },
                                {
                                    "name": "London",
                                    "time": "12:26PM",
                                    "tempurature": "23C"
                                }
                            ]
                        }
                    ]
                }
            ]
        }),
    );

    let expected_output = "<html>
    Oceania
        Australia
            Brisbane
                Oceania
                    Australia
                        Brisbane
                        Melbourne
                        Adelaide
                    New Zealand
                        Wellington
                Europe
                    England
                        Manchester
                        London
            Melbourne
                Oceania
                    Australia
                        Brisbane
                        Melbourne
                        Adelaide
                    New Zealand
                        Wellington
                Europe
                    England
                        Manchester
                        London
            Adelaide
                Oceania
                    Australia
                        Brisbane
                        Melbourne
                        Adelaide
                    New Zealand
                        Wellington
                Europe
                    England
                        Manchester
                        London
        New Zealand
            Wellington
                Oceania
                    Australia
                        Brisbane
                        Melbourne
                        Adelaide
                    New Zealand
                        Wellington
                Europe
                    England
                        Manchester
                        London
    Europe
        England
            Manchester
                Oceania
                    Australia
                        Brisbane
                        Melbourne
                        Adelaide
                    New Zealand
                        Wellington
                Europe
                    England
                        Manchester
                        London
            London
                Oceania
                    Australia
                        Brisbane
                        Melbourne
                        Adelaide
                    New Zealand
                        Wellington
                Europe
                    England
                        Manchester
                        London
</html>";
    let rendered_html = render_template(html.to_string(), params);
    assert_eq!(rendered_html.unwrap(), expected_output);
}

use std::collections::HashMap;

use crate::{
    ast::{construct_ast, ASTNode, ASTNodeIdentifier, AST},
    token::{generate_tokens, parse_tokens},
};

fn validate_iterator(node: &ASTNode, node_iterator_name: &String, open_loop_stack: &Vec<String>) {
    if !open_loop_stack.contains(&node_iterator_name) {
        println!("{:#?}", node);
        panic!(
            "\nError with variable: {}\n{}{}\nDoes not exist: {}\n",
            node.value,
            " ".to_string().repeat(23),
            "^".to_string().repeat(node_iterator_name.len()),
            node_iterator_name
        );
    }
}

fn validate_property(
    node: &ASTNode,
    node_iterator_name: &String,
    node_property_name: &str,
    params: &HashMap<String, serde_json::Value>,
) {
    if params.get(node_property_name).is_none() {
        println!("{:#?}", node);
        panic!(
            "\nError with variable: {}\n{}{}\n'{}' is not a property of '{}'\n",
            node.value,
            " ".to_string().repeat(24 + node_iterator_name.len()),
            "^".to_string().repeat(node_property_name.len()),
            node_property_name,
            node_iterator_name
        );
    }
}

fn validate_loop_data(node: &ASTNode, data: Option<&serde_json::Value>) -> serde_json::Value {
    if data.is_some() {
        data.unwrap().to_owned()
    } else {
        println!("{:#?}", node);
        let construct_token = node.tokens.get(2).unwrap();
        let constructor = construct_token.value.split(" ").nth(3).unwrap();
        panic!(
            "\nData is missing from parameter data mapping:\n{} at line {}:{}\n{}{}\n",
            node.value,
            construct_token.line_start,
            construct_token.pos_start,
            " ".to_string().repeat(17),
            "^".to_string().repeat(constructor.len()),
        )
    }
}

fn generate_variable_data(
    variable_key: &str,
    params: &HashMap<String, serde_json::Value>,
) -> String {
    let variable = params.get(variable_key);
    match variable {
        Some(e) => serde_json::to_string(e).unwrap().replace("\"", ""),
        None => panic!(
            "\n${{{}}} is missing from parameter data mapping.\n",
            variable_key
        ),
    }
}

pub fn generate_template(
    ast: AST,
    params: HashMap<String, serde_json::Value>,
    loop_stack: Vec<String>,
) -> String {
    let mut html = String::new();
    let mut open_loop_stack: Vec<String> = loop_stack;

    for node in ast.nodes {
        if node.identifier == ASTNodeIdentifier::NewLine {
            html.push_str(&node.value)
        } else if node.identifier == ASTNodeIdentifier::Loop {
            let variable_iterator_name = node.value.split_whitespace().nth(1).unwrap().to_string();
            open_loop_stack.push(variable_iterator_name);
            let list_iterator_name = node
                .value
                .replace("#}", "")
                .split_whitespace()
                .nth(3)
                .unwrap()
                .to_string();
            let data_retrieval = params.get(&*list_iterator_name);
            let list_data = validate_loop_data(&node, data_retrieval);

            let list_data_with_key_name = &list_data[list_iterator_name].as_array();

            // Depending on the root mapping, this needs to be handled
            let loop_over = if list_data_with_key_name.is_some() {
                list_data_with_key_name.unwrap()
            } else {
                list_data.as_array().unwrap()
            };
            for item in loop_over {
                let value_to_string = item.as_object().unwrap();
                let mut new_map = HashMap::new();

                for (k, v) in value_to_string.iter() {
                    new_map.insert(k.clone(), v.clone());
                }

                let generated_html_from_loop_children = generate_template(
                    node.children.clone().unwrap(),
                    new_map.clone(),
                    open_loop_stack.clone(),
                );

                html.push_str(&generated_html_from_loop_children);
            }
            open_loop_stack.pop();
        } else if node.identifier == ASTNodeIdentifier::Variable {
            let node_value_cleaned = node.value.replace("${", "").replace("}", "");
            if node.value.contains(".") {
                let node_iterator_name = node_value_cleaned.split(".").nth(0).unwrap().to_string();
                let node_property_name = node_value_cleaned.split(".").last().unwrap();

                validate_iterator(&node, &node_iterator_name, &open_loop_stack);
                validate_property(&node, &node_iterator_name, &node_property_name, &params);

                let variable = generate_variable_data(&node_property_name, &params);
                html.push_str(&variable);
            } else {
                let variable = generate_variable_data(&node_value_cleaned, &params);
                html.push_str(&variable);
            }
        } else if node.identifier != ASTNodeIdentifier::LoopEnd {
            html.push_str(&node.value);
        }
    }

    html
}

pub fn render_template(
    template_html: String,
    parameters: HashMap<String, serde_json::Value>,
) -> String {
    let tokens = generate_tokens(template_html);
    let parsed_tokens = parse_tokens(tokens);
    let ast = construct_ast(parsed_tokens);
    println!("{:#?}", ast);
    println!("PARAMETERS: {:#?}", parameters);
    let loop_stack: Vec<String> = vec![];
    let rendered_html = generate_template(ast, parameters, loop_stack);
    rendered_html
}

use std::collections::HashMap;

use crate::{
    ast::{construct_ast, ASTNode, ASTNodeIdentifier, Ast},
    token::{generate_tokens, parse_tokens},
};

fn validate_iterator(
    node: &ASTNode,
    node_iterator_name: &String,
    open_loop_stack: &[String],
) -> Result<(), String> {
    if !open_loop_stack.contains(node_iterator_name) {
        return Err(format!(
            "\nError with variable: {}\n{}{}\nDoes not exist: {}\n",
            node.value,
            " ".to_string().repeat(23),
            "^".to_string().repeat(node_iterator_name.len()),
            node_iterator_name
        ));
    }
    Ok(())
}

fn validate_property(
    node: &ASTNode,
    node_iterator_name: &String,
    node_property_name: &str,
    params: &HashMap<String, serde_json::Value>,
    parent_params: &HashMap<String, serde_json::Value>,
) -> Result<(), String> {
    let x = parent_params.get(node_iterator_name).unwrap();
    if params.get(node_property_name).is_none() && x.get(node_property_name).is_none() {
        return Err(format!(
            "\nError with variable: {}\n{}{}\n'{}' is not a property of '{}'\n",
            node.value,
            " ".to_string().repeat(24 + node_iterator_name.len()),
            "^".to_string().repeat(node_property_name.len()),
            node_property_name,
            node_iterator_name
        ));
    }
    Ok(())
}

fn validate_loop_data(
    node: &ASTNode,
    data: Option<&serde_json::Value>,
) -> Result<serde_json::Value, String> {
    if let Some(data) = data {
        Ok(data.to_owned())
    } else {
        let construct_token = node.tokens.get(2).unwrap();
        let constructor = construct_token.value.split(' ').nth(3).unwrap();
        Err(format!(
            "\nData is missing from parameter data mapping for this loop:\n{} at line {}:{}\n{}{}\n",
            node.value,
            construct_token.line_start,
            construct_token.pos_start,
            " ".to_string().repeat(node.value.len() - constructor.len() - 2),
            "^".to_string().repeat(constructor.len()),
        ))
    }
}

fn generate_variable_data(
    variable_key: &str,
    params: &HashMap<String, serde_json::Value>,
) -> Result<String, String> {
    let variable = params.get(variable_key);
    match variable {
        Some(e) => Ok(serde_json::to_string(e).unwrap().replace('\"', "")),
        None => Err(format!(
            "\n${{{}}} is missing from parameter data mapping.\n",
            variable_key
        )),
    }
}

pub fn generate_template(
    ast: Ast,
    params: HashMap<String, serde_json::Value>,
    parent_params: HashMap<String, serde_json::Value>,
    loop_stack: Vec<String>,
) -> Result<String, String> {
    let mut html = String::new();
    let mut open_loop_stack: Vec<String> = loop_stack;

    for node in ast.nodes {
        if node.identifier == ASTNodeIdentifier::NewLine {
            html.push_str(&node.value)
        } else if node.identifier == ASTNodeIdentifier::Loop {
            let variable_iterator_name = node.value.split_whitespace().nth(1).unwrap().to_string();
            open_loop_stack.push(variable_iterator_name.clone());
            let the_node_str = node
                .value
                .replace("#}", "")
                .split_whitespace()
                .nth(3)
                .unwrap()
                .to_string();
            let list_iterator_name = if the_node_str.contains('.') {
                let the_split = the_node_str.split('.');

                the_split.last().unwrap()
            } else {
                &the_node_str
            };

            let data_retrieval = if params.get(list_iterator_name).is_some() {
                params.get(list_iterator_name)
            } else {
                parent_params.get(&the_node_str)
            };

            let list_data = match validate_loop_data(&node, data_retrieval) {
                Ok(data) => data,
                Err(e) => return Err(e),
            };

            let list_data_with_key_name = &list_data[&list_iterator_name].as_array();

            // Depending on the root mapping, this needs to be handled
            let loop_over = if list_data_with_key_name.is_some() {
                list_data_with_key_name.unwrap()
            } else {
                list_data.as_array().unwrap()
            };

            for (i, item) in loop_over.iter().enumerate() {
                let item_to_mapping = match item.as_object() {
                    Some(data) => data,
                    None => {
                        return Err(format!(
                            "Data was not contained inside of an object for list: {}",
                            list_iterator_name
                        ))
                    }
                };
                let mut new_params = HashMap::new();

                for (k, v) in item_to_mapping.iter() {
                    new_params.insert(k.clone(), v.clone());
                }

                let key_data_from_params = params.get(list_iterator_name);
                let mut clone_of_parent_params = parent_params.clone();
                if let Some(key_data_from_params) = key_data_from_params {
                    let x = key_data_from_params.as_object();
                    let y = key_data_from_params.as_array();
                    if let Some(x) = x {
                        let item = x.get(list_iterator_name).unwrap();
                        let the_item = item.get(i).unwrap();
                        if !parent_params.contains_key(&variable_iterator_name) {
                            clone_of_parent_params.remove(&variable_iterator_name);
                        }
                        clone_of_parent_params
                            .insert(variable_iterator_name.clone(), the_item.clone());
                    } else if let Some(y) = y {
                        let the_item = y.get(i).unwrap();
                        if !parent_params.contains_key(&variable_iterator_name) {
                            clone_of_parent_params.remove(&variable_iterator_name);
                        }
                        clone_of_parent_params
                            .insert(variable_iterator_name.clone(), the_item.clone());
                    }
                } else if parent_params.get(&variable_iterator_name).is_some() {
                    clone_of_parent_params.remove(&variable_iterator_name);
                    clone_of_parent_params.insert(variable_iterator_name.clone(), item.clone());
                }

                match generate_template(
                    node.children.clone().unwrap(),
                    new_params,
                    clone_of_parent_params,
                    open_loop_stack.clone(),
                ) {
                    Ok(data) => html.push_str(data.as_str()),
                    Err(e) => return Err(e),
                }
            }
            open_loop_stack.pop();
        } else if node.identifier == ASTNodeIdentifier::Variable {
            let node_value_cleaned = node.value.replace("${", "").replace('}', "");
            if node.value.contains('.') {
                let node_iterator_name = node_value_cleaned.split('.').next().unwrap().to_string();
                let node_property_name = node_value_cleaned.split('.').last().unwrap();

                match validate_iterator(&node, &node_iterator_name, &open_loop_stack) {
                    Ok(()) => (),
                    Err(e) => return Err(e),
                }

                match validate_property(
                    &node,
                    &node_iterator_name,
                    node_property_name,
                    &params,
                    &parent_params,
                ) {
                    Ok(()) => (),
                    Err(e) => return Err(e),
                }

                if parent_params.contains_key(&node_iterator_name) {
                    let the_data = parent_params.get(&node_iterator_name).unwrap().as_object();
                    if let Some(the_data) = the_data {
                        let mut new_map = HashMap::new();

                        for (k, v) in the_data.iter() {
                            new_map.insert(k.clone(), v.clone());
                        }
                        if let Ok(data) = generate_variable_data(node_property_name, &new_map) {
                            html.push_str(&data)
                        }
                    }
                }
            } else {
                match generate_variable_data(&node_value_cleaned, &params) {
                    Ok(data) => html.push_str(&data),
                    Err(e) => return Err(e),
                };
            }
        } else if node.identifier != ASTNodeIdentifier::LoopEnd {
            html.push_str(&node.value);
        }
    }

    Ok(html)
}

pub fn render_template(
    template_html: String,
    parameters: HashMap<String, serde_json::Value>,
) -> Result<String, String> {
    let tokens = generate_tokens(template_html);
    let parsed_tokens = parse_tokens(tokens);
    let ast = match construct_ast(parsed_tokens) {
        Ok(tree) => tree,
        Err(e) => return Err(e),
    };
    let loop_stack: Vec<String> = vec![];

    generate_template(ast, parameters.clone(), parameters, loop_stack)
}

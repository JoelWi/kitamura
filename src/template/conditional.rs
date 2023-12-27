use std::collections::{HashMap, HashSet};

use crate::{ast::ASTNode, template::generate_template};

fn conditional_contents(raw_condition_string: &String) -> Vec<String> {
    let contents = raw_condition_string[5..raw_condition_string.len() - 2].to_owned();
    contents.split_whitespace().map(|s| s.to_owned()).collect()
}

fn evaluate_condition_ops(
    contents_split: &Vec<String>,
    stack_ops: &mut Vec<String>,
    valid_ops: &mut Vec<bool>,
    params: &HashMap<String, serde_json::Value>,
    _parent_params: &HashMap<String, serde_json::Value>,
) {
    let valid_operands = HashSet::from(["&&", "||"]);
    for item in contents_split {
        if valid_operands.contains(item.as_str()) {
            stack_ops.push(item.to_owned());
        }
        if item.contains('?') {
            let item_split = item.split('?');
            let parameter_if = item.split('?').next().unwrap();
            let api = item_split.last().to_owned().unwrap();

            match api {
                "exists" => {
                    let parameter: Option<String> = if params.get(parameter_if).is_some() {
                        Some(params.get(parameter_if).unwrap().to_string())
                    } else {
                        None
                    };

                    match parameter {
                        None => valid_ops.push(false),
                        Some(_p) => valid_ops.push(true),
                    }
                }
                "not_empty" => {
                    let parameter: Option<String> = if params.get(parameter_if).is_some() {
                        Some(params.get(parameter_if).unwrap().to_string())
                    } else {
                        None
                    };

                    match parameter {
                        None => valid_ops.push(false),
                        Some(p) => {
                            if p.len() > 2 {
                                valid_ops.push(true)
                            } else {
                                valid_ops.push(false)
                            }
                        }
                    }
                }
                _ => println!("Not valid api but this won't hit once this moves to the parser"),
            }
        }
    }
}

fn validate_condition(
    node: &ASTNode,
    open_loop_stack: &[String],
    contents_split: &Vec<String>,
    stack_ops: &mut Vec<String>,
    valid_ops: &mut Vec<bool>,
    params: &HashMap<String, serde_json::Value>,
    parent_params: &HashMap<String, serde_json::Value>,
) -> Result<String, String> {
    match contents_split.len() {
        3 => {
            if valid_ops[0] && valid_ops[1] && stack_ops[0] == *"&&" {
                valid_ops.remove(0);
                valid_ops.remove(0);
                stack_ops.remove(0);
                match generate_template(
                    node.children.clone().unwrap(),
                    params.clone(),
                    parent_params.clone(),
                    open_loop_stack.to_owned(),
                ) {
                    Ok(data) => Ok(data),
                    Err(e) => Err(e),
                }
            } else {
                Ok("".to_owned())
            }
        }
        1 => {
            if valid_ops[0] {
                valid_ops.remove(0);
                match generate_template(
                    node.children.clone().unwrap(),
                    params.clone(),
                    parent_params.clone(),
                    open_loop_stack.to_owned(),
                ) {
                    Ok(data) => Ok(data),
                    Err(e) => Err(e),
                }
            } else {
                Ok("".to_owned())
            }
        }
        _ => Err("Invalid arguments".to_owned()),
    }
}

pub fn evaluate_condition(
    node: &ASTNode,
    params: HashMap<String, serde_json::Value>,
    parent_params: HashMap<String, serde_json::Value>,
    open_loop_stack: &[String],
) -> Result<String, String> {
    let contents_split = conditional_contents(&node.value);

    let mut stack_ops: Vec<String> = vec![];
    let mut valid_ops: Vec<bool> = vec![];

    evaluate_condition_ops(
        &contents_split,
        &mut stack_ops,
        &mut valid_ops,
        &params,
        &parent_params,
    );

    validate_condition(
        node,
        open_loop_stack,
        &contents_split,
        &mut stack_ops,
        &mut valid_ops,
        &params,
        &parent_params,
    )
}

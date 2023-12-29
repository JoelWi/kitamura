use std::collections::HashMap;

use crate::{ast::ASTNode, template::generate_template};

fn conditional_contents(raw_condition_string: &str) -> Vec<String> {
    raw_condition_string
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect()
}

fn evaluate_condition_ops(
    contents_split: &Vec<String>,
    evaluations: &mut Vec<EvalOp>,
    params: &HashMap<String, serde_json::Value>,
    _parent_params: &HashMap<String, serde_json::Value>,
) -> Result<(), String> {
    for item in contents_split {
        if item == "&&" {
            evaluations.push(EvalOp::AndOp);
        } else if item == "||" {
            evaluations.push(EvalOp::OrOp);
        } else if item.contains('?') {
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
                        None => evaluations.push(EvalOp::False),
                        Some(_p) => evaluations.push(EvalOp::True),
                    }
                }
                "not_empty" => {
                    let parameter: Option<String> = if params.get(parameter_if).is_some() {
                        Some(params.get(parameter_if).unwrap().to_string())
                    } else {
                        None
                    };

                    match parameter {
                        None => evaluations.push(EvalOp::False),
                        Some(p) => {
                            if p.len() > 2 {
                                evaluations.push(EvalOp::True)
                            } else {
                                evaluations.push(EvalOp::False)
                            }
                        }
                    }
                }
                _ => return Err(format!("Not valid api: {}", api)),
            }
        }
    }

    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
enum EvalOp {
    True,
    False,
    AndOp,
    OrOp,
}

#[derive(Debug)]
struct GroupNode {
    value: String,
    children: Option<Vec<GroupNode>>,
}

fn split_up_groups(condition_content: &str) -> Vec<GroupNode> {
    let mut groupings = vec![];
    let mut grouping = String::from("");

    let mut inner_groupings = 0;

    //let mut prev_char = ' ';
    for char in condition_content.chars() {
        match char {
            //'&' => {
            //    if prev_char == '&' {
            //        if grouping.len() > 0 {
            //            groupings.push(GroupNode {
            //                value: grouping[0..grouping.len() - 1].to_owned(),
            //                children: None,
            //            });
            //        }

            //      groupings.push(GroupNode {
            //          value: "&&".to_owned(),
            //          children: None,
            //      });

            //    prev_char = char::from(' ');
            //    grouping = String::from("");
            // } else {
            //     prev_char = char;
            //}
            //}
            //'|' => {
            //    if prev_char == '|' {
            //        println!("Length of grouping: {}", grouping.len());
            //        if grouping.len() > 0 {
            //            groupings.push(GroupNode {
            //                value: grouping[0..grouping.len() - 1].to_owned(),
            //                children: None,
            //            });
            //        }

            //        groupings.push(GroupNode {
            //            value: "||".to_owned(),
            //            children: None,
            //        });

            //        prev_char = char::from(' ');
            //        grouping = String::from("");
            //    } else {
            //        prev_char = char;
            //    }
            // }
            '(' => {
                if inner_groupings > 0 {
                    grouping.push(char);
                }
                if !grouping.is_empty() && inner_groupings == 0 {
                    groupings.push(GroupNode {
                        value: grouping,
                        children: None,
                    });
                    grouping = String::from("");
                    inner_groupings = 0;
                }
                inner_groupings += 1;
            }
            ')' => {
                if inner_groupings > 1 {
                    grouping.push(char);
                    inner_groupings -= 1;
                } else if !grouping.is_empty() {
                    if grouping.contains('(') {
                        groupings.push(GroupNode {
                            value: grouping.clone(),
                            children: Some(split_up_groups(&grouping)),
                        });
                        grouping = String::from("");
                        inner_groupings = 0;
                    } else {
                        groupings.push(GroupNode {
                            value: grouping,
                            children: None,
                        });
                        grouping = String::from("");
                        inner_groupings = 0;
                    }
                }
            }
            ' ' => {
                if !grouping.is_empty() {
                    grouping.push(char);
                }
            }
            _ => grouping.push(char),
        }
    }

    if !grouping.is_empty() && inner_groupings == 0 {
        groupings.push(GroupNode {
            value: grouping.clone(),
            children: None,
        });
    }

    groupings
}

fn final_evaluations(evaluations: Vec<EvalOp>) -> EvalOp {
    let mut prev = EvalOp::False;
    for (i, eval) in evaluations.clone().into_iter().enumerate() {
        match eval {
            EvalOp::False => {
                if i >= 2 {
                    let prev_prev = evaluations.get(i - 2).unwrap();
                    if (prev == EvalOp::OrOp && *prev_prev == EvalOp::False)
                        || (prev == EvalOp::AndOp
                            && (*prev_prev == EvalOp::True || *prev_prev == EvalOp::False))
                    {
                        return EvalOp::False;
                    }
                }

                if i == evaluations.len() - 1 && i == 0 {
                    return EvalOp::False;
                }
            }
            _ => prev = eval,
        }
    }

    EvalOp::True
}

fn evaluate_groupings(
    group_split: &Vec<GroupNode>,
    params: &HashMap<String, serde_json::Value>,
    parent_params: &HashMap<String, serde_json::Value>,
) -> Result<Vec<EvalOp>, String> {
    let mut evaluations = vec![];
    for group_node in group_split {
        if group_node.children.is_some() {
            let nested_evaluations =
                evaluate_groupings(group_node.children.as_ref().unwrap(), params, parent_params)?;

            let res = final_evaluations(nested_evaluations);
            evaluations.push(res);
        } else {
            let contents_split = conditional_contents(&group_node.value.replace(['(', ')'], ""));

            match &contents_split[0][0..] {
                "&&" => {
                    if contents_split.len() == 1 {
                        evaluations.push(EvalOp::AndOp)
                    }
                }
                "||" => {
                    if contents_split.len() == 1 {
                        evaluations.push(EvalOp::OrOp)
                    }
                }
                _ => {
                    let mut node_evaluations = vec![];

                    evaluate_condition_ops(
                        &contents_split,
                        &mut node_evaluations,
                        params,
                        parent_params,
                    )?;

                    let last_node = node_evaluations.last().unwrap();
                    //let first_node = node_evaluations.first().unwrap();

                    // Do I need this?/
                    //if *first_node == EvalOp::AndOp || *first_node == EvalOp::OrOp {
                    //    let res = final_evaluations(node_evaluations[1..].to_vec());
                    //    println!("nested eval INSIDE FIRST: {:?}", res);
                    //    evaluations.push(first_node.clone());
                    //    match res {
                    //        EvalOp::True => evaluations.push(EvalOp::True),
                    //        _ => evaluations.push(EvalOp::False),
                    //EvalOp::AndOp => evaluations.push(EvalOp::AndOp),
                    //EvalOp::OrOp => evaluations.push(EvalOp::OrOp),
                    //    };
                    if *last_node == EvalOp::AndOp || *last_node == EvalOp::OrOp {
                        let res = final_evaluations(
                            node_evaluations[0..node_evaluations.len() - 1].to_vec(),
                        );
                        match res {
                            EvalOp::True => evaluations.push(EvalOp::True),
                            _ => evaluations.push(EvalOp::False),
                            //EvalOp::AndOp => evaluations.push(EvalOp::AndOp),
                            //EvalOp::OrOp => evaluations.push(EvalOp::OrOp),
                        };
                        evaluations.push(last_node.clone());
                    } else {
                        let res = final_evaluations(node_evaluations);
                        match res {
                            EvalOp::True => evaluations.push(EvalOp::True),
                            _ => evaluations.push(EvalOp::False),
                            //EvalOp::AndOp => evaluations.push(EvalOp::AndOp),
                            //EvalOp::OrOp => evaluations.push(EvalOp::OrOp),
                        };
                    }
                }
            }
        };
    }

    // Do I need  this?
    //if evaluations.is_empty() {
    //    evaluations.push(EvalOp::False);
    //}

    Ok(evaluations)
}

pub fn evaluate_condition(
    node: &ASTNode,
    params: HashMap<String, serde_json::Value>,
    parent_params: HashMap<String, serde_json::Value>,
    open_loop_stack: &[String],
) -> Result<String, String> {
    let group_split = split_up_groups(&node.value[4..node.value.len() - 2]);

    let evaluations = evaluate_groupings(&group_split, &params, &parent_params)?;

    let last_node = evaluations.last().unwrap();
    if evaluations.len() == 2 && (*last_node == EvalOp::AndOp || *last_node == EvalOp::OrOp) {
        return Err(format!(
            "Incorrect amount of arguments, nothing of right side of : {:?}",
            last_node
        ));
    }

    let can_we_enter_the_inner_content = final_evaluations(evaluations);

    match can_we_enter_the_inner_content {
        EvalOp::True => {
            match generate_template(
                node.children.clone().unwrap(),
                params.clone(),
                parent_params.clone(),
                open_loop_stack.to_owned(),
            ) {
                Ok(data) => Ok(data),
                Err(e) => Err(e),
            }
        }
        _ => Ok("".to_owned()),
    }
}

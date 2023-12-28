use std::collections::{HashMap, HashSet};

use crate::{ast::ASTNode, template::generate_template};

fn conditional_contents(raw_condition_string: &String) -> Vec<String> {
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
) {
    for item in contents_split {
        if item == "&&" {
            evaluations.push(EvalOp::ANDOP);
        } else if item == "||" {
            evaluations.push(EvalOp::OROP);
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
                        None => evaluations.push(EvalOp::FALSE),
                        Some(_p) => evaluations.push(EvalOp::TRUE),
                    }
                }
                "not_empty" => {
                    let parameter: Option<String> = if params.get(parameter_if).is_some() {
                        Some(params.get(parameter_if).unwrap().to_string())
                    } else {
                        None
                    };

                    match parameter {
                        None => evaluations.push(EvalOp::FALSE),
                        Some(p) => {
                            if p.len() > 2 {
                                evaluations.push(EvalOp::TRUE)
                            } else {
                                evaluations.push(EvalOp::FALSE)
                            }
                        }
                    }
                }
                _ => println!("Not valid api but this won't hit once this moves to the parser"),
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum EvalOp {
    TRUE,
    FALSE,
    ANDOP,
    OROP,
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

    for char in condition_content.chars() {
        match char {
            '(' => {
                if inner_groupings > 0 {
                    grouping.push(char);
                }
                if grouping.len() > 0 && inner_groupings == 0 {
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
                //if inner_groupings == 1 {
                //    inner_groupings -= 1;
                //    if grouping.len() > 0 {
                //       groupings.push(GroupNode {
                //            value: grouping,
                //            children: None,
                //        });
                //        grouping = String::from("");
                //    }
                if inner_groupings > 1 {
                    grouping.push(char);
                    inner_groupings -= 1;
                    //if grouping.len() > 0 {
                    //    groupings.push(GroupNode {
                    //        value: grouping.clone(),
                    //        children: Some(split_up_groups(&grouping)),
                    //    });
                    //    grouping = String::from("");
                    // }
                } else {
                    if grouping.len() > 0 {
                        if grouping.contains('(') {
                            //println!("time to push to children");
                            groupings.push(GroupNode {
                                value: grouping.clone(),
                                children: Some(split_up_groups(&grouping)),
                            });
                            grouping = String::from("");
                            inner_groupings = 0;
                        } else {
                            //println!("time to push to top level");
                            groupings.push(GroupNode {
                                value: grouping,
                                children: None,
                            });
                            grouping = String::from("");
                            inner_groupings = 0;
                        }
                    }
                }
            }
            ' ' => {
                if grouping.len() > 0 {
                    grouping.push(char);
                }
            }
            _ => grouping.push(char),
        }
    }

    if grouping.len() > 0 && inner_groupings == 0 {
        groupings.push(GroupNode {
            value: grouping.clone(),
            children: None,
        });
    }

    println!("grouping that is left: {}", grouping);
    return groupings;
}

fn final_evaluations(evaluations: Vec<EvalOp>) -> bool {
    let mut prev = EvalOp::FALSE;
    for (i, eval) in evaluations.clone().into_iter().enumerate() {
        match eval {
            EvalOp::FALSE => {
                if i >= 2 {
                    let prev_prev = evaluations.get(i - 2).unwrap();
                    if prev == EvalOp::OROP && *prev_prev == EvalOp::FALSE {
                        return false;
                    } else if prev == EvalOp::ANDOP
                        && (*prev_prev == EvalOp::TRUE || *prev_prev == EvalOp::FALSE)
                    {
                        return false;
                    }
                }

                if i == evaluations.len() {
                    return false;
                }
            }
            _ => prev = eval,
        }
    }

    true
}

fn evaluate_groupings(
    group_split: &Vec<GroupNode>,
    params: &HashMap<String, serde_json::Value>,
    parent_params: &HashMap<String, serde_json::Value>,
) -> Vec<EvalOp> {
    let mut evaluations = vec![];
    for group_node in group_split {
        if group_node.children.is_some() {
            //println!("Inside child grouping");
            let nested_evaluations = evaluate_groupings(
                &group_node.children.as_ref().unwrap(),
                params,
                parent_params,
            );
            let res = final_evaluations(nested_evaluations);
            //println!("nested eval: {}", res);
            if res {
                evaluations.push(EvalOp::TRUE);
            } else {
                evaluations.push(EvalOp::FALSE);
            }
        } else {
            //println!("inside top level grouping");
            let contents_split =
                conditional_contents(&group_node.value.replace("(", "").replace(")", ""));

            match &contents_split[0][0..] {
                "&&" => evaluations.push(EvalOp::ANDOP),
                "||" => evaluations.push(EvalOp::OROP),
                _ => {
                    let mut node_evauations = vec![];

                    evaluate_condition_ops(
                        &contents_split,
                        &mut node_evauations,
                        &params,
                        &parent_params,
                    );

                    println!("{:?}", contents_split);
                    println!("{:?}", node_evauations);
                    let res = final_evaluations(node_evauations);
                    println!("nested eval: {}", res);
                    match res {
                        true => evaluations.push(EvalOp::TRUE),
                        false => evaluations.push(EvalOp::FALSE),
                    };
                }
            }
        };
    }

    if evaluations.len() == 0 {
        evaluations.push(EvalOp::FALSE);
    }

    println!("final evals being returned:");
    println!("{:?}", evaluations);

    evaluations
}

pub fn evaluate_condition(
    node: &ASTNode,
    params: HashMap<String, serde_json::Value>,
    parent_params: HashMap<String, serde_json::Value>,
    open_loop_stack: &[String],
) -> Result<String, String> {
    let group_split = split_up_groups(&node.value[4..node.value.len() - 2]);
    println!("{:#?}", group_split);

    let evaluations = evaluate_groupings(&group_split, &params, &parent_params);
    println!("final eval after all groupings:");
    println!("{:?}", evaluations);

    let can_we_enter_the_inner_content = final_evaluations(evaluations);

    match can_we_enter_the_inner_content {
        false => Ok("".to_owned()),
        true => {
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
    }
}

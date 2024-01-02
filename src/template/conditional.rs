use std::collections::HashMap;

use crate::{
    ast::ASTNode,
    error::{Error, TemplateResult},
    template::generate_template,
};

fn conditional_contents(raw_condition_string: &str) -> Vec<String> {
    raw_condition_string
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect()
}

fn evaluate_condition_ops(
    contents_split: &[String],
    evaluations: &mut Vec<EvalOp>,
    params: &HashMap<String, serde_json::Value>,
    _parent_params: &HashMap<String, serde_json::Value>,
) -> Result<(), Error> {
    for (i, item) in contents_split.iter().enumerate() {
        if item == "&&" {
            evaluations.push(EvalOp::AndOp);
        } else if item == "||" {
            evaluations.push(EvalOp::OrOp);
        } else if item == "==" {
            let parameter = contents_split.get(i - 1).unwrap();
            let value = contents_split.get(i + 1).unwrap().replace('\'', "");
            let matches = params.get(parameter).unwrap().to_string().replace('"', "");

            match value[1..value.len() - 1] == matches[1..matches.len() - 1] {
                true => evaluations.push(EvalOp::True),
                false => evaluations.push(EvalOp::False),
            }
        } else if item == "!=" {
            let parameter = contents_split.get(i - 1).unwrap();
            let value = contents_split.get(i + 1).unwrap().replace('\'', "");
            let matches = params.get(parameter).unwrap().to_string().replace('"', "");

            match value[1..value.len() - 1] != matches[1..matches.len() - 1] {
                true => evaluations.push(EvalOp::True),
                false => evaluations.push(EvalOp::False),
            }
        } else if item.contains('?') {
            let item_split = item.split('?');
            let parameter_if = item.split('?').next().unwrap();
            let api = item_split
                .clone()
                .last()
                .to_owned()
                .unwrap()
                .split('(')
                .next()
                .unwrap();

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
                "contains" => {
                    let parameter = params.get(parameter_if).unwrap().to_string();
                    let contains_val = item_split
                        .last()
                        .unwrap()
                        .to_owned()
                        .split('(')
                        .last()
                        .unwrap()
                        .to_owned();

                    match parameter.contains(&contains_val[1..contains_val.len() - 2]) {
                        false => evaluations.push(EvalOp::False),
                        true => evaluations.push(EvalOp::True),
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
                _ => return Err(Error::InvalidApi(format!("Not valid api: {}", api))),
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
    let mut entered_api_val = false;

    for char in condition_content.chars() {
        match char {
            '(' => {
                if inner_groupings > 0 {
                    grouping.push(char);
                }

                if grouping.contains("?contains") && grouping[grouping.len() - 9..] == *"?contains"
                {
                    grouping.push(char);
                    entered_api_val = true;
                    inner_groupings -= 1;
                } else if !grouping.is_empty() && inner_groupings == 0 {
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
                if entered_api_val {
                    grouping.push(char);
                    entered_api_val = false;
                } else if inner_groupings > 1 {
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

fn sanitise_string(raw_string: String) -> String {
    let mut latest_input: [char; 9] = ['!', '!', '!', '!', '!', '!', '!', '!', '!'];
    let mut head = 0;
    let mut sanitised_string = String::from("");
    let mut was_ob_added = false;

    for char in raw_string.chars() {
        match char {
            '(' => {
                let li_str = {
                    let mut tmp = String::from("");
                    let tail = {
                        if head != 0 {
                            head - 1
                        } else {
                            tmp.push(latest_input[head]);
                            head += 1;
                            head - 1
                        }
                    };

                    while tail != head {
                        tmp.push(latest_input[head]);
                        head += 1;
                        if head > 8 {
                            head = 0;
                        }
                    }

                    if tail != 0 {
                        tmp.push(latest_input[head]);
                    }

                    tmp
                };

                if li_str.as_str() == "?contains" {
                    sanitised_string.push(char);
                    was_ob_added = true;
                }
            }
            ')' => match was_ob_added {
                true => {
                    sanitised_string.push(char);
                    was_ob_added = false;
                }
                false => {}
            },
            _ => sanitised_string.push(char),
        }
        latest_input[head] = char;

        head += 1;
        if head > 8 {
            head = 0;
        }
    }

    sanitised_string
}

fn evaluate_groupings(
    group_split: &Vec<GroupNode>,
    params: &HashMap<String, serde_json::Value>,
    parent_params: &HashMap<String, serde_json::Value>,
) -> Result<Vec<EvalOp>, Error> {
    let mut evaluations = vec![];
    for group_node in group_split {
        if group_node.children.is_some() {
            let nested_evaluations =
                evaluate_groupings(group_node.children.as_ref().unwrap(), params, parent_params)?;

            let res = final_evaluations(nested_evaluations);
            evaluations.push(res);
        } else {
            let sanitised_string = sanitise_string(group_node.value.clone());
            let contents_split = conditional_contents(&sanitised_string);

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

                    if *last_node == EvalOp::AndOp || *last_node == EvalOp::OrOp {
                        let res = final_evaluations(
                            node_evaluations[0..node_evaluations.len() - 1].to_vec(),
                        );
                        match res {
                            EvalOp::True => evaluations.push(EvalOp::True),
                            _ => evaluations.push(EvalOp::False),
                        };
                        evaluations.push(last_node.clone());
                    } else {
                        let res = final_evaluations(node_evaluations);
                        match res {
                            EvalOp::True => evaluations.push(EvalOp::True),
                            _ => evaluations.push(EvalOp::False),
                        };
                    }
                }
            }
        };
    }

    Ok(evaluations)
}

pub fn evaluate_condition(
    node: &ASTNode,
    params: HashMap<String, serde_json::Value>,
    parent_params: HashMap<String, serde_json::Value>,
    open_loop_stack: &[String],
) -> TemplateResult {
    let group_split = split_up_groups(&node.value[4..node.value.len() - 2]);

    let evaluations = evaluate_groupings(&group_split, &params, &parent_params)?;

    let last_node = evaluations.last().unwrap();
    if evaluations.len() == 2 && (*last_node == EvalOp::AndOp || *last_node == EvalOp::OrOp) {
        return Err(Error::InvalidSyntax(format!(
            "Incorrect amount of arguments, nothing of right side of : {:?}",
            last_node
        )));
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

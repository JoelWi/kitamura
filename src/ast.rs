use crate::token::{Identifier, Token};

#[derive(Debug, Default, Clone)]
pub struct Ast {
    pub nodes: Vec<ASTNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ASTNodeIdentifier {
    Unknown,
    Text,
    Variable,
    Loop,
    LoopEnd,
    NewLine,
}

#[derive(Debug, Clone)]
pub struct ASTNode {
    pub identifier: ASTNodeIdentifier,
    pub value: String,
    pub tokens: Vec<Token>,
    pub children: Option<Ast>,
}

fn assign_identity(ast_node: &mut ASTNode) -> Result<(), String> {
    let opening_chars = &ast_node.value[0..2];
    if opening_chars == "${" && ast_node.value.ends_with('}') {
        ast_node.identifier = ASTNodeIdentifier::Variable;
    } else if opening_chars == "{#" && ast_node.value.ends_with("#}") {
        let control_flow = ast_node.value.split(' ').next().unwrap();
        match control_flow {
            "{#endfor#}" => ast_node.identifier = ASTNodeIdentifier::LoopEnd,
            "{#for" => {
                ast_node.identifier = ASTNodeIdentifier::Loop;
                ast_node.children = Some(Ast { nodes: vec![] });
            }
            _ => {
                let construct_token = ast_node.tokens.get(2).unwrap();
                return Err(format!(
                    "\nUnknown construct: {} at line {}:{}\n{}{}\n",
                    ast_node.value,
                    construct_token.line_start,
                    construct_token.pos_start,
                    " ".to_string().repeat(21),
                    "^".to_string().repeat(control_flow.len() - 2),
                ));
            }
        }
    };
    Ok(())
}
pub fn construct_ast(parsed_tokens: Vec<Vec<Token>>) -> Result<Ast, String> {
    let mut open_brace_count = 0;
    let mut constructed_ast = Ast { nodes: vec![] };

    // Iterate over groupings of tokens that make up something e.g. variable
    for token_group in parsed_tokens {
        let mut ast_node = ASTNode {
            identifier: ASTNodeIdentifier::Unknown,
            value: String::new(),
            tokens: vec![],
            children: Option::None,
        };

        let mut bad_token = Token::new(Identifier::Text, 1, 1, 1, 1);

        // Iterate over a group and construct to ASTNode
        for token in token_group {
            // Validation
            match token.identifier {
                Identifier::Text => ast_node.identifier = ASTNodeIdentifier::Text,
                Identifier::NewLine => ast_node.identifier = ASTNodeIdentifier::NewLine,
                _ => {
                    if token.value == "{" {
                        open_brace_count += 1;
                    }
                    if open_brace_count > 1 && bad_token.value.is_empty() {
                        bad_token = token.clone();
                    }
                }
            }

            // ASTNode construction
            ast_node.value.push_str(token.value.as_str());
            ast_node.tokens.push(token);
        }

        // Can add other stuff here when it comes e.g. conditions
        if ast_node.value.len() > 2 {
            match assign_identity(&mut ast_node) {
                Ok(()) => {}
                Err(e) => return Err(e),
            }
        }

        // Variable error handling
        if open_brace_count > 1 {
            return Err(format!(
                "Error: Extra opening {{ found at line: {} position: {} in {}",
                bad_token.line_start,
                bad_token.pos_start - 1,
                ast_node.value
            ));
        }

        // Sanitise whitespace only tokens good idea?
        let last_node = constructed_ast.nodes.last();

        if let Some(last_node) = last_node {
            if (ast_node.identifier == ASTNodeIdentifier::LoopEnd
                || ast_node.identifier == ASTNodeIdentifier::Loop)
                && last_node.value.replace(' ', "").is_empty()
            {
                constructed_ast.nodes.pop();
            }
        }

        match ast_node.identifier {
            ASTNodeIdentifier::NewLine => {
                if constructed_ast.nodes.last().unwrap().identifier != ASTNodeIdentifier::LoopEnd {
                    constructed_ast.nodes.push(ast_node);
                }
            }
            _ => constructed_ast.nodes.push(ast_node),
        }

        open_brace_count = 0;
    }

    // Naive iteration to move children nodes into parent

    let mut new_ast = Ast { nodes: vec![] };
    let mut loop_node_vec: Vec<ASTNode> = vec![];

    for node in constructed_ast.nodes {
        match node.identifier {
            ASTNodeIdentifier::Loop => {
                loop_node_vec.push(node);
            }
            ASTNodeIdentifier::LoopEnd => {
                if loop_node_vec.len() > 1 {
                    let length = loop_node_vec.len() - 1;
                    let latest_node = loop_node_vec[length].clone();

                    let length_of_next = length - 1;
                    let latest_node_next =
                        &mut loop_node_vec[length_of_next].children.as_mut().unwrap();
                    latest_node_next.nodes.push(latest_node);
                    loop_node_vec.pop();
                } else {
                    new_ast.nodes.push(loop_node_vec.last().unwrap().clone());
                    loop_node_vec.pop();
                }

                if !loop_node_vec.is_empty() {
                    let length = loop_node_vec.len() - 1;
                    let latest_node = &mut loop_node_vec[length].children.as_mut().unwrap();
                    latest_node.nodes.push(node);
                } else {
                    new_ast.nodes.push(node);
                }
            }
            ASTNodeIdentifier::NewLine => {
                if !loop_node_vec.is_empty() {
                    let length = loop_node_vec.len() - 1;
                    let latest_node = &mut loop_node_vec[length].children.as_mut().unwrap();

                    if new_ast.nodes.last().unwrap().identifier != ASTNodeIdentifier::NewLine
                        || !latest_node.nodes.is_empty()
                    {
                        latest_node.nodes.push(node);
                    }
                } else {
                    new_ast.nodes.push(node);
                }
            }
            _ => {
                if !loop_node_vec.is_empty() {
                    let length = loop_node_vec.len() - 1;
                    let latest_node = &mut loop_node_vec[length].children.as_mut().unwrap();
                    latest_node.nodes.push(node);
                } else {
                    new_ast.nodes.push(node);
                }
            }
        }
    }

    // Catch for any open loop control flows that weren't closed
    if !loop_node_vec.is_empty() {
        return Err(format!(
            "Control flow '{}' has no closing statement",
            loop_node_vec.last().unwrap().value
        ));
    }

    Ok(new_ast)
}

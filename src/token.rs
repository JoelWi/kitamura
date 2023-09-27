#[derive(Debug, Clone, PartialEq)]
pub enum Identifier {
    Text,
    Dollar,
    OpenBracket,
    ClosedBracket,
    Pound,
    NewLine,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub identifier: Identifier,
    pub value: String,
    pub line_start: i32,
    line_end: i32,
    pub pos_start: i32,
    pub pos_end: i32,
}

impl Token {
    pub fn new(
        identifier: Identifier,
        line_start: i32,
        line_end: i32,
        pos_start: i32,
        pos_end: i32,
    ) -> Self {
        Token {
            identifier,
            value: String::new(),
            line_start,
            line_end,
            pos_start,
            pos_end,
        }
    }
}

fn check_for_special_identifier(char: &char) -> bool {
    *char == '$' || *char == '{' || *char == '}' || *char == '#' || *char == '\n'
}

pub fn generate_tokens(template_html: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut token = Token::new(Identifier::Text, 1, 1, 1, 1);
    let mut i = 1;
    let mut current_line = 1;

    for char in template_html.chars() {
        let special_char_match = check_for_special_identifier(&char);

        match special_char_match {
            true => {
                // Push the text token if prev token wasn't special
                if token.value.len() > 0 {
                    tokens.push(token);
                }

                // new token setup to account for the above
                token = Token::new(Identifier::Text, current_line, current_line, i + 1, i + 1);
                token.value.push(char);

                match char {
                    '$' => token.identifier = Identifier::Dollar,
                    '{' => token.identifier = Identifier::OpenBracket,
                    '}' => token.identifier = Identifier::ClosedBracket,
                    '#' => token.identifier = Identifier::Pound,
                    '\n' => {
                        token.identifier = Identifier::NewLine;
                        current_line = current_line + 1;
                        token.line_end = current_line;
                        i = 0;
                    }
                    _ => {}
                }

                // Push special char token and generate new token setup
                tokens.push(token);

                token = Token::new(Identifier::Text, current_line, current_line, i + 2, i + 2);
            }
            false => token.value.push(char),
        }
        i = i + 1;
        token.pos_end = i;
    }

    // Push final token builder after loop exit
    if token.value.len() > 0 {
        tokens.push(token);
    }
    tokens
}

pub fn parse_tokens(tokens: Vec<Token>) -> Vec<Vec<Token>> {
    let mut variables: Vec<Vec<Token>> = vec![];
    let mut variable: Vec<Token> = vec![];

    for token in tokens {
        match token.identifier {
            Identifier::Dollar => variable.push(token),
            Identifier::OpenBracket => variable.push(token),
            Identifier::ClosedBracket => {
                variable.push(token);
                variables.push(variable);
                variable = vec![];
            }
            Identifier::Pound => variable.push(token),
            Identifier::Text => {
                if variable.len() > 0 {
                    variable.push(token);
                } else {
                    variables.push(vec![token]);
                }
            }
            Identifier::NewLine => variables.push(vec![token]),
        }
    }

    variables
}

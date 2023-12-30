#[derive(Debug)]
pub enum Error {
    InvalidSyntax(String),
    InvalidApi(String),
    Unknown(String),
}

pub type TemplateResult = Result<String, Error>;

use std::collections::HashMap;

mod ast;
mod template;
mod token;

pub fn render_template(html: String, parameters: HashMap<String, serde_json::Value>) -> String {
    template::render_template(html, parameters)
}

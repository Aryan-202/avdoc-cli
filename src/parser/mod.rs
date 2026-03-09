pub mod js_parser;
pub mod python_parser;
pub mod go_parser;



pub trait Parser {
    fn parse_functions(&self, content: &str) -> Vec<crate::models::function::Function>;
    fn parse_imports(&self, content: &str) -> Vec<String>;
    fn parse_comments(&self, content: &str) -> Vec<String>;
}

pub fn get_parser(language: &str) -> Option<Box<dyn Parser>> {
    match language {
        "javascript" | "typescript" => Some(Box::new(js_parser::JsParser)),
        "python" => Some(Box::new(python_parser::PythonParser)),
        "go" => Some(Box::new(go_parser::GoParser)),
        _ => None,
    }
}
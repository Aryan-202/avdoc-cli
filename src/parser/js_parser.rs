use regex::Regex;
use crate::models::function::{Function, Parameter};
use super::Parser;

pub struct JsParser;

impl Parser for JsParser {
    fn parse_functions(&self, content: &str) -> Vec<Function> {
        let mut functions = Vec::new();
        
        // Match function declarations
        let function_re = Regex::new(r"(?m)^\s*function\s+([a-zA-Z_$][a-zA-Z0-9_$]*)\s*\(([^)]*)\)").unwrap();
        for cap in function_re.captures_iter(content) {
            let name = cap[1].to_string();
            let params_str = &cap[2];
            
            let parameters = params_str
                .split(',')
                .filter(|p| !p.trim().is_empty())
                .map(|p| Parameter {
                    name: p.trim().to_string(),
                    type_name: None,
                })
                .collect();
            
            functions.push(Function {
                name,
                parameters,
                return_type: None,
                doc_comment: self.extract_doc_comment(content, &cap[0]),
                line_number: self.find_line_number(content, &cap[0]),
            });
        }
        
        // Match arrow functions and methods
        // This is a simplified version - a real parser would be more comprehensive
        
        functions
    }
    
    fn parse_imports(&self, content: &str) -> Vec<String> {
        let mut imports = Vec::new();
        
        // Match ES6 imports
        let import_re = Regex::new(r#"(?m)^\s*import\s+.*?\s+from\s+['"]([^'"]+)['"]"#).unwrap();
        for cap in import_re.captures_iter(content) {
            imports.push(cap[1].to_string());
        }
        
        // Match require statements
        let require_re = Regex::new(r#"(?m)require\s*\(\s*['"]([^'"]+)['"]\s*\)"#).unwrap();
        for cap in require_re.captures_iter(content) {
            imports.push(cap[1].to_string());
        }
        
        imports
    }
    
    fn parse_comments(&self, content: &str) -> Vec<String> {
        let mut comments = Vec::new();
        
        // Match JSDoc comments
        let jsdoc_re = Regex::new(r"(?s)/\*\*.*?\*/").unwrap();
        for mat in jsdoc_re.find_iter(content) {
            comments.push(mat.as_str().to_string());
        }
        
        // Match single-line comments
        let line_comment_re = Regex::new(r"(?m)//.*$").unwrap();
        for mat in line_comment_re.find_iter(content) {
            comments.push(mat.as_str().to_string());
        }
        
        comments
    }
}

impl JsParser {
    fn extract_doc_comment(&self, content: &str, function_str: &str) -> Option<String> {
        let lines: Vec<&str> = content.lines().collect();
        let function_line = self.find_line_number(content, function_str);
        
        // Look for JSDoc comment above the function
        for i in (0..function_line.saturating_sub(5)).rev() {
            let line = lines.get(i)?;
            if line.trim().starts_with("//") {
                continue;
            }
            if line.trim().starts_with("/*") || line.trim().starts_with("/**") {
                // Collect the comment block
                let mut comment = String::new();
                let mut j = i;
                while j < lines.len() && !lines[j].trim().ends_with("*/") {
                    comment.push_str(lines[j]);
                    comment.push('\n');
                    j += 1;
                }
                if j < lines.len() {
                    comment.push_str(lines[j]);
                }
                return Some(comment);
            }
            if !line.trim().is_empty() && !line.trim().starts_with('*') {
                break;
            }
        }
        
        None
    }
    
    fn find_line_number(&self, content: &str, pattern: &str) -> usize {
        content.lines()
            .position(|line| line.contains(pattern))
            .unwrap_or(0)
    }
}
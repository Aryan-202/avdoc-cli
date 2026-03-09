use regex::Regex;
use crate::models::function::{Function, Parameter};
use super::Parser;

pub struct GoParser;

impl Parser for GoParser {
    fn parse_functions(&self, content: &str) -> Vec<Function> {
        let mut functions = Vec::new();
        
        // Match function declarations
        let function_re = Regex::new(r"(?m)^\s*func\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(([^)]*)\)\s*(?:\(?([^)]*)\)?)?\s*\{").unwrap();
        for cap in function_re.captures_iter(content) {
            let name = cap[1].to_string();
            let params_str = &cap[2];
            let return_str = cap.get(3).map(|m| m.as_str());
            
            let parameters = params_str
                .split(',')
                .filter(|p| !p.trim().is_empty())
                .map(|p| {
                    let parts: Vec<&str> = p.trim().split_whitespace().collect();
                    if parts.len() == 2 {
                        Parameter {
                            name: parts[0].to_string(),
                            type_name: Some(parts[1].to_string()),
                        }
                    } else {
                        Parameter {
                            name: parts[0].to_string(),
                            type_name: None,
                        }
                    }
                })
                .collect();
            
            functions.push(Function {
                name,
                parameters,
                return_type: return_str.map(|s| s.trim().to_string()),
                doc_comment: self.extract_comment(content, &cap[0]),
                line_number: self.find_line_number(content, &cap[0]),
            });
        }
        
        functions
    }
    
    fn parse_imports(&self, content: &str) -> Vec<String> {
        let mut imports = Vec::new();
        
        // Match import statements
        let import_re = Regex::new(r#"(?m)^\s*import\s+\(?\s*"?([^"\n]+)"?\s*\)?"#).unwrap();
        for cap in import_re.captures_iter(content) {
            imports.push(cap[1].to_string());
        }
        
        imports
    }
    
    fn parse_comments(&self, content: &str) -> Vec<String> {
        let mut comments = Vec::new();
        
        // Match single-line comments
        let line_comment_re = Regex::new(r"(?m)//.*$").unwrap();
        for mat in line_comment_re.find_iter(content) {
            comments.push(mat.as_str().to_string());
        }
        
        // Match multi-line comments
        let multi_comment_re = Regex::new(r"(?s)/\*.*?\*/").unwrap();
        for mat in multi_comment_re.find_iter(content) {
            comments.push(mat.as_str().to_string());
        }
        
        comments
    }
}

impl GoParser {
    fn extract_comment(&self, content: &str, function_str: &str) -> Option<String> {
        let lines: Vec<&str> = content.lines().collect();
        let function_line = self.find_line_number(content, function_str);
        
        // Look for comment above the function
        for i in (0..function_line).rev() {
            let line = lines.get(i)?.trim();
            if line.is_empty() {
                continue;
            }
            if line.starts_with("//") {
                return Some(line.to_string());
            }
            if line.starts_with("/*") {
                // Collect multi-line comment
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
            break;
        }
        
        None
    }
    
    fn find_line_number(&self, content: &str, pattern: &str) -> usize {
        content.lines()
            .position(|line| line.contains(pattern))
            .unwrap_or(0)
    }
}
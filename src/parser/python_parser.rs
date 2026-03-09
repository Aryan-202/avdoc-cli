use super::Parser;
use crate::models::function::{Function, Parameter};
use regex::Regex;

pub struct PythonParser;

impl Parser for PythonParser {
    fn parse_functions(&self, content: &str) -> Vec<Function> {
        let mut functions = Vec::new();

        // Match function definitions
        let function_re =
            Regex::new(r"(?m)^\s*def\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(([^)]*)\)\s*:").unwrap();
        for cap in function_re.captures_iter(content) {
            let name = cap[1].to_string();
            let params_str = &cap[2];

            let parameters = params_str
                .split(',')
                .filter(|p| !p.trim().is_empty())
                .map(|p| {
                    let parts: Vec<&str> = p.split(':').collect();
                    let name = parts[0].trim().to_string();
                    let type_name = parts.get(1).map(|t| t.trim().to_string());

                    Parameter { name, type_name }
                })
                .collect();

            functions.push(Function {
                name,
                parameters,
                return_type: self.extract_return_type(content, &cap[0]),
                doc_comment: self.extract_docstring(content, &cap[0]),
                line_number: self.find_line_number(content, &cap[0]),
            });
        }

        functions
    }

    fn parse_imports(&self, content: &str) -> Vec<String> {
        let mut imports = Vec::new();

        // Match import statements
        let import_re = Regex::new(r"(?m)^\s*import\s+([a-zA-Z0-9_.]+)").unwrap();
        for cap in import_re.captures_iter(content) {
            imports.push(cap[1].to_string());
        }

        // Match from ... import statements
        let from_re = Regex::new(r"(?m)^\s*from\s+([a-zA-Z0-9_.]+)\s+import").unwrap();
        for cap in from_re.captures_iter(content) {
            imports.push(cap[1].to_string());
        }

        imports
    }

    fn parse_comments(&self, content: &str) -> Vec<String> {
        let mut comments = Vec::new();

        // Match single-line comments
        let line_comment_re = Regex::new(r"(?m)#.*$").unwrap();
        for mat in line_comment_re.find_iter(content) {
            comments.push(mat.as_str().to_string());
        }

        // Match multi-line strings used as docstrings
        // This is simplified - real implementation would need to handle string literals
        let docstring_re = Regex::new(r#"(?s)""".*?"""|'''.*?'''"#).unwrap();
        for mat in docstring_re.find_iter(content) {
            comments.push(mat.as_str().to_string());
        }

        comments
    }
}

impl PythonParser {
    fn extract_docstring(&self, content: &str, function_str: &str) -> Option<String> {
        let lines: Vec<&str> = content.lines().collect();
        let function_line = self.find_line_number(content, function_str);

        // Look for docstring immediately after function definition
        if let Some(next_line) = lines.get(function_line + 1) {
            let trimmed = next_line.trim();
            if trimmed.starts_with("\"\"\"") || trimmed.starts_with("'''") {
                let mut docstring = next_line.to_string();
                let mut i = function_line + 2;

                while i < lines.len() {
                    let line = lines[i];
                    docstring.push('\n');
                    docstring.push_str(line);

                    if line.trim().ends_with("\"\"\"") || line.trim().ends_with("'''") {
                        break;
                    }
                    i += 1;
                }

                return Some(docstring);
            }
        }

        None
    }

    fn extract_return_type(&self, _content: &str, function_str: &str) -> Option<String> {
        // Look for return type annotation
        let return_re = Regex::new(r"->\s*([a-zA-Z_][a-zA-Z0-9_\[\],\s]*)").unwrap();
        if let Some(cap) = return_re.captures(function_str) {
            return Some(cap[1].trim().to_string());
        }
        None
    }

    fn find_line_number(&self, content: &str, pattern: &str) -> usize {
        content
            .lines()
            .position(|line| line.contains(pattern))
            .unwrap_or(0)
    }
}

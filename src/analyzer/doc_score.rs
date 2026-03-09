use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::repo::Repository;
use crate::parser::{self};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoreResult {
    pub overall_score: u8,
    pub file_scores: HashMap<String, u8>,
    pub total_files: usize,
    pub documented_functions: usize,
    pub undocumented_functions: usize,
    pub recommendations: Vec<String>,
}

pub struct DocScoreAnalyzer;

impl DocScoreAnalyzer {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn analyze(&self, repo: &Repository) -> Result<ScoreResult> {
        let mut file_scores = HashMap::new();
        let mut total_documented = 0;
        let mut total_undocumented = 0;
        let mut recommendations = Vec::new();
        
        for file in &repo.files {
            if let Some(content) = &file.content {
                if let Some(parser) = parser::get_parser(&file.language) {
                    let functions = parser.parse_functions(content);
                    let comments = parser.parse_comments(content);
                    
                    let documented = functions.iter()
                        .filter(|f| f.doc_comment.is_some())
                        .count();
                    
                    total_documented += documented;
                    total_undocumented += functions.len() - documented;
                    
                    // Calculate file score
                    let file_score = self.calculate_file_score(
                        functions.len(),
                        documented,
                        comments.len(),
                        content.len()
                    );
                    
                    file_scores.insert(file.path.to_string_lossy().to_string(), file_score);
                    
                    // Generate recommendations
                    if file_score < 60 {
                        recommendations.push(format!(
                            "Add documentation to {}. Found {} undocumented functions.",
                            file.path.display(),
                            functions.len() - documented
                        ));
                    }
                }
            }
        }
        
        // Calculate overall score
        let overall_score = if file_scores.is_empty() {
            0
        } else {
            (file_scores.values().sum::<u8>() as f64 / file_scores.len() as f64).round() as u8
        };
        
        Ok(ScoreResult {
            overall_score,
            file_scores,
            total_files: repo.files.len(),
            documented_functions: total_documented,
            undocumented_functions: total_undocumented,
            recommendations,
        })
    }
    
    fn calculate_file_score(&self, total_fns: usize, documented: usize, comments: usize, lines: usize) -> u8 {
        if total_fns == 0 {
            return 100; // No functions to document
        }
        
        // Documentation coverage (60% weight)
        let coverage = (documented as f64 / total_fns as f64) * 60.0;
        
        // Comment density (20% weight)
        let comment_density = if lines > 0 {
            ((comments as f64 * 5.0) / lines as f64).min(20.0)
        } else {
            0.0
        };
        
        // Bonus points for having README or project docs (20% weight)
        let has_readme = 20.0; // Simplified - would check actual existence
        
        let score = coverage + comment_density + has_readme;
        score.round() as u8
    }
}
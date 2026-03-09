use anyhow::Result;

use crate::models::repo::Repository;
use crate::models::service::Architecture;
use crate::ai::llm_client::LLMClient;

pub struct MermaidGenerator {
    llm_client: LLMClient,
}

impl MermaidGenerator {
    pub fn new(llm_client: LLMClient) -> Self {
        Self { llm_client }
    }
    
    pub async fn generate(&self, repo: &Repository, architecture: &Architecture) -> Result<String> {
        // First try AI-generated diagram
        match self.generate_ai_diagram(repo, architecture).await {
            Ok(diagram) => Ok(diagram),
            Err(_) => Ok(self.generate_basic_diagram(architecture)), // Fallback to basic diagram
        }
    }
    
    async fn generate_ai_diagram(&self, repo: &Repository, architecture: &Architecture) -> Result<String> {
        let prompt = self.build_prompt(repo, architecture);
        self.llm_client.generate_diagram(&prompt).await
    }
    
    fn generate_basic_diagram(&self, architecture: &Architecture) -> String {
        let mut diagram = String::new();
        
        diagram.push_str("graph TD\n");
        diagram.push_str("    %% Architecture Diagram\n");
        
        // Add services as nodes
        for (i, service) in architecture.services.iter().enumerate() {
            let node_id = format!("S{}", i + 1);
            diagram.push_str(&format!(
                "    {}[{}]\n",
                node_id,
                service.name
            ));
        }
        
        // Add dependencies as edges
        for (i, service) in architecture.services.iter().enumerate() {
            let source_id = format!("S{}", i + 1);
            
            for dep in &service.dependencies {
                if let Some(target_idx) = architecture.services.iter()
                    .position(|s| s.name.to_lowercase().contains(&dep.to_lowercase())) {
                    let target_id = format!("S{}", target_idx + 1);
                    diagram.push_str(&format!("    {} --> {}\n", source_id, target_id));
                }
            }
        }
        
        diagram
    }
    
    fn build_prompt(&self, repo: &Repository, architecture: &Architecture) -> String {
        let mut prompt = String::from("Generate a Mermaid.js architecture diagram for this repository.\n\n");
        
        prompt.push_str("Repository structure:\n");
        for file in &repo.files {
            prompt.push_str(&format!("- {} ({})\n", file.path.display(), file.language));
        }
        
        prompt.push_str("\nDetected services:\n");
        for service in &architecture.services {
            prompt.push_str(&format!("- {}: {:?}\n", service.name, service.service_type));
            prompt.push_str(&format!("  Dependencies: {:?}\n", service.dependencies));
        }
        
        prompt.push_str("\nGenerate a clean Mermaid.js graph TD diagram that shows the relationships between these services.");
        
        prompt
    }
}
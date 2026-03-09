use anyhow::Result;
use std::collections::{HashMap, HashSet};

use crate::models::repo::Repository;
use crate::models::service::{Architecture, Service, ServiceType};
use crate::parser::{self};

pub struct ArchitectureAnalyzer;

impl ArchitectureAnalyzer {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn analyze(&self, repo: &Repository) -> Result<Architecture> {
        let mut services = Vec::new();
        let mut dependency_graph = HashMap::new();
        
        // Group files by service type
        let mut service_files: HashMap<ServiceType, Vec<&crate::models::repo::SourceFile>> = HashMap::new();
        
        for file in &repo.files {
            let service_type = self.detect_service_type(file);
            service_files.entry(service_type).or_default().push(file);
        }
        
        // Analyze each service
        for (service_type, files) in service_files {
            let mut service = Service {
                name: format!("{:?}", service_type),
                service_type,
                files: files.iter().map(|f| f.path.clone()).collect(),
                dependencies: HashSet::new(),
                apis: Vec::new(),
            };
            
            // Analyze dependencies
            for file in files {
                if let Some(content) = &file.content {
                    if let Some(parser) = parser::get_parser(&file.language) {
                        let imports = parser.parse_imports(content);
                        for import in imports {
                            service.dependencies.insert(import);
                        }
                    }
                }
            }
            
            services.push(service);
        }
        
        // Build dependency graph
        for service in &services {
            let deps: HashSet<String> = service.dependencies.iter()
                .filter_map(|dep| self.resolve_dependency(dep, &services))
                .collect();
            dependency_graph.insert(service.name.clone(), deps);
        }
        
        Ok(Architecture {
            services,
            dependency_graph,
            entry_points: self.detect_entry_points(repo),
        })
    }
    
    fn detect_service_type(&self, file: &crate::models::repo::SourceFile) -> ServiceType {
        let path = file.path.to_string_lossy();
        
        if path.contains("controller") || path.contains("handler") {
            ServiceType::API
        } else if path.contains("model") || path.contains("entity") {
            ServiceType::Database
        } else if path.contains("service") {
            ServiceType::Service
        } else if path.contains("frontend") || path.contains("client") || file.language == "javascript" || file.language == "typescript" {
            ServiceType::Frontend
        } else if path.contains("util") || path.contains("helper") {
            ServiceType::Utility
        } else {
            ServiceType::Unknown
        }
    }
    
    fn resolve_dependency(&self, import: &str, services: &[Service]) -> Option<String> {
        // Simple dependency resolution - would be more sophisticated in production
        for service in services {
            if import.contains(&service.name.to_lowercase()) {
                return Some(service.name.clone());
            }
        }
        None
    }
    
    fn detect_entry_points(&self, repo: &Repository) -> Vec<String> {
        let mut entry_points = Vec::new();
        
        for file in &repo.files {
            let path = file.path.to_string_lossy();
            if path.ends_with("main.rs") || 
               path.ends_with("main.go") || 
               path.ends_with("main.py") || 
               path.ends_with("index.js") || 
               path.ends_with("app.js") {
                entry_points.push(path.to_string());
            }
        }
        
        entry_points
    }
}
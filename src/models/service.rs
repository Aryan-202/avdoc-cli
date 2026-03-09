use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ServiceType {
    API,
    Database,
    Service,
    Frontend,
    Utility,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub service_type: ServiceType,
    pub files: Vec<PathBuf>,
    pub dependencies: HashSet<String>,
    pub apis: Vec<APIEndpoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIEndpoint {
    pub path: String,
    pub method: String,
    pub handler: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Architecture {
    pub services: Vec<Service>,
    pub dependency_graph: HashMap<String, HashSet<String>>,
    pub entry_points: Vec<String>,
}
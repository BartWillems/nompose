use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Compose {
    pub version: String,
    pub services: HashMap<String, Service>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Service {
    pub image: String,
    pub environment: HashMap<String, String>,
    pub ports: Vec<String>,
}

// version: "3.7"
// services:
//   redis:
//     image: redis:latest
//     environment:
//       password: "secret"
//     ports:
//       - 8000:8000

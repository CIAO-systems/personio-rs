/*
 * Personnel Data
 *
 * API for reading and writing personnel data including data about attendances, absences, documents, etc
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DocumentEmployeeResponseObjectFirstName {
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "universal_id", skip_serializing_if = "Option::is_none")]
    pub universal_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}

impl DocumentEmployeeResponseObjectFirstName {
    pub fn new() -> DocumentEmployeeResponseObjectFirstName {
        DocumentEmployeeResponseObjectFirstName {
            label: None,
            value: None,
            r#type: None,
            universal_id: None,
            additional_properties: std::collections::HashMap::new(),
        }
    }
}


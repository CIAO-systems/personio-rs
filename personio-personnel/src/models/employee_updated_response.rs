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
pub struct EmployeeUpdatedResponse {
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<models::EmployeeUpdatedResponseData>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}

impl EmployeeUpdatedResponse {
    pub fn new() -> EmployeeUpdatedResponse {
        EmployeeUpdatedResponse {
            success: None,
            data: None,
            meta: None,
            additional_properties: std::collections::HashMap::new(),
        }
    }
}


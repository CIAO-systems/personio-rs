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
pub struct EmployeeUpdatedResponseData {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<f64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}

impl EmployeeUpdatedResponseData {
    pub fn new() -> EmployeeUpdatedResponseData {
        EmployeeUpdatedResponseData {
            id: None,
        }
    }
}


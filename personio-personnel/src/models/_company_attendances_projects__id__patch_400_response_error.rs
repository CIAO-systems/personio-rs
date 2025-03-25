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
pub struct CompanyAttendancesProjectsIdPatch400ResponseError {
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}

impl CompanyAttendancesProjectsIdPatch400ResponseError {
    pub fn new() -> CompanyAttendancesProjectsIdPatch400ResponseError {
        CompanyAttendancesProjectsIdPatch400ResponseError {
            active: None,
            additional_properties: std::collections::HashMap::new(),
        }
    }
}


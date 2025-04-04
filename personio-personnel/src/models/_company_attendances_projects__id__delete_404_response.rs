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
pub struct CompanyAttendancesProjectsIdDelete404Response {
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}

impl CompanyAttendancesProjectsIdDelete404Response {
    pub fn new() -> CompanyAttendancesProjectsIdDelete404Response {
        CompanyAttendancesProjectsIdDelete404Response {
            success: None,
            message: None,
            additional_properties: std::collections::HashMap::new(),
        }
    }
}


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
pub struct CompanyEmployeesPostRequestEmployeeCustomAttributes {
    /// Dynamic field, represented by unique id.
    #[serde(rename = "dynamic_{{ field uid }}", skip_serializing_if = "Option::is_none")]
    pub dynamic_left_curly_bracket_left_curly_bracket__field_uid_right_curly_bracket_right_curly_bracket: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}

impl CompanyEmployeesPostRequestEmployeeCustomAttributes {
    pub fn new() -> CompanyEmployeesPostRequestEmployeeCustomAttributes {
        CompanyEmployeesPostRequestEmployeeCustomAttributes {
            dynamic_left_curly_bracket_left_curly_bracket__field_uid_right_curly_bracket_right_curly_bracket: None,
        }
    }
}


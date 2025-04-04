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
pub struct CompanyTimeOffTypesGet200Response {
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<models::TimeOffTypeResource>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}

impl CompanyTimeOffTypesGet200Response {
    pub fn new() -> CompanyTimeOffTypesGet200Response {
        CompanyTimeOffTypesGet200Response {
            success: None,
            data: None,
            additional_properties: std::collections::HashMap::new(),
        }
    }
}


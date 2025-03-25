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
pub struct EmployeesResponse {
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "data")]
    pub data: Vec<models::EmployeesResponseAllOfData>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<models::EmployeesResponseAllOfMetadata>>,
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}

impl EmployeesResponse {
    pub fn new(success: bool, data: Vec<models::EmployeesResponseAllOfData>) -> EmployeesResponse {
        EmployeesResponse {
            success,
            data,
            metadata: None,
            offset: None,
            limit: None,
        }
    }
}


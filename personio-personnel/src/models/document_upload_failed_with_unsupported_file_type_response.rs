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
pub struct DocumentUploadFailedWithUnsupportedFileTypeResponse {
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<models::DocumentUploadFailedWithUnsupportedFileTypeResponseAllOfError>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}

impl DocumentUploadFailedWithUnsupportedFileTypeResponse {
    pub fn new() -> DocumentUploadFailedWithUnsupportedFileTypeResponse {
        DocumentUploadFailedWithUnsupportedFileTypeResponse {
            success: None,
            error: None,
            additional_properties: std::collections::HashMap::new(),
        }
    }
}


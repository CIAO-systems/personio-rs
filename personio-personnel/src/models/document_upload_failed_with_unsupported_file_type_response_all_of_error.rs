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
pub struct DocumentUploadFailedWithUnsupportedFileTypeResponseAllOfError {
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[serde(rename = "error_data", skip_serializing_if = "Option::is_none")]
    pub error_data: Option<Box<models::DocumentUploadFailedWithUnsupportedFileTypeResponseAllOfErrorErrorData>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}

impl DocumentUploadFailedWithUnsupportedFileTypeResponseAllOfError {
    pub fn new() -> DocumentUploadFailedWithUnsupportedFileTypeResponseAllOfError {
        DocumentUploadFailedWithUnsupportedFileTypeResponseAllOfError {
            message: None,
            code: None,
            error_data: None,
            additional_properties: std::collections::HashMap::new(),
        }
    }
}


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
pub struct CompanyAttendancesProjectsIdPatchRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Marks the availability of the project
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

impl CompanyAttendancesProjectsIdPatchRequest {
    pub fn new() -> CompanyAttendancesProjectsIdPatchRequest {
        CompanyAttendancesProjectsIdPatchRequest {
            name: None,
            active: None,
        }
    }
}


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
pub struct AttendanceUpdateRequest {
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Format: hh:mm
    #[serde(rename = "start_time", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// Format: hh:mm
    #[serde(rename = "end_time", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Option<String>>,
    #[serde(rename = "break", skip_serializing_if = "Option::is_none")]
    pub r#break: Option<i32>,
    #[serde(rename = "comment", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub comment: Option<Option<String>>,
    #[serde(rename = "project_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Option<i32>>,
    /// Optional, default value is true. If set to false, the approval status of the attendance period will be \"pending\" if an approval rule is set for the attendances type. The respective approval flow will be triggered.
    #[serde(rename = "skip_approval", skip_serializing_if = "Option::is_none")]
    pub skip_approval: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}

impl AttendanceUpdateRequest {
    pub fn new() -> AttendanceUpdateRequest {
        AttendanceUpdateRequest {
            date: None,
            start_time: None,
            end_time: None,
            r#break: None,
            comment: None,
            project_id: None,
            skip_approval: None,
            additional_properties: std::collections::HashMap::new(),
        }
    }
}


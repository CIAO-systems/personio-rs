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
pub struct AttendanceCreateRequestAttendancesInner {
    #[serde(rename = "employee")]
    pub employee: i32,
    #[serde(rename = "date")]
    pub date: String,
    /// Format: hh:mm
    #[serde(rename = "start_time")]
    pub start_time: String,
    /// Format: hh:mm
    #[serde(rename = "end_time", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Option<String>>,
    #[serde(rename = "break")]
    pub r#break: i32,
    #[serde(rename = "comment", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub comment: Option<Option<String>>,
    #[serde(rename = "project_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Option<i32>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}

impl AttendanceCreateRequestAttendancesInner {
    pub fn new(employee: i32, date: String, start_time: String, r#break: i32) -> AttendanceCreateRequestAttendancesInner {
        AttendanceCreateRequestAttendancesInner {
            employee,
            date,
            start_time,
            end_time: None,
            r#break,
            comment: None,
            project_id: None,
            additional_properties: std::collections::HashMap::new(),
        }
    }
}


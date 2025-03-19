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
pub struct DurationAttribute {
    #[serde(rename = "attribute_id", skip_serializing_if = "Option::is_none")]
    pub attribute_id: Option<String>,
    #[serde(rename = "data_type", skip_serializing_if = "Option::is_none")]
    pub data_type: Option<DataType>,
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "employee_id", skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<i32>,
}

impl DurationAttribute {
    pub fn new() -> DurationAttribute {
        DurationAttribute {
            attribute_id: None,
            data_type: None,
            duration: None,
            employee_id: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DataType {
    #[serde(rename = "DURATION")]
    Duration,
}

impl Default for DataType {
    fn default() -> DataType {
        Self::Duration
    }
}


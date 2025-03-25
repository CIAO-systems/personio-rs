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
pub struct CompensationAttribute {
    #[serde(rename = "attribute_id", skip_serializing_if = "Option::is_none")]
    pub attribute_id: Option<String>,
    #[serde(rename = "data_type", skip_serializing_if = "Option::is_none")]
    pub data_type: Option<DataType>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "currency_code", skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "currency_symbol", skip_serializing_if = "Option::is_none")]
    pub currency_symbol: Option<String>,
    #[serde(rename = "overtime_hours", skip_serializing_if = "Option::is_none")]
    pub overtime_hours: Option<String>,
    #[serde(rename = "bonus_type", skip_serializing_if = "Option::is_none")]
    pub bonus_type: Option<String>,
    #[serde(rename = "employee_id", skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<i32>,
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}

impl CompensationAttribute {
    pub fn new() -> CompensationAttribute {
        CompensationAttribute {
            attribute_id: None,
            data_type: None,
            amount: None,
            currency_code: None,
            currency_symbol: None,
            overtime_hours: None,
            bonus_type: None,
            employee_id: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DataType {
    #[serde(rename = "COMPENSATION")]
    Compensation,
}

impl Default for DataType {
    fn default() -> DataType {
        Self::Compensation
    }
}


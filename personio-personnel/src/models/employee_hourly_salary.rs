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
pub struct EmployeeHourlySalary {
    #[serde(rename = "label", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub label: Option<Option<serde_json::Value>>,
    #[serde(rename = "value", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub value: Option<Option<serde_json::Value>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::TypeEnum>,
    #[serde(rename = "universal_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub universal_id: Option<Option<serde_json::Value>>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}

impl EmployeeHourlySalary {
    pub fn new() -> EmployeeHourlySalary {
        EmployeeHourlySalary {
            label: None,
            value: None,
            r#type: None,
            universal_id: None,
            currency: None,
            additional_properties: std::collections::HashMap::new(),
        }
    }
}


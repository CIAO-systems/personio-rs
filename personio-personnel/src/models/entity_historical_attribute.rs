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
pub struct EntityHistoricalAttribute {
    #[serde(rename = "attribute_id", skip_serializing_if = "Option::is_none")]
    pub attribute_id: Option<String>,
    #[serde(rename = "entity_id", skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(rename = "data_type", skip_serializing_if = "Option::is_none")]
    pub data_type: Option<DataType>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "employee_id", skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<i32>,
    #[serde(rename = "effective_date", skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}

impl EntityHistoricalAttribute {
    pub fn new() -> EntityHistoricalAttribute {
        EntityHistoricalAttribute {
            attribute_id: None,
            entity_id: None,
            data_type: None,
            value: None,
            employee_id: None,
            effective_date: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DataType {
    #[serde(rename = "ENTITY")]
    Entity,
}

impl Default for DataType {
    fn default() -> DataType {
        Self::Entity
    }
}


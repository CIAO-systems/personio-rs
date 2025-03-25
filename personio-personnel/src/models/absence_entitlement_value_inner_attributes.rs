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
pub struct AbsenceEntitlementValueInnerAttributes {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<serde_json::Value>>,
    #[serde(rename = "entitlement", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub entitlement: Option<Option<serde_json::Value>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}

impl AbsenceEntitlementValueInnerAttributes {
    pub fn new() -> AbsenceEntitlementValueInnerAttributes {
        AbsenceEntitlementValueInnerAttributes {
            id: None,
            name: None,
            entitlement: None,
        }
    }
}


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
pub struct AbsenceEntitlement {
    #[serde(rename = "label", deserialize_with = "Option::deserialize")]
    pub label: Option<serde_json::Value>,
    #[serde(rename = "value")]
    pub value: Vec<models::AbsenceEntitlementValueInner>,
}

impl AbsenceEntitlement {
    pub fn new(label: Option<serde_json::Value>, value: Vec<models::AbsenceEntitlementValueInner>) -> AbsenceEntitlement {
        AbsenceEntitlement {
            label,
            value,
        }
    }
}


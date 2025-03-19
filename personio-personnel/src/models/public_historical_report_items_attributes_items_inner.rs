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
pub struct PublicHistoricalReportItemsAttributesItemsInner {
    #[serde(rename = "employee_id", skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<i32>,
    #[serde(rename = "historical_attributes", skip_serializing_if = "Option::is_none")]
    pub historical_attributes: Option<Vec<models::PublicHistoricalReportItemsAttributesItemsInnerHistoricalAttributesInner>>,
}

impl PublicHistoricalReportItemsAttributesItemsInner {
    pub fn new() -> PublicHistoricalReportItemsAttributesItemsInner {
        PublicHistoricalReportItemsAttributesItemsInner {
            employee_id: None,
            historical_attributes: None,
        }
    }
}


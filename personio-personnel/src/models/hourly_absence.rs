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
pub struct HourlyAbsence {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "measurement_unit", skip_serializing_if = "Option::is_none")]
    pub measurement_unit: Option<String>,
    /// Period effective duration in minutes
    #[serde(rename = "effective_duration", skip_serializing_if = "Option::is_none")]
    pub effective_duration: Option<i32>,
    #[serde(rename = "employee", skip_serializing_if = "Option::is_none")]
    pub employee: Option<Box<models::ShortEmployee>>,
    #[serde(rename = "absence_type", skip_serializing_if = "Option::is_none")]
    pub absence_type: Option<Box<models::AbsenceType>>,
    #[serde(rename = "certificate", skip_serializing_if = "Option::is_none")]
    pub certificate: Option<Box<models::HourlyAbsencePeriodResponseAttributesCertificate>>,
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(rename = "end", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub end: Option<Option<String>>,
    #[serde(rename = "half_day_start", skip_serializing_if = "Option::is_none")]
    pub half_day_start: Option<bool>,
    #[serde(rename = "half_day_end", skip_serializing_if = "Option::is_none")]
    pub half_day_end: Option<bool>,
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "origin", skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// ID of the employee who created the absence period.
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<i32>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "approved_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub approved_at: Option<Option<String>>,
    /// Breakdowns of effective duration by day of absence.
    #[serde(rename = "breakdowns", skip_serializing_if = "Option::is_none")]
    pub breakdowns: Option<Vec<models::AbsencePeriodBreakdown>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}

impl HourlyAbsence {
    pub fn new() -> HourlyAbsence {
        HourlyAbsence {
            id: None,
            measurement_unit: None,
            effective_duration: None,
            employee: None,
            absence_type: None,
            certificate: None,
            start: None,
            end: None,
            half_day_start: None,
            half_day_end: None,
            comment: None,
            origin: None,
            status: None,
            timezone: None,
            created_by: None,
            created_at: None,
            updated_at: None,
            approved_at: None,
            breakdowns: None,
            additional_properties: std::collections::HashMap::new(),
        }
    }
}


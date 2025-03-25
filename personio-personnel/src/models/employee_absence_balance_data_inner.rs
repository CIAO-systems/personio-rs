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
pub struct EmployeeAbsenceBalanceDataInner {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Effective balance, as of request date; not deducting any upcoming absence periods, but only what has been taken. If the absence period is currently ongoing, the effective balance only considers the part of the period until today. 
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none")]
    pub balance: Option<f64>,
    /// Available balance is what actually left to be planned (where upcoming absence periods are already deducted).
    #[serde(rename = "available_balance", skip_serializing_if = "Option::is_none")]
    pub available_balance: Option<f64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}

impl EmployeeAbsenceBalanceDataInner {
    pub fn new() -> EmployeeAbsenceBalanceDataInner {
        EmployeeAbsenceBalanceDataInner {
            id: None,
            name: None,
            category: None,
            balance: None,
            available_balance: None,
        }
    }
}


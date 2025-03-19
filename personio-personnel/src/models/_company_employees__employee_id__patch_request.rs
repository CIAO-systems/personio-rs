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
pub struct CompanyEmployeesEmployeeIdPatchRequest {
    #[serde(rename = "employee", skip_serializing_if = "Option::is_none")]
    pub employee: Option<Box<models::CompanyEmployeesEmployeeIdPatchRequestEmployee>>,
}

impl CompanyEmployeesEmployeeIdPatchRequest {
    pub fn new() -> CompanyEmployeesEmployeeIdPatchRequest {
        CompanyEmployeesEmployeeIdPatchRequest {
            employee: None,
        }
    }
}


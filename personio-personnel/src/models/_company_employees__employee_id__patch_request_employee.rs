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
pub struct CompanyEmployeesEmployeeIdPatchRequestEmployee {
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "preferred_name", skip_serializing_if = "Option::is_none")]
    pub preferred_name: Option<String>,
    #[serde(rename = "gender", skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// The subcompany employee belongs to. Should be predefined in Personio. Otherwise will be ignored with showing meta error in the response.
    #[serde(rename = "subcompany", skip_serializing_if = "Option::is_none")]
    pub subcompany: Option<String>,
    /// The department employee belongs to. Should be predefined in Personio. Otherwise will be ignored with showing meta error in the response.
    #[serde(rename = "department", skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    /// The office employee belongs to. Should be predefined in Personio. Otherwise will be ignored with showing meta error in the response.
    #[serde(rename = "office", skip_serializing_if = "Option::is_none")]
    pub office: Option<String>,
    /// Employee hire date. Format: \"yyyy-mm-dd\". Update of the `hire_date` will not update employee status on its own (for that you'll need to update the `status` field)
    #[serde(rename = "hire_date", skip_serializing_if = "Option::is_none")]
    pub hire_date: Option<String>,
    #[serde(rename = "weekly_working_hours", skip_serializing_if = "Option::is_none")]
    pub weekly_working_hours: Option<f64>,
    /// Status of the employee.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Employee ID of the Supervisor to be assigned. It needs to belong to a current existing employee and not the same as the one of the employee being updated, otherwise an error will be returned. If sent as null, will unset the employee's supervisor.
    #[serde(rename = "supervisor_id", skip_serializing_if = "Option::is_none")]
    pub supervisor_id: Option<f64>,
    #[serde(rename = "custom_attributes", skip_serializing_if = "Option::is_none")]
    pub custom_attributes: Option<Box<models::CompanyEmployeesPostRequestEmployeeCustomAttributes>>,
}

impl CompanyEmployeesEmployeeIdPatchRequestEmployee {
    pub fn new() -> CompanyEmployeesEmployeeIdPatchRequestEmployee {
        CompanyEmployeesEmployeeIdPatchRequestEmployee {
            first_name: None,
            last_name: None,
            preferred_name: None,
            gender: None,
            position: None,
            subcompany: None,
            department: None,
            office: None,
            hire_date: None,
            weekly_working_hours: None,
            status: None,
            supervisor_id: None,
            custom_attributes: None,
        }
    }
}


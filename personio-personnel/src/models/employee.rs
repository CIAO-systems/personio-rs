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
pub struct Employee {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Box<models::EmployeeId>>,
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<Box<models::EmployeeFirstName>>,
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<Box<models::EmployeeLastName>>,
    #[serde(rename = "preferred_name", skip_serializing_if = "Option::is_none")]
    pub preferred_name: Option<Box<models::EmployeePreferredName>>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<Box<models::EmployeeEmail>>,
    #[serde(rename = "gender", skip_serializing_if = "Option::is_none")]
    pub gender: Option<Box<models::EmployeeGender>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<models::EmployeeStatus>>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<Box<models::EmployeePosition>>,
    #[serde(rename = "supervisor", skip_serializing_if = "Option::is_none")]
    pub supervisor: Option<Box<models::Supervisor>>,
    #[serde(rename = "employment_type", skip_serializing_if = "Option::is_none")]
    pub employment_type: Option<Box<models::EmployeeEmploymentType>>,
    #[serde(rename = "weekly_working_hours", skip_serializing_if = "Option::is_none")]
    pub weekly_working_hours: Option<Box<models::EmployeeWeeklyWorkingHours>>,
    #[serde(rename = "hire_date", skip_serializing_if = "Option::is_none")]
    pub hire_date: Option<Box<models::EmployeeHireDate>>,
    #[serde(rename = "contract_end_date", skip_serializing_if = "Option::is_none")]
    pub contract_end_date: Option<Box<models::EmployeeContractEndDate>>,
    #[serde(rename = "termination_date", skip_serializing_if = "Option::is_none")]
    pub termination_date: Option<Box<models::EmployeeTerminationDate>>,
    #[serde(rename = "termination_type", skip_serializing_if = "Option::is_none")]
    pub termination_type: Option<Box<models::EmployeeTerminationType>>,
    #[serde(rename = "termination_reason", skip_serializing_if = "Option::is_none")]
    pub termination_reason: Option<Box<models::EmployeeTerminationReason>>,
    #[serde(rename = "probation_period_end", skip_serializing_if = "Option::is_none")]
    pub probation_period_end: Option<Box<models::EmployeeProbationPeriodEnd>>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Box<models::EmployeeCreatedAt>>,
    #[serde(rename = "last_modified_at", skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<Box<models::EmployeeLastModifiedAt>>,
    #[serde(rename = "subcompany", skip_serializing_if = "Option::is_none")]
    pub subcompany: Option<Box<models::Office>>,
    #[serde(rename = "office", skip_serializing_if = "Option::is_none")]
    pub office: Option<Box<models::Office>>,
    #[serde(rename = "department", skip_serializing_if = "Option::is_none")]
    pub department: Option<Box<models::Department>>,
    #[serde(rename = "cost_centers", skip_serializing_if = "Option::is_none")]
    pub cost_centers: Option<Box<models::CostCenters>>,
    #[serde(rename = "holiday_calendar", skip_serializing_if = "Option::is_none")]
    pub holiday_calendar: Option<Box<models::HolidayCalendar>>,
    #[serde(rename = "work_schedule", skip_serializing_if = "Option::is_none")]
    pub work_schedule: Option<Box<models::WorkSchedule>>,
    #[serde(rename = "absence_entitlement", skip_serializing_if = "Option::is_none")]
    pub absence_entitlement: Option<Box<models::AbsenceEntitlement>>,
    #[serde(rename = "team", skip_serializing_if = "Option::is_none")]
    pub team: Option<Box<models::Team>>,
    #[serde(rename = "fix_salary", skip_serializing_if = "Option::is_none")]
    pub fix_salary: Option<Box<models::EmployeeFixSalary>>,
    #[serde(rename = "fix_salary_interval", skip_serializing_if = "Option::is_none")]
    pub fix_salary_interval: Option<Box<models::EmployeeFixSalaryInterval>>,
    #[serde(rename = "hourly_salary", skip_serializing_if = "Option::is_none")]
    pub hourly_salary: Option<Box<models::EmployeeHourlySalary>>,
    #[serde(rename = "last_working_day", skip_serializing_if = "Option::is_none")]
    pub last_working_day: Option<Box<models::EmployeeLastWorkingDay>>,
    #[serde(rename = "profile_picture", skip_serializing_if = "Option::is_none")]
    pub profile_picture: Option<Box<models::EmployeeProfilePicture>>,
    #[serde(rename = "dynamic_21827", skip_serializing_if = "Option::is_none")]
    pub dynamic_21827: Option<Box<models::EmployeeDynamic21827>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}

impl Employee {
    pub fn new() -> Employee {
        Employee {
            id: None,
            first_name: None,
            last_name: None,
            preferred_name: None,
            email: None,
            gender: None,
            status: None,
            position: None,
            supervisor: None,
            employment_type: None,
            weekly_working_hours: None,
            hire_date: None,
            contract_end_date: None,
            termination_date: None,
            termination_type: None,
            termination_reason: None,
            probation_period_end: None,
            created_at: None,
            last_modified_at: None,
            subcompany: None,
            office: None,
            department: None,
            cost_centers: None,
            holiday_calendar: None,
            work_schedule: None,
            absence_entitlement: None,
            team: None,
            fix_salary: None,
            fix_salary_interval: None,
            hourly_salary: None,
            last_working_day: None,
            profile_picture: None,
            dynamic_21827: None,
            additional_properties: std::collections::HashMap::new(),
        }
    }
}


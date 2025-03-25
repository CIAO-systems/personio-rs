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
pub struct ShortEmployeeAttributes {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Box<models::ShortEmployeeAttributesId>>,
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<Box<models::ShortEmployeeAttributesFirstName>>,
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<Box<models::ShortEmployeeAttributesLastName>>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<Box<models::EmployeeEmail>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}

impl ShortEmployeeAttributes {
    pub fn new() -> ShortEmployeeAttributes {
        ShortEmployeeAttributes {
            id: None,
            first_name: None,
            last_name: None,
            email: None,
            additional_properties: std::collections::HashMap::new(),
        }
    }
}


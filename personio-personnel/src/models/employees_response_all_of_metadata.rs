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
pub struct EmployeesResponseAllOfMetadata {
    /// The total number of items matching the query filters.
    #[serde(rename = "total_elements")]
    pub total_elements: i32,
    /// The total number of pages distributing sets of items matching the query filters.
    #[serde(rename = "total_pages")]
    pub total_pages: i32,
    /// Current page number containing the returned set of items matching with the query filters. The page number range is from 0 to total_pages - 1
    #[serde(rename = "current_page")]
    pub current_page: i32,
}

impl EmployeesResponseAllOfMetadata {
    pub fn new(total_elements: i32, total_pages: i32, current_page: i32) -> EmployeesResponseAllOfMetadata {
        EmployeesResponseAllOfMetadata {
            total_elements,
            total_pages,
            current_page,
        }
    }
}


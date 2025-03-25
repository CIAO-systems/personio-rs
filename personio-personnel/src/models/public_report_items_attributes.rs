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
pub struct PublicReportItemsAttributes {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Report author
    #[serde(rename = "author_first_name", skip_serializing_if = "Option::is_none")]
    pub author_first_name: Option<String>,
    /// Report author
    #[serde(rename = "author_last_name", skip_serializing_if = "Option::is_none")]
    pub author_last_name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "end_date", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "data_refreshed_at", skip_serializing_if = "Option::is_none")]
    pub data_refreshed_at: Option<String>,
    #[serde(rename = "columns", skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<String>>,
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<models::PublicReportAttributesFiltersInner>>,
    #[serde(rename = "period_type", skip_serializing_if = "Option::is_none")]
    pub period_type: Option<PeriodType>,
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<models::PublicReportItemsAttributesItemsInner>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}

impl PublicReportItemsAttributes {
    pub fn new() -> PublicReportItemsAttributes {
        PublicReportItemsAttributes {
            id: None,
            name: None,
            description: None,
            author_first_name: None,
            author_last_name: None,
            r#type: None,
            status: None,
            start_date: None,
            end_date: None,
            created_at: None,
            updated_at: None,
            data_refreshed_at: None,
            columns: None,
            filters: None,
            period_type: None,
            items: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "point_in_time")]
    PointInTime,
}

impl Default for Type {
    fn default() -> Type {
        Self::PointInTime
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "up_to_date")]
    UpToDate,
    #[serde(rename = "updating")]
    Updating,
    #[serde(rename = "update_failed")]
    UpdateFailed,
}

impl Default for Status {
    fn default() -> Status {
        Self::UpToDate
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PeriodType {
    #[serde(rename = "fixed")]
    Fixed,
    #[serde(rename = "today")]
    Today,
    #[serde(rename = "yesterday")]
    Yesterday,
    #[serde(rename = "last_day_of_this_week")]
    LastDayOfThisWeek,
    #[serde(rename = "last_day_of_last_week")]
    LastDayOfLastWeek,
    #[serde(rename = "last_day_of_this_month")]
    LastDayOfThisMonth,
    #[serde(rename = "last_day_of_last_month")]
    LastDayOfLastMonth,
    #[serde(rename = "last_day_of_this_quarter")]
    LastDayOfThisQuarter,
    #[serde(rename = "last_day_of_last_quarter")]
    LastDayOfLastQuarter,
    #[serde(rename = "last_day_of_this_year")]
    LastDayOfThisYear,
    #[serde(rename = "last_day_of_last_year")]
    LastDayOfLastYear,
}

impl Default for PeriodType {
    fn default() -> PeriodType {
        Self::Fixed
    }
}


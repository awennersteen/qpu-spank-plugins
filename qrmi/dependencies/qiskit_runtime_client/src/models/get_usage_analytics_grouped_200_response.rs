/*
 * Qiskit Runtime API
 *
 * The Qiskit Runtime API description
 *
 * The version of the OpenAPI document: 0.21.2
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetUsageAnalyticsGrouped200Response {
    /// Data
    #[serde(rename = "data")]
    pub data: Vec<models::GetUsageAnalyticsGrouped200ResponseDataInner>,
}

impl GetUsageAnalyticsGrouped200Response {
    pub fn new(
        data: Vec<models::GetUsageAnalyticsGrouped200ResponseDataInner>,
    ) -> GetUsageAnalyticsGrouped200Response {
        GetUsageAnalyticsGrouped200Response { data }
    }
}

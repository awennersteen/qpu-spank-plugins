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

/// BackendsResponseV2 : Backends Response when `IBM-API-Version >= 2025-01-01`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackendsResponseV2 {
    /// A list of backends with their status
    #[serde(rename = "devices", skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<models::BackendsResponseV2DevicesInner>>,
}

impl BackendsResponseV2 {
    /// Backends Response when `IBM-API-Version >= 2025-01-01`
    pub fn new() -> BackendsResponseV2 {
        BackendsResponseV2 { devices: None }
    }
}

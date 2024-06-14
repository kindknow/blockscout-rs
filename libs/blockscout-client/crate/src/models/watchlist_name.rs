/*
 * BlockScout API
 *
 * API for BlockScout web app
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: you@your-company.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WatchlistName {
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "label")]
    pub label: String,
}

impl WatchlistName {
    pub fn new(display_name: String, label: String) -> WatchlistName {
        WatchlistName {
            display_name,
            label,
        }
    }
}
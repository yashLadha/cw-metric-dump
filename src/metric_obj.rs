use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricObj {
    pub name: String,
    pub namespace: String,
    pub stat: Option<String>,
    pub extended_stat: Option<String>,
    pub dimensions: Option<HashMap<String, String>>,
}

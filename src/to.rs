use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::common::Describe;

#[derive(Serialize, Deserialize, Debug)]
pub struct SqlxData {
    pub db: String,
    #[serde(flatten)]
    pub entries: HashMap<String, ToEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToEntry {
    pub describe: Describe,
    pub query: String,
}

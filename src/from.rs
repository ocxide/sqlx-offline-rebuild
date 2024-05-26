use serde::{Deserialize, Serialize};

use crate::common::Describe;

#[derive(Serialize, Deserialize)]
pub struct FromOld {
    pub query: String,
    pub describe: Describe,
    pub hash: String,
}

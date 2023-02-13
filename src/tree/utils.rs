use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CommaSeparated<T: Node> {
    pub inner: Vec<T>,
    pub commas: Vec<usize>, // `,`
}

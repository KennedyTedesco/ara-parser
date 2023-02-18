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

impl<T: Node + std::fmt::Display> std::fmt::Display for CommaSeparated<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (index, node) in self.inner.iter().enumerate() {
            write!(f, "{}", node)?;

            if index < self.inner.len() - 1 {
                write!(f, ", ")?;
            }
        }
        Ok(())
    }
}

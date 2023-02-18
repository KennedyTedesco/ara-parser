use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::definition::r#type::TypeDefinition;
use crate::tree::utils::CommaSeparated;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GenericGroupExpression {
    pub double_colon_less_than: usize,
    pub types: CommaSeparated<TypeDefinition>,
    pub greater_than: usize,
}

impl Node for GenericGroupExpression {
    fn initial_position(&self) -> usize {
        self.double_colon_less_than
    }

    fn final_position(&self) -> usize {
        self.greater_than + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        self.types.inner.iter().map(|t| t as &dyn Node).collect()
    }

    fn get_description(&self) -> String {
        "generic group expression".to_string()
    }
}

impl std::fmt::Display for GenericGroupExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "::")?;
        write!(f, "<")?;
        for (i, t) in self.types.inner.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", t)?;
        }
        write!(f, ">")?;
        Ok(())
    }
}

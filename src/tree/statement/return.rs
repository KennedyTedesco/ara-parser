use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::expression::Expression;
use crate::tree::token::Keyword;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ReturnStatement {
    Explicit {
        comments: CommentGroup,
        r#return: Keyword,
        expression: Option<Expression>,
        semicolon: usize,
    },
    Implicit {
        comments: CommentGroup,
        expression: Expression,
    },
}

impl Node for ReturnStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            Self::Explicit { comments, .. } => Some(comments),
            Self::Implicit { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            Self::Explicit { r#return, .. } => r#return.initial_position(),
            Self::Implicit { expression, .. } => expression.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Explicit { semicolon, .. } => semicolon + 1,
            Self::Implicit { expression, .. } => expression.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Explicit {
                r#return,
                expression,
                ..
            } => {
                if let Some(expression) = expression {
                    vec![r#return, expression]
                } else {
                    vec![r#return]
                }
            }
            Self::Implicit { expression, .. } => vec![expression],
        }
    }

    fn get_description(&self) -> String {
        match &self {
            Self::Explicit { .. } => "explicit return statement".to_string(),
            Self::Implicit { .. } => "implicit return statement".to_string(),
        }
    }

    fn is_describable(&self) -> bool {
        false
    }
}

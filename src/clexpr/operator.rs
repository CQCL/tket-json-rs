//! A tree of operators forming a classical expression.

#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::op::ClOp;

/// A node in a classical expression tree.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ClOperator {
    /// The operation to be performed.
    pub op: ClOp,
    /// The arguments to the operation.
    pub args: Vec<ClArgument>,
}

/// An argument to a classical expression operation.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "type", content = "input")]
pub enum ClArgument {
    /// A terminal argument.
    #[serde(rename = "term")]
    Terminal(ClTerminal),
    /// A sub-expression.
    #[serde(rename = "expr")]
    Expression(Box<ClOperator>),
}

/// A terminal argument in a classical expression operation.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "type", content = "term")]
pub enum ClTerminal {
    /// A terminal argument.
    #[serde(rename = "var")]
    Variable(ClVariable),
    /// A constant integer.
    #[serde(rename = "int")]
    Int(u64),
}

/// A variable terminal argument in a classical expression operation.
///
/// The indices refer to the local identifiers in the [`super::ClExpr`] structure.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Hash)]
#[non_exhaustive]
#[serde(tag = "type", content = "var")]
pub enum ClVariable {
    /// A register variable.
    #[serde(rename = "reg")]
    Register {
        /// The register identifier in [`super::ClExpr::reg_posn`].
        index: u32,
    },
    /// A constant bit.
    #[serde(rename = "bit")]
    Bit {
        /// The bit identifier in [`super::ClExpr::bit_posn`].
        index: u32,
    },
}

impl Default for ClArgument {
    fn default() -> Self {
        ClArgument::Terminal(ClTerminal::default())
    }
}

impl Default for ClTerminal {
    fn default() -> Self {
        ClTerminal::Int(0)
    }
}

impl Default for ClVariable {
    fn default() -> Self {
        ClVariable::Register { index: 0 }
    }
}

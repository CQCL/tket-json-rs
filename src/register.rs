//! Basic types for quantum and classical registers.

use derive_more::{Display, From};
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// An identifier for a bit or qubit in a register.
///
/// See [`Qubit`] and [`Bit`] for more specific types.
///
/// The first element is the name of the register, and the second element is a
/// multi-dimensional index into the register.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Display, Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
#[display("{_0}[{}]", _1.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(", "))]
pub struct ElementId(pub String, pub Vec<i64>);

/// An identifier for a qubit in a register.
///
/// See [`ElementId`] for the concrete generic index.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Display, Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash, From)]
#[display("{id}")]
#[serde(transparent)]
pub struct Qubit {
    /// Identifier for the qubit.
    pub id: ElementId,
}

/// An identifier for a bit in a [`BitRegister`].
///
/// See [`ElementId`] for the concrete generic index.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Display, Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash, From)]
#[display("{id}")]
#[serde(transparent)]
pub struct Bit {
    /// Identifier for the bit.
    pub id: ElementId,
}

/// A classical bit register.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BitRegister {
    /// Name of the bit register.
    pub name: String,
    /// Number of bits in the register.
    pub size: u32,
}

/// A vector of booleans.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct Bitstring {
    /// Vector of booleans.
    pub vec: Vec<bool>,
}

impl From<Qubit> for ElementId {
    fn from(q: Qubit) -> Self {
        q.id
    }
}

impl From<Bit> for ElementId {
    fn from(b: Bit) -> Self {
        b.id
    }
}

//! Classical expressions

pub mod op;
pub mod operator;

use operator::ClOperator;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::de::SeqAccess;
use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize};

/// Data encoding a classical expression.
///
/// A classical expression operates over multi-bit registers and/or individual bits,
/// each identified by an index local to the expression.
///
/// This is included in a [`Operation`] when the operation is a [`OpType::ClExpr`].
///
///   [`Operation`]: crate::circuit_json::Operation
///   [`OpType::ClExpr`]: crate::optype::OpType::ClExpr
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ClExpr {
    /// Mapping between bit variables in the expression and the position of the
    /// corresponding bit in the `args` list.
    pub bit_posn: Vec<(u32, u32)>,
    /// The encoded expression.
    ///
    /// This expression may only refer to bits and registers by their id either
    /// in [`Self::bit_posn`] or [`Self::reg_posn`], respectively.
    pub expr: ClOperator,
    /// A list of registers defined over the input bits, with a local
    /// identifier.
    ///
    /// `expr` may contain references to these registers by their
    /// [`InputClRegister::index`].
    pub reg_posn: Vec<InputClRegister>,
    /// The output bits of the expression.
    ///
    /// This is a list of bit indices in [`ClExpr::bit_posn`].
    pub output_posn: ClRegisterBits,
}

/// An input register for a classical expression.
///
/// Contains the input index as well as the bits that are part of the register.
///
/// Serialized as a list with two elements: the index and the bits.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct InputClRegister {
    /// The identifier for this register variable in the [`ClExpr::expr`] expression.
    pub index: u32,
    /// The sequence of positions of bits comprising the register variable.
    ///
    /// The indexes in this sequence refer to the bits in the [`ClExpr::bit_posn`] list.
    pub bits: ClRegisterBits,
}

/// The sequence of positions of bits in the output.
///
/// The indices here refer to the bit identifiers in the operation's [`ClExpr::bit_posn`].
///
/// Registers are little-endian, so the first bit is the least significant.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ClRegisterBits(pub Vec<u32>);

impl Serialize for InputClRegister {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.index)?;
        seq.serialize_element(&self.bits)?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for InputClRegister {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;

        impl<'de_vis> serde::de::Visitor<'de_vis> for Visitor {
            type Value = InputClRegister;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a list of two elements: the index and the bits")
            }

            fn visit_seq<A: SeqAccess<'de_vis>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let index = seq
                    .next_element::<u32>()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let bits = seq
                    .next_element::<ClRegisterBits>()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                Ok(InputClRegister { index, bits })
            }
        }

        deserializer.deserialize_seq(Visitor)
    }
}

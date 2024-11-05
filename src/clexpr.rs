//! Classical expressions

pub mod op;
pub mod operator;

use operator::ClOperator;
use serde::de::SeqAccess;
use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize};

/// Data encoding a classical expression.
///
/// A classical expression operates over multi-bit registers and/or individual bits,
/// which are identified here by their individual bit positions.
///
/// This is included in a [`Operation`] when the operation is a [`OpType::ClExpr`].
///
///   [`Operation`]: crate::circuit_json::Operation
///   [`OpType::ClExpr`]: crate::optype::OpType::ClExpr
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ClExpr {
    /// TODO: ???
    pub bit_posn: Vec<u32>,
    /// The encoded expression.
    pub expr: ClOperator,
    /// The input bits of the expression.
    pub reg_posn: Vec<InputClRegister>,
    /// The output bits of the expression.
    pub output_posn: ClRegisterBits,
}

/// An input register for a classical expression.
///
/// Contains the input index as well as the bits that are part of the register.
///
/// Serialized as a list with two elements: the index and the bits.
#[derive(Debug, Default, PartialEq, Clone)]
pub struct InputClRegister {
    /// The index of the register variable in the expression.
    pub index: u32,
    /// The sequence of positions of bits comprising the register variable.
    pub bits: ClRegisterBits,
}

/// The sequence of positions of bits in the output.
///
/// Registers are little-endian, so the first bit is the least significant.
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

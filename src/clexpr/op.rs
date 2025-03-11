//! Classical expression operations.

#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::EnumString;

/// List of supported classical expressions.
///
/// Corresponds to `pytket.circuit.ClOp`.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq, Eq, Hash, EnumString)]
#[non_exhaustive]
pub enum ClOp {
    /// Invalid operation
    #[default]
    INVALID,

    /// Bitwise AND
    BitAnd,
    /// Bitwise OR
    BitOr,
    /// Bitwise XOR
    BitXor,
    /// Bitwise equality
    BitEq,
    /// Bitwise inequality
    BitNeq,
    /// Bitwise NOT
    BitNot,
    /// Constant zero bit
    BitZero,
    /// Constant one bit
    BitOne,

    /// Registerwise AND
    RegAnd,
    /// Registerwise OR
    RegOr,
    /// Registerwise XOR
    RegXor,
    /// Registerwise equality
    RegEq,
    /// Registerwise inequality
    RegNeq,
    /// Registerwise NOT
    RegNot,
    /// Constant all-zeros register
    RegZero,
    /// Constant all-ones register
    RegOne,
    /// Integer less-than comparison
    RegLt,
    /// Integer greater-than comparison
    RegGt,
    /// Integer less-than-or-equal comparison
    RegLeq,
    /// Integer greater-than-or-equal comparison
    RegGeq,
    /// Integer addition
    RegAdd,
    /// Integer subtraction
    RegSub,
    /// Integer multiplication
    RegMul,
    /// Integer division
    RegDiv,
    /// Integer exponentiation
    RegPow,
    /// Left shift
    RegLsh,
    /// Right shift
    RegRsh,
    /// Integer negation
    RegNeg,
}

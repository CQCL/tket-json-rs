//! Contains structs for serializing and deserializing TKET circuits to and from
//! JSON.

use crate::opbox::OpBox;
use crate::optype::OpType;
use serde::{Deserialize, Serialize};

/// A register of locations sharing the same name.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Register(pub String, pub Vec<i64>);

/// A gate defined by a circuit.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CompositeGate {
    // List of Symbols
    args: Vec<String>,
    definition: Box<SerialCircuit>,
    name: String,
}

/// A classical bit register.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BitRegister {
    name: String,
    size: u32,
}

/// The units used in a [`ClassicalExp`].
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ClassicalExpUnit {
    /// Unsigned 32-bit integer.
    U32(u32),
    /// Register of locations.
    Register(Register),
    /// Register of bits.
    BitRegister(BitRegister),
    /// A nested classical expression.
    ClassicalExpUnit(ClassicalExp),
}

/// A box for holding classical expressions on Bits.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClassicalExp {
    args: Vec<ClassicalExpUnit>,
    op: String,
}

/// Decorates another op, adding a QASM-style classical condition.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Conditional {
    op: Box<Operation>,
    width: u32,
    value: u32,
}

/// Serializable operation descriptor.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Operation<P = String> {
    /// The type of operation.
    #[serde(rename = "type")]
    pub op_type: OpType,
    /// Number of input and output qubits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_qb: Option<u32>,
    /// Expressions for the parameters of the operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<P>>,
    /// Internal box for the operation.
    #[serde(rename = "box")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_box: Option<OpBox>,
    /// The pre-computed signature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<Vec<String>>,
    /// A QASM-style classical condition for the operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional: Option<Conditional>,
}

/// Operation applied in a circuit, with defined arguments.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Command<P = String> {
    /// The operation to be applied.
    pub op: Operation<P>,
    /// The arguments to the operation.
    pub args: Vec<Register>,
    /// Operation group identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opgroup: Option<String>,
}

/// A permutation of the elements of a register.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Permutation(pub Register, pub Register);

/// Pytket canonical serialized circuit
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SerialCircuit<P = String> {
    /// The name of the circuit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The global phase, as a symengine expression.
    pub phase: P,
    /// List of commands in the circuit.
    pub commands: Vec<Command<P>>,
    /// Input qubit registers.
    pub qubits: Vec<Register>,
    /// Input bit registers.
    pub bits: Vec<Register>,
    /// Implicit permutation of the output qubits.
    pub implicit_permutation: Vec<Permutation>,
}

impl<P> Operation<P> {
    /// Returns a default-initialized Operation with the given type.
    ///
    /// For optypes that require additional parameters, this may generate
    /// invalid operations.
    pub fn from_optype(op_type: OpType) -> Self {
        Self {
            op_type,
            n_qb: None,
            params: None,
            op_box: None,
            signature: None,
            conditional: None,
        }
    }
}

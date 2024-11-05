//! Contains structs for serializing and deserializing TKET circuits to and from
//! JSON.

use crate::clexpr::ClExpr;
use crate::opbox::OpBox;
use crate::optype::OpType;
use serde::{Deserialize, Serialize};

/// A register of locations sharing the same name.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Register(pub String, pub Vec<i64>);

/// A gate defined by a circuit.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CompositeGate {
    /// Name of the composite gate.
    pub name: String,
    /// Expressions corresponding to parameter values of the composite gate, if it has parameters.
    pub args: Vec<String>,
    /// The circuit defining the gate.
    pub definition: Box<SerialCircuit>,
}

/// A classical bit register.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BitRegister {
    /// Name of the bit register.
    pub name: String,
    /// Number of bits in the register.
    pub size: u32,
}

/// A vector of booleans.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct Bitstring {
    /// Vector of booleans.
    pub vec: Vec<bool>,
}

/// A 2D matrix.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct Matrix<T = f64> {
    /// A 2D vector of complex numbers.
    pub data: Vec<Vec<(T, T)>>,
}

/// The units used in a [`ClassicalExp`].
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(untagged)]
#[non_exhaustive]
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
    /// Arguments to the expression.
    pub args: Vec<ClassicalExpUnit>,
    /// The expression.
    pub op: String,
}

/// Decorates another op, adding a QASM-style classical condition.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Conditional {
    /// Internal operation to conditionally apply.
    pub op: Box<Operation>,
    /// Number of bits in the classical condition.
    pub width: u32,
    /// Value to compare against.
    pub value: u32,
}

/// Additional fields for classical operations,
/// which only act on Bits classically.
//
// Note: The order of the variants here is important.
// Serde will return the first matching variant when deserializing,
// so CopyBits and SetBits must come after other variants that
// define `values` and `n_i`.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Classical {
    /// Multi-bit operation.
    MultiBit {
        /// The inner operation.
        op: Box<Operation>,
        /// Multiplier on underlying op for MultiBitOp.
        n: u32,
    },
    /// A range predicate.
    RangePredicate {
        /// Number of pure input wires to the RangePredicate.
        n_i: u32,
        /// The inclusive minimum of the RangePredicate.
        lower: u64,
        /// The inclusive maximum of the RangePredicate.
        upper: u64,
    },
    /// ExplicitModifierOp/ExplicitPredicateOp.
    Explicit {
        /// Number of pure input wires to the ExplicitModifierOp/ExplicitPredicateOp.
        n_i: u32,
        /// Name of classical ExplicitModifierOp/ExplicitPredicateOp (e.g. AND).
        name: String,
        /// Truth table of ExplicitModifierOp/ExplicitPredicateOp.
        values: Vec<bool>,
    },
    /// ClassicalTransformOp
    ClassicalTransform {
        /// Number of input/output wires.
        n_io: u32,
        /// Name of classical ClassicalTransformOp (e.g. ClassicalCX).
        name: String,
        /// Truth table of ClassicalTransformOp.
        values: Vec<u32>,
    },
    /// CopyBitsOp.
    CopyBits {
        /// Number of input wires to the CopyBitsOp.
        n_i: u32,
    },
    /// SetBitsOp.
    SetBits {
        /// List of bools that SetBitsOp sets bits to.
        values: Vec<bool>,
    },
}

/// Additional fields for Wasm operations.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Wasm {
    n: u64,
    ww_n: u64,
    width_i_parameter: Vec<u64>,
    width_o_parameter: Vec<u64>,
    func_name: String,
    wasm_file_uid: String,
}

/// Serializable operation descriptor.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Operation<P = String> {
    /// The type of operation.
    #[serde(rename = "type")]
    pub op_type: OpType,
    /// Number of input and output qubits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_qb: Option<u32>,
    /// Additional string stored in the op
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// Expressions for the parameters of the operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<P>>,
    /// Internal box for the operation.
    #[serde(rename = "box")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_box: Option<OpBox>,
    /// Classical expression.
    ///
    /// Required if the operation is of type [`OpType::ClExpr`].
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "expr")]
    pub classical_expr: Option<ClExpr>,
    /// The pre-computed signature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<Vec<String>>,
    /// A QASM-style classical condition for the operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional: Option<Conditional>,
    /// Data for commands which only act on Bits classically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classical: Option<Box<Classical>>,
    /// Data for commands which apply WASM operations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wasm: Option<Box<Wasm>>,
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

/// A classic basis state permutation.
/// Used when defining Toffoli boxes.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct Permutation(pub Vec<(Vec<bool>, Vec<bool>)>);

/// An implicit permutation of the elements of a register.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplicitPermutation(pub Register, pub Register);

/// Pytket canonical serialized circuit
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
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
    pub implicit_permutation: Vec<ImplicitPermutation>,
    /// Number of wasm wires in the circuit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_ws: Option<u64>,
    /// A list of qubits initialized at the start of the circuit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_qubits: Option<Vec<Register>>,
    /// A list of qubits discarded at the end of the circuit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discarded_qubits: Option<Vec<Register>>,
}

impl<P> Default for Operation<P> {
    fn default() -> Self {
        Self {
            op_type: Default::default(),
            n_qb: None,
            data: None,
            params: None,
            op_box: None,
            classical_expr: None,
            signature: None,
            conditional: None,
            classical: None,
            wasm: None,
        }
    }
}

impl<P: Default> Default for SerialCircuit<P> {
    fn default() -> Self {
        Self {
            name: None,
            phase: Default::default(),
            commands: Default::default(),
            qubits: Default::default(),
            bits: Default::default(),
            implicit_permutation: Default::default(),
            number_of_ws: None,
            created_qubits: None,
            discarded_qubits: None,
        }
    }
}

impl<P> Operation<P> {
    /// Returns a default-initialized Operation with the given type.
    ///
    /// For optypes that require additional parameters, this may generate
    /// invalid operations.
    pub fn from_optype(op_type: OpType) -> Self {
        Self {
            op_type,
            ..Operation::default()
        }
    }

    /// Applies a function over the parameters of the operation.
    ///
    /// Returns a new Operation with the same data, but with a new generic
    /// type for the parameters.
    pub fn map_params<Q>(self, f: impl FnMut(P) -> Q) -> Operation<Q> {
        Operation {
            op_type: self.op_type,
            n_qb: self.n_qb,
            data: self.data,
            params: self
                .params
                .map(|params| params.into_iter().map(f).collect()),
            op_box: self.op_box,
            classical_expr: self.classical_expr,
            signature: self.signature,
            conditional: self.conditional,
            classical: self.classical,
            wasm: self.wasm,
        }
    }
}

impl<P> Command<P> {
    /// Applies a function over the parameters of the command.
    ///
    /// Returns a new Command with the same data, but with a new generic type
    /// for the parameters.
    pub fn map_params<Q>(self, f: impl FnMut(P) -> Q) -> Command<Q> {
        Command {
            op: self.op.map_params(f),
            args: self.args,
            opgroup: self.opgroup,
        }
    }
}

impl<P> SerialCircuit<P> {
    /// Initialize a new SerialCircuit with the given name and phase.
    pub fn new(name: Option<String>, phase: P) -> Self {
        Self {
            name,
            phase,
            commands: Vec::new(),
            qubits: Vec::new(),
            bits: Vec::new(),
            implicit_permutation: Vec::new(),
            number_of_ws: None,
            created_qubits: None,
            discarded_qubits: None,
        }
    }

    /// Applies a function over the parameters of the circuit.
    ///
    /// Returns a new SerialCircuit with the same data, but with a new generic
    /// type for the parameters.
    pub fn map_params<Q>(self, mut f: impl FnMut(P) -> Q) -> SerialCircuit<Q> {
        let phase = f(self.phase);
        let commands = self
            .commands
            .into_iter()
            .map(|c| c.map_params(&mut f))
            .collect();
        SerialCircuit {
            name: self.name,
            phase,
            commands,
            qubits: self.qubits,
            bits: self.bits,
            implicit_permutation: self.implicit_permutation,
            number_of_ws: self.number_of_ws,
            created_qubits: self.created_qubits,
            discarded_qubits: self.discarded_qubits,
        }
    }
}

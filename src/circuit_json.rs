//! Contains structs for serializing and deserializing TKET circuits to and from
//! JSON.

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

/// A simple struct for Pauli strings with +/- phase, used to represent Pauli
/// strings in a stabiliser subgroup.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PauliStabiliser {
    coeff: bool,
    string: Vec<String>,
}

/// Unique identifier for an [`OpBox`].
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BoxID(uuid::Uuid);

/// Box for an operation, the enum variant names come from the names
/// of the C++ operations and are renamed if the string corresponding
/// to the operation is differently named when serializing.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "type")]
#[allow(missing_docs)]
#[non_exhaustive]
pub enum OpBox {
    /// Operation defined as a circuit.
    CircBox {
        id: BoxID,
        /// The circuit defining the operation.
        circuit: SerialCircuit,
    },
    /// One-qubit operation defined as a unitary matrix.
    Unitary1qBox {
        id: BoxID,
        /// 2x2 matrix of complex numbers
        matrix: [[(f32, f32); 2]; 2],
    },
    /// Two-qubit operation defined as a unitary matrix.
    Unitary2qBox {
        id: BoxID,
        /// 4x4 matrix of complex numbers
        matrix: [[(f32, f32); 4]; 4],
    },
    /// Three-qubit operation defined as a unitary matrix.
    Unitary3qBox {
        id: BoxID,
        /// 8x8 matrix of complex numbers
        matrix: Box<[[(f32, f32); 8]; 8]>,
    },
    /// Two-qubit operation defined in terms of a hermitian matrix and a phase.
    ExpBox {
        id: BoxID,
        /// 4x4 matrix of complex numbers
        matrix: [[(f32, f32); 4]; 4],
        /// Phase of the operation.
        phase: f64,
    },
    /// Operation defined as the exponential of a tensor of Pauli operators.
    PauliExpBox {
        id: BoxID,
        /// List of Pauli operators.
        paulis: Vec<String>,
        /// Symengine expression.
        phase: String,
        /// Config param for decomposition of Pauli exponentials.
        #[serde(default)]
        cx_config: String,
    },
    /// Operation defined as a pair of exponential of a tensor of Pauli operators.
    PauliExpPairBox {
        id: BoxID,
        /// List of List of Pauli operators.
        paulis: Vec<Vec<String>>,
        /// List of Symengine expressions.
        phase: Vec<String>,
        /// Config param for decomposition of Pauli exponentials.
        cx_config: String,
    },
    /// Operation defined as a set of commuting exponentials of a tensor of Pauli operators.
    PauliExpCommutingSetBox {
        id: BoxID,
        /// List of List of Pauli operators.
        paulis: Vec<(Vec<String>, String)>,
        /// List of Symengine expressions.
        pauli_gadgets: Vec<(Vec<String>, String)>,
        /// Config param for decomposition of Pauli exponentials.
        cx_config: String,
    },
    /// An operation capable of representing arbitrary Circuits made up of CNOT
    /// and RZ, as a PhasePolynomial plus a boolean matrix representing an
    /// additional linear transformation.
    PhasePolyBox {
        id: BoxID,
        /// Number of qubits.
        n_qubits: u32,
        qubit_indices: Vec<(Register, u32)>,
    },
    /// A user-defined assertion specified by a list of Pauli stabilisers.
    StabiliserAssertionBox {
        id: BoxID,
        stabilisers: Vec<PauliStabiliser>,
    },
    /// A user-defined assertion specified by a 2x2, 4x4, or 8x8 projector matrix.
    ProjectorAssertionBox {
        id: BoxID,
        matrix: Vec<Vec<(f32, f32)>>,
    },
    /// A user-defined gate defined by a parametrized Circuit.
    Composite {
        id: BoxID,
        gate: CompositeGate,
        // Vec of Symengine Expr
        params: Vec<String>,
    },
    /// Wraps another quantum op, adding control qubits.
    QControlBox {
        id: BoxID,
        /// Number of control qubits.
        n_controls: u32,
        /// The operation to be controlled.
        op: Box<Operation>,
        /// The state of the control.
        #[serde(default)]
        control_state: u32,
    },
    /// Holding box for abstract expressions on Bits.
    ClassicalExpBox {
        id: BoxID,
        n_i: u32,
        n_io: u32,
        n_o: u32,
        exp: ClassicalExp,
    },
    /// A user-defined multiplexor specified by a map from bitstrings to Operations.
    MultiplexorBox {
        id: BoxID,
        op_map: Vec<(Vec<bool>, Operation)>,
    },
    /// A user-defined multiplexed rotation gate specified by a map from
    /// bitstrings to Operations.
    MultiplexedRotationBox {
        id: BoxID,
        op_map: Vec<(Vec<bool>, Operation)>,
    },
    /// A user-defined multiplexed rotation gate specified by a map from
    /// bitstrings to Operations.
    MultiplexedU2Box {
        id: BoxID,
        op_map: Vec<(Vec<bool>, Operation)>,
        #[serde(default = "default_impl_diag")]
        impl_diag: bool,
    },
    /// An operation that constructs a circuit to implement the specified
    /// permutation of classical basis states.
    ToffoliBox {
        id: BoxID,
        /// The classical basis state permutation.
        permutation: Permutation,
        // Synthesis strategy. See [`ToffoliBoxSynthStrat`].
        strat: ToffoliBoxSynthStrat,
        // The rotation axis of the multiplexors used in the decomposition. Can
        // be either `Rx` or `Ry`. Only applicable to the
        // [`ToffoliBoxSynthStrat::Matching`] strategy. Default to `Ry`.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        rotation_axis: Option<OpType>,
    },
}

fn default_impl_diag() -> bool {
    true
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

/// Strategies for synthesising ToffoliBoxes.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum ToffoliBoxSynthStrat {
    /// Use multiplexors to perform parallel swaps on hypercubes.
    Matching,
    /// Use CnX gates to perform transpositions.
    Cycle,
}

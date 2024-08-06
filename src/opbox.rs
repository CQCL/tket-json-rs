//! Data definition for box operations.
//!
//! These definitions correspond to the operations' `box` attribute in the
//! [`circuit_v1`](https://github.com/CQCL/tket/blob/develop/schemas/circuit_v1.json)
//! schema.

use std::collections::HashMap;

use crate::circuit_json::{
    Bitstring, ClassicalExp, CompositeGate, Matrix, Operation, Permutation, Register, SerialCircuit,
};
use crate::optype::OpType;
use serde::{Deserialize, Serialize};

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
        matrix: [[(f64, f64); 2]; 2],
    },
    /// Two-qubit operation defined as a unitary matrix.
    Unitary2qBox {
        id: BoxID,
        /// 4x4 matrix of complex numbers
        matrix: [[(f64, f64); 4]; 4],
    },
    /// Three-qubit operation defined as a unitary matrix.
    Unitary3qBox {
        id: BoxID,
        /// 8x8 matrix of complex numbers
        matrix: Box<[[(f64, f64); 8]; 8]>,
    },
    /// Two-qubit operation defined in terms of a hermitian matrix and a phase.
    ExpBox {
        id: BoxID,
        /// 4x4 matrix of complex numbers
        matrix: [[(f64, f64); 4]; 4],
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
    /// A pair of (not necessarily commuting) Pauli exponentials performed in sequence.
    PauliExpPairBox {
        id: BoxID,
        /// List of List of Pauli operators.
        paulis_pair: Vec<Vec<String>>,
        /// List of Symengine expressions.
        phase_pair: Vec<String>,
        /// Config param for decomposition of Pauli exponentials.
        cx_config: String,
    },
    /// Operation defined as a set of commuting exponentials of a tensor of Pauli operators.
    PauliExpCommutingSetBox {
        id: BoxID,
        /// List of Symengine expressions.
        pauli_gadgets: Vec<(Vec<String>, String)>,
        /// Config param for decomposition of Pauli exponentials.
        cx_config: String,
    },
    /// An unordered collection of Pauli exponentials that can be synthesised in
    /// any order, causing a change in the unitary operation. Synthesis order
    /// depends on the synthesis strategy chosen only.
    TermSequenceBox {
        id: BoxID,
        /// List of Symengine expressions.
        pauli_gadgets: Vec<(Vec<String>, String)>,
        /// Synthesis strategy. See [`PauliSynthStrat`].
        #[serde(default)]
        synth_strategy: PauliSynthStrat,
        /// Partition strategy. See [`PauliPartitionStrat`].
        #[serde(default)]
        partition_strategy: PauliPartitionStrat,
        /// Graph colouring method. See [`GraphColourMethod`].
        #[serde(default)]
        graph_colouring: GraphColourMethod,
        /// Configurations for CXs upon decompose phase gadgets.
        #[serde(default)]
        cx_config: CXConfigType,
    },
    /// An operation capable of representing arbitrary Circuits made up of CNOT
    /// and RZ, as a PhasePolynomial plus a boolean matrix representing an
    /// additional linear transformation.
    PhasePolyBox {
        id: BoxID,
        /// Number of qubits.
        n_qubits: u32,
        /// Map from qubits to inputs.
        qubit_indices: Vec<(Register, u32)>,
        /// The phase polynomial definition.
        /// Represented by a map from bitstring to expression of coefficient.
        phase_polynomial: Vec<Vec<(Bitstring, String)>>,
        /// The additional linear transformation.
        linear_transformation: Matrix,
    },
    /// A user-defined assertion specified by a list of Pauli stabilisers.
    StabiliserAssertionBox {
        id: BoxID,
        stabilisers: Vec<PauliStabiliser>,
    },
    /// A user-defined assertion specified by a 2x2, 4x4, or 8x8 projector matrix.
    ProjectorAssertionBox { id: BoxID, matrix: Matrix },
    /// A user-defined gate defined by a parametrised Circuit.
    CustomGate {
        id: BoxID,
        /// The gate defined as a circuit.
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
    /// Binary matrix form of a stabilizer tableau for unitary Clifford circuits.
    UnitaryTableauBox {
        id: BoxID,
        /// The tableau.
        tab: UnitaryTableau,
    },
    /// A user-defined multiplexor specified by a map from bitstrings to Operations.
    MultiplexorBox {
        id: BoxID,
        op_map: Vec<(Bitstring, Operation)>,
    },
    /// A user-defined multiplexed rotation gate specified by a map from
    /// bitstrings to Operations.
    MultiplexedRotationBox {
        id: BoxID,
        op_map: Vec<(Bitstring, Operation)>,
    },
    /// A user-defined multiplexed rotation gate specified by a map from
    /// bitstrings to Operations.
    MultiplexedU2Box {
        id: BoxID,
        op_map: Vec<(Bitstring, Operation)>,
        #[serde(default = "default_true")]
        impl_diag: bool,
    },
    /// A user-defined multiplexed tensor product of U2 gates specified by a map
    /// from bitstrings to lists of Op or a list of bitstring-list(Op s) pairs.
    MultiplexedTensoredU2Box {
        id: BoxID,
        op_map: Vec<(Bitstring, Operation)>,
    },
    /// An operation that constructs a circuit to implement the specified
    /// permutation of classical basis states.
    ToffoliBox {
        id: BoxID,
        /// The classical basis state permutation.
        permutation: Vec<(Vec<bool>, Vec<bool>)>,
        // Synthesis strategy. See [`ToffoliBoxSynthStrat`].
        strat: ToffoliBoxSynthStrat,
        // The rotation axis of the multiplexors used in the decomposition. Can
        // be either `Rx` or `Ry`. Only applicable to the
        // [`ToffoliBoxSynthStrat::Matching`] strategy. Default to `Ry`.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        rotation_axis: Option<OpType>,
    },
    /// An operation composed of ‘action’, ‘compute’ and ‘uncompute’ circuits
    ConjugationBox {
        id: BoxID,
        /// Reversible computation.
        compute: Box<Operation>,
        /// Internal operation to be applied.
        action: Box<Operation>,
        /// Reverse uncomputation.
        #[serde(default)]
        uncompute: Option<Box<Operation>>,
    },
    /// A placeholder operation that holds resource data. This box type cannot
    /// be decomposed into a circuit. It only serves to record resource data for
    /// a region of a circuit: for example, upper and lower bounds on gate
    /// counts and depth. A circuit containing such a box cannot be executed.
    DummyBox {
        id: BoxID,
        /// Number of qubits.
        n_qubits: u32,
        /// Number of bits.
        n_bits: u32,
        /// Dummy resource data.
        resource_data: ResourceData,
    },
    /// A box for preparing quantum states using multiplexed-Ry and multiplexed-Rz gates.
    StatePreparationBox {
        id: BoxID,
        /// Normalised statevector of complex numbers
        statevector: Matrix,
        /// Whether to implement the dagger of the state preparation circuit, default to false.
        #[serde(default)]
        is_inverse: bool,
        /// Whether to explicitly set the state to zero initially (by default
        /// the initial zero state is assumed and no explicit reset is applied).
        #[serde(default)]
        with_initial_reset: bool,
    },
    /// A box for synthesising a diagonal unitary matrix into a sequence of multiplexed-Rz gates.
    DiagonalBox {
        id: BoxID,
        /// Diagonal entries.
        diagonal: Matrix,
        /// Indicates whether the multiplexed-Rz gates take the shape of an upper triangle or a lower triangle. Default to true.
        #[serde(default = "default_true")]
        upper_triangle: bool,
    },
}

fn default_true() -> bool {
    true
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

/// Strategies for synthesising PauliBoxes.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum PauliSynthStrat {
    /// Synthesise gadgets individually.
    Individual,
    /// Synthesise gadgets using an efficient pairwise strategy from Cowtan et
    /// al <https://arxiv.org/abs/1906.01734>.
    Pairwise,
    /// Synthesise gadgets in commuting sets.
    #[default]
    Sets,
}

/// Strategies for partitioning Pauli tensors.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum PauliPartitionStrat {
    /// Build sets of Pauli tensors in which each qubit has the same Pauli or
    /// Pauli.I. Requires no additional CX gates for diagonalisation.
    NonConflictingSets,
    /// Build sets of mutually commuting Pauli tensors. Requires O(n^2) CX gates to diagonalise.
    #[default]
    CommutingSets,
}

/// Available methods to perform graph colouring.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum GraphColourMethod {
    /// Does not build the graph before performing the colouring; partitions
    /// while iterating through the Pauli tensors in the input order.
    #[default]
    Lazy,
    /// Builds the graph and then greedily colours by iterating through the
    /// vertices, with the highest degree first.
    LargestFirst,
    /// Builds the graph and then systematically checks all possibilities until
    /// it finds a colouring with the minimum possible number of colours. Such
    /// colourings need not be unique. Exponential time in the worst case, but
    /// often runs much faster.
    Exhaustive,
}

/// Available configurations for CXs upon decompose phase gadgets
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum CXConfigType {
    /// Linear nearest neighbour CX sequence. Linear depth.
    Snake,
    /// Every CX has same target, linear depth, good for gate cancellation.
    Star,
    /// Balanced tree: logarithmic depth, harder to route.
    #[default]
    Tree,
    /// Support for multi-qubit architectures, decomposing to 3-qubit XXPhase3
    /// gates instead of CXs where possible.
    MultiQGate,
}

/// A simple struct for Pauli strings with +/- phase, used to represent Pauli
/// strings in a stabiliser subgroup.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PauliStabiliser {
    coeff: bool,
    string: Vec<String>,
}

/// An object holding resource data for use in a [`OpType::DummyBox`].
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ResourceData {
    /// Dictionary of counts of selected [`OpType`]s.
    pub op_type_count: HashMap<OpType, ResourceBounds>,
    /// Overall gate depth.
    pub gate_depth: ResourceBounds,
    /// Dictionary of depths of selected [`OpType`]s.
    pub op_type_depth: HashMap<OpType, ResourceBounds>,
    /// Overall two-qubit-gate depth.
    pub two_qubit_gate_depth: ResourceBounds,
}

/// Structure holding a minimum and maximum value of some resource, where both
/// values are unsigned integers.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ResourceBounds {
    /// Minimum value.
    pub min: u32,
    /// Maximum value.
    pub max: u32,
}

/// Binary matrix form of a stabilizer tableau for unitary Clifford circuits.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnitaryTableau {
    /// A symplectic tableau.
    pub tab: SymplecticTableau,
    /// Ordered naming of qubits in the tableau.
    pub qubits: Vec<Register>,
}

/// Binary matrix form of a collection of Pauli strings.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SymplecticTableau {
    /// Number of rows in the tableau.
    #[serde(default)]
    pub nrows: u32,
    /// Number of columns in the tableau.
    #[serde(default)]
    pub nqubits: u32,
    /// The X matrix.
    pub xmat: Matrix<bool>,
    /// The Z matrix.
    pub zmat: Matrix<bool>,
    /// The phase matrix.
    pub phase: Matrix<bool>,
}

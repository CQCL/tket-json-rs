use crate::optype::OpType;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Hash)]
pub struct Register(pub String, pub Vec<i64>);

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CompositeGate {
    // List of Symbols
    args: Vec<String>,
    definition: Box<SerialCircuit>,
    name: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Hash)]
pub struct BitRegister {
    name: String,
    size: u32,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Hash)]
#[serde(untagged)]
pub enum ClassicalExpUnit {
    U32(u32),
    Register(Register),
    BitRegister(BitRegister),
    ClassicalExpUnit(ClassicalExp),
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Hash)]
pub struct ClassicalExp {
    args: Vec<ClassicalExpUnit>,
    op: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Hash)]
pub struct PauliStabiliser {
    coeff: bool,
    string: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Hash)]
pub struct BoxID(uuid::Uuid);

/// Box for an operation, the enum variant names come from the names
/// of the C++ operations and are renamed if the string corresponding
/// to the operation is differently named when serializing.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum OpBox {
    CircBox {
        id: BoxID,
        circuit: SerialCircuit,
    },
    Unitary1qBox {
        id: BoxID,
        // 2x2 matrix of complex numbers
        matrix: [[(f32, f32); 2]; 2],
    },
    Unitary2qBox {
        id: BoxID,
        // 4x4 matrix of complex numbers
        matrix: [[(f32, f32); 4]; 4],
    },
    Unitary3qBox {
        id: BoxID,
        // 8x8 matric of complex numbers
        matrix: Box<[[(f32, f32); 8]; 8]>,
    },
    ExpBox {
        id: BoxID,
        // 4x4 matrix of complex numbers
        matrix: [[(f32, f32); 4]; 4],
        phase: f64,
    },
    PauliExpBox {
        id: BoxID,
        paulis: Vec<String>,
        // Symengine Expr
        phase: String,
    },
    PhasePolyBox {
        id: BoxID,
        n_qubits: u32,
        qubit_indices: Vec<(u32, u32)>,
    },
    StabiliserAssertionBox {
        id: BoxID,
        stabilisers: Vec<PauliStabiliser>,
    },
    ProjectorAssertionBox {
        id: BoxID,
        matrix: Vec<Vec<(f32, f32)>>,
    },
    Composite {
        id: BoxID,
        gate: CompositeGate,
        // Vec of Symengine Expr
        params: Vec<String>,
    },
    QControlBox {
        id: BoxID,
        n_controls: u32,
        op: Box<Operation>,
    },
    ClassicalExpBox {
        id: BoxID,
        n_i: u32,
        n_io: u32,
        n_o: u32,
        exp: ClassicalExp,
    },
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Conditional {
    op: Box<Operation>,
    width: u32,
    value: u32,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Operation<P = String> {
    #[serde(rename = "type")]
    pub op_type: OpType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_qb: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<P>>,
    #[serde(rename = "box")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_box: Option<OpBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional: Option<Conditional>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Command<P = String> {
    pub op: Operation<P>,
    pub args: Vec<Register>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opgroup: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Hash)]
pub struct Permutation(pub Register, pub Register);

/// Pytket canonical circuit
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SerialCircuit<P = String> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    // Symengine Expr
    pub phase: P,
    pub commands: Vec<Command<P>>,
    pub qubits: Vec<Register>,
    pub bits: Vec<Register>,
    pub implicit_permutation: Vec<Permutation>,
}

impl<P> Operation<P> {
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

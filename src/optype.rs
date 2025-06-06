//! Defines the `OpType` enum, which represents the operation types in a quantum
//! circuit.

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::EnumString;

/// Operation types in a quantum circuit.
#[cfg_attr(feature = "pyo3", pyclass(name = "RsOpType", eq, eq_int))]
#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq, Eq, Hash, EnumString)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[non_exhaustive]
pub enum OpType {
    /// Quantum input node of the circuit
    Input,

    /// Quantum output node of the circuit
    Output,

    /// Quantum node with no predecessors, implicitly in zero state.
    Create,

    /// Quantum node with no successors, not composable with input nodes of other
    /// circuits.
    Discard,

    /// Classical input node of the circuit
    ClInput,

    /// Classical output node of the circuit
    ClOutput,

    /// No-op that must be preserved by compilation
    Barrier,

    /// FlowOp introducing a target for Branch or Goto commands
    Label,

    /// Execution jumps to a label if a condition bit is true (1),
    /// otherwise continues to next command
    Branch,

    /// Execution jumps to a label unconditionally
    Goto,

    /// Execution halts and the program terminates
    Stop,

    /// A general classical operation where all inputs are also outputs
    ClassicalTransform,

    /// Op containing a classical wasm function call.
    WASM,

    /// An operation to set some bits to specified values
    SetBits,

    /// An operation to copy some bit values
    CopyBits,

    /// A classical predicate defined by a range of values in binary encoding
    RangePredicate,

    /// A classical predicate defined by a truth table
    ExplicitPredicate,

    /// An operation defined by a truth table that modifies one bit
    ExplicitModifier,

    /// A classical operation applied to multiple bits simultaneously
    MultiBit,

    /// Global phase $\alpha \mapsto \[\e^{i\pi\alpha}\]$
    Phase,

    /// \f$ \left[ \begin{array}{cc} 1 & 0 \\ 0 & -1 \end{array} \right] \f$
    Z,

    /// \f$ \left[ \begin{array}{cc} 0 & 1 \\ 1 & 0 \end{array} \right] \f$
    X,

    /// \f$ \left[ \begin{array}{cc} 0 & -i \\ i & 0 \end{array} \right] \f$
    Y,

    /// \f$ \left[ \begin{array}{cc} 1 & 0 \\ 0 & i \end{array} \right] =
    /// \mathrm{U1}(\frac12) \f$
    S,

    /// \f$ \left[ \begin{array}{cc} 1 & 0 \\ 0 & -i \end{array} \right] =
    /// \mathrm{U1}(-\frac12) \f$
    Sdg,

    /// \f$ \left[ \begin{array}{cc} 1 & 0 \\ 0 & e^{i\pi/4} \end{array} \right]
    /// = \mathrm{U1}(\frac14) \f$
    T,

    /// \f$ \left[ \begin{array}{cc} 1 & 0 \\ 0 & e^{-i\pi/4} \end{array} \right]
    /// \equiv \mathrm{U1}(-\frac14) \f$
    Tdg,

    /// \f$ \frac{1}{\sqrt 2} \left[ \begin{array}{cc} 1 & -i \\ -i & 1
    /// \end{array} \right] = \mathrm{Rx}(\frac12) \f$
    V,

    /// \f$ \frac{1}{\sqrt 2} \left[ \begin{array}{cc} 1 & i \\ i & 1 \end{array}
    /// \right] = \mathrm{Rx}(-\frac12) \f$
    Vdg,

    /// \f$ \frac{1}{2} \left[ \begin{array}{cc} 1+i & 1-i \\ 1-i & 1+i
    /// \end{array} \right] = e^{\frac{i\pi}{4}}\mathrm{Rx}(\frac12) \f$
    SX,

    /// \f$ \frac{1}{2} \left[ \begin{array}{cc} 1-i & 1+i \\ 1+i & 1-i
    /// \end{array} \right] = e^{\frac{-i\pi}{4}}\mathrm{Rx}(-\frac12) \f$
    SXdg,

    /// \f$ \frac{1}{\sqrt 2} \left[ \begin{array}{cc} 1 & 1 \\ 1 & -1
    /// \end{array} \right] \f$
    H,

    /// \f$ \mathrm{Rx}(\alpha) = e^{-\frac12 i \pi \alpha X} = \left[
    /// \begin{array}{cc} \cos\frac{\pi\alpha}{2} & -i\sin\frac{\pi\alpha}{2} \\
    /// -i\sin\frac{\pi\alpha}{2} & \cos\frac{\pi\alpha}{2} \end{array} \right]
    /// \f$
    Rx,

    /// \f$ \mathrm{Ry}(\alpha) = e^{-\frac12 i \pi \alpha Y} = \left[
    /// \begin{array}{cc} \cos\frac{\pi\alpha}{2} & -\sin\frac{\pi\alpha}{2}
    /// \\ \sin\frac{\pi\alpha}{2} & \cos\frac{\pi\alpha}{2} \end{array} \right]
    /// \f$
    Ry,

    /// \f$ \mathrm{Rz}(\alpha) = e^{-\frac12 i \pi \alpha Z} = \left[
    /// \begin{array}{cc} e^{-\frac12 i \pi\alpha} & 0 \\ 0 & e^{\frac12 i
    /// \pi\alpha} \end{array} \right] \f$
    Rz,

    /// \f$ \mathrm{U3}(\theta, \phi, \lambda) = \left[ \begin{array}{cc}
    /// \cos\frac{\pi\theta}{2} & -e^{i\pi\lambda} \sin\frac{\pi\theta}{2} \\
    /// e^{i\pi\phi} \sin\frac{\pi\theta}{2} & e^{i\pi(\lambda+\phi)}
    /// \cos\frac{\pi\theta}{2} \end{array} \right] = e^{\frac12
    /// i\pi(\lambda+\phi)} \mathrm{Rz}(\phi) \mathrm{Ry}(\theta)
    /// \mathrm{Rz}(\lambda) \f$
    U3,

    /// \f$ \mathrm{U2}(\phi, \lambda) = \mathrm{U3}(\frac12, \phi, \lambda)
    /// = e^{\frac12 i\pi(\lambda+\phi)} \mathrm{Rz}(\phi) \mathrm{Ry}(\frac12)
    /// \mathrm{Rz}(\lambda) \f$
    U2,

    /// \f$ \mathrm{U1}(\lambda) = \mathrm{U3}(0, 0, \lambda) = e^{\frac12
    /// i\pi\lambda} \mathrm{Rz}(\lambda) \f$
    U1,

    /// \f$ \mathrm{TK1}(\alpha, \beta, \gamma) = \mathrm{Rz}(\alpha)
    /// \mathrm{Rx}(\beta) \mathrm{Rz}(\gamma) \f$
    #[serde(alias = "tk1")]
    TK1,

    /// \f$ \mathrm{TK2}(\alpha, \beta, \gamma) = \mathrm{XXPhase}(\alpha)
    /// \mathrm{YYPhase}(\beta) \mathrm{ZZPhase}(\gamma) \f$
    #[serde(alias = "tk2")]
    TK2,

    /// Controlled [`OpType::X`].
    CX,

    /// Controlled [`OpType::Y`]
    CY,

    /// Controlled [`OpType::Z`]
    CZ,

    /// Controlled [`OpType::H`]
    CH,

    /// Controlled [`OpType::V`]
    ///
    /// \f$ \left[ \begin{array}{cccc}
    /// 1 & 0 & 0 & 0 \\
    /// 0 & 1 & 0 & 0 \\
    /// 0 & 0 & \frac{1}{\sqrt 2} & -i \frac{1}{\sqrt 2} \\
    /// 0 & 0 & -i \frac{1}{\sqrt 2} & \frac{1}{\sqrt 2}
    /// \end{array} \right] \f$
    CV,

    /// Controlled [`OpType::Vdg`]
    ///
    /// \f$ \left[ \begin{array}{cccc}
    /// 1 & 0 & 0 & 0 \\
    /// 0 & 1 & 0 & 0 \\
    /// 0 & 0 & \frac{1}{\sqrt 2} & i \frac{1}{\sqrt 2} \\
    /// 0 & 0 & i \frac{1}{\sqrt 2} & \frac{1}{\sqrt 2}
    /// \end{array} \right] \f$
    CVdg,

    /// Controlled [`OpType::SX`]
    ///
    /// \f$ \left[ \begin{array}{cccc}
    /// 1 & 0 & 0 & 0 \\
    /// 0 & 1 & 0 & 0 \\
    /// 0 & 0 & \frac{1+i}{2} & \frac{1-i}{2} \\
    /// 0 & 0 & \frac{1-i}{2} & \frac{1+i}{2}
    /// \end{array} \right] \f$
    CSX,

    /// Controlled [`OpType::SXdg`]
    ///
    /// \f$ \left[ \begin{array}{cccc}
    /// 1 & 0 & 0 & 0 \\
    /// 0 & 1 & 0 & 0 \\
    /// 0 & 0 & \frac{1-i}{2} & \frac{1+i}{2} \\
    /// 0 & 0 & \frac{1+i}{2} & \frac{1-i}{2}
    /// \end{array} \right] \f$
    CSXdg,

    /// Controlled [`OpType::S`] gate
    CS,

    /// Controlled [`OpType::Sdg`] gate
    CSdg,

    /// Controlled [`OpType::Rz`]
    ///
    /// \f$ \mathrm{CRz}(\alpha) = \left[ \begin{array}{cccc} 1 & 0 & 0 & 0 \\ 0
    /// & 1 & 0 & 0 \\ 0 & 0 & e^{-\frac12 i \pi\alpha} & 0 \\ 0 & 0 & 0 &
    /// e^{\frac12 i \pi\alpha} \end{array} \right] \f$
    ///
    /// The phase parameter \f$ \alpha \f$ is defined modulo \f$ 4 \f$.
    CRz,

    /// Controlled [`OpType::Rx`]
    ///
    /// \f$ \mathrm{CRx}(\alpha) = \left[ \begin{array}{cccc} 1 & 0 & 0 & 0 \\ 0
    /// & 1 & 0 & 0 \\ 0 & 0 & \cos \frac{\pi \alpha}{2} & -i \sin \frac{\pi
    /// \alpha}{2}
    /// \\ 0 & 0 & -i \sin \frac{\pi \alpha}{2}  & \cos \frac{\pi \alpha}{2}
    /// \end{array} \right] \f$
    ///
    /// The phase parameter \f$ \alpha \f$ is defined modulo \f$ 4 \f$.
    CRx,

    /// Controlled [`OpType::Ry`]
    ///
    /// \f$ \mathrm{CRy}(\alpha) = \left[ \begin{array}{cccc} 1 & 0 & 0 & 0 \\ 0
    /// & 1 & 0 & 0 \\ 0 & 0 & \cos \frac{\pi \alpha}{2} & -\sin \frac{\pi
    /// \alpha}{2}
    /// \\ 0 & 0 & \sin \frac{\pi \alpha}{2}  & \cos \frac{\pi \alpha}{2}
    /// \end{array} \right] \f$
    ///
    /// The phase parameter \f$ \alpha \f$ is defined modulo \f$ 4 \f$.
    CRy,

    /// Controlled [`OpType::U1`]
    ///
    /// \f$ \mathrm{CU1}(\alpha) = \left[ \begin{array}{cccc} 1 & 0 & 0 & 0 \\ 0
    /// & 1 & 0 & 0 \\ 0 & 0 & 1 & 0 \\ 0 & 0 & 0 & e^{i\pi\alpha} \end{array}
    /// \right] \f$
    CU1,

    /// Controlled [`OpType::U3`]
    CU3,

    /// \f$ \alpha \mapsto e^{-\frac12 i \pi\alpha Z^{\otimes n}} \f$
    PhaseGadget,

    /// Controlled [`OpType::CX`]
    CCX,

    /// Swap two qubits
    SWAP,

    /// Controlled [`OpType::SWAP`]
    CSWAP,

    /// Three-qubit gate that swaps the first and third qubits
    BRIDGE,

    /// Identity
    #[allow(non_camel_case_types)]
    #[default]
    noop,

    /// Measure a qubit, producing a classical output
    Measure,

    /// Measure a qubit producing no output
    Collapse,

    /// Reset a qubit to the zero state
    Reset,

    /// \f$ \frac{1}{\sqrt 2} \left[ \begin{array}{cccc} 0 & 0 & 1 & i \\ 0 & 0 &
    /// i & 1 \\ 1 & -i & 0 & 0 \\ -i & 1 & 0 & 0 \end{array} \right] \f$
    ECR,

    /// \f$ \alpha \mapsto e^{\frac14 i \pi\alpha (X \otimes X + Y \otimes Y)}
    /// = \left[ \begin{array}{cccc} 1 & 0 & 0 & 0 \\ 0 & \cos\frac{\pi\alpha}{2}
    /// & i\sin\frac{\pi\alpha}{2} & 0 \\ 0 & i\sin\frac{\pi\alpha}{2} &
    /// \cos\frac{\pi\alpha}{2} & 0 \\ 0 & 0 & 0 & 1 \end{array} \right] \f$
    ///
    /// Also known as an XY gate.
    ISWAP,

    /// \f$ (\alpha, \beta) \mapsto \mathrm{Rz}(\beta) \mathrm{Rx}(\alpha)
    /// \mathrm{Rz}(-\beta) \f$
    PhasedX,

    /// PhasedX gates on multiple qubits
    NPhasedX,

    /// \f$ \mathrm{ZZPhase}(\frac12) \f$
    ZZMax,

    /// \f$ \alpha \mapsto e^{-\frac12 i \pi\alpha (X \otimes X)} = \left[
    /// \begin{array}{cccc} \cos\frac{\pi\alpha}{2} & 0 & 0 &
    /// -i\sin\frac{\pi\alpha}{2} \\ 0 & \cos\frac{\pi\alpha}{2} &
    /// -i\sin\frac{\pi\alpha}{2} & 0 \\ 0 & -i\sin\frac{\pi\alpha}{2} &
    /// \cos\frac{\pi\alpha}{2} & 0 \\ -i\sin\frac{\pi\alpha}{2} & 0 & 0 &
    /// \cos\frac{\pi\alpha}{2} \end{array} \right] \f$
    XXPhase,

    /// \f$ \alpha \mapsto e^{-\frac12 i \pi\alpha (Y \otimes Y)} = \left[
    /// \begin{array}{cccc} \cos\frac{\pi\alpha}{2} & 0 & 0 &
    /// i\sin\frac{\pi\alpha}{2} \\ 0 & \cos\frac{\pi\alpha}{2} &
    /// -i\sin\frac{\pi\alpha}{2} & 0 \\ 0 & -i\sin\frac{\pi\alpha}{2} &
    /// \cos\frac{\pi\alpha}{2} & 0 \\ i\sin\frac{\pi\alpha}{2} & 0 & 0 &
    /// \cos\frac{\pi\alpha}{2} \end{array} \right] \f$
    YYPhase,

    /// \f$ \alpha \mapsto e^{-\frac12 i \pi\alpha (Z \otimes Z)} = \left[
    /// \begin{array}{cccc} e^{-\frac12 i \pi\alpha} & 0 & 0 & 0 \\ 0 &
    /// e^{\frac12 i \pi\alpha} & 0 & 0 \\ 0 & 0 & e^{\frac12 i \pi\alpha} & 0 \\ 0
    /// & 0 & 0 & e^{-\frac12 i \pi\alpha} \end{array} \right] \f$
    ZZPhase,

    /// Three-qubit phase MSGate
    XXPhase3,

    /// \f$ \alpha \mapsto e^{-\frac12 i\pi\alpha \cdot \mathrm{SWAP}} = \left[
    /// \begin{array}{cccc} e^{-\frac12 i \pi\alpha} & 0 & 0 & 0 \\ 0 &
    /// \cos\frac{\pi\alpha}{2} & -i\sin\frac{\pi\alpha}{2} & 0 \\ 0 &
    /// -i\sin\frac{\pi\alpha}{2} & \cos\frac{\pi\alpha}{2} & 0 \\ 0 & 0 & 0 &
    /// e^{-\frac12 i \pi\alpha} \end{array} \right] \f$
    ESWAP,

    /// \f$ (\alpha, \beta) \mapsto \left[ \begin{array}{cccc} 1 & 0 & 0 & 0 \\ 0
    /// & \cos \pi\alpha & -i\sin  \pi\alpha & 0 \\ 0 &
    /// -i\sin \pi\alpha & \cos \pi\alpha & 0 \\ 0 & 0 & 0 &
    /// e^{-i\pi\beta} \end{array} \right] \f$
    FSim,

    /// Fixed instance of a [`OpType::FSim`] gate with parameters
    /// \f$ (\frac12, \frac16) \f$:
    /// \f$ \left[ \begin{array}{cccc} 1 & 0 & 0 & 0 \\ 0 & 0 & -i & 0 \\ 0 & -i
    /// & 0 & 0 \\ 0 & 0 & 0 & e^{-i\pi/6} \end{array} \right] \f$
    Sycamore,

    /// Fixed instance of a [`OpType::ISWAP`] gate with parameter \f$ 1.0 \f$:
    /// \f$ \left[ \begin{array}{cccc} 1 & 0 & 0 & 0 \\ 0 & 0 & i & 0 \\ 0 & i
    /// & 0 & 0 \\ 0 & 0 & 0 & 1 \end{array} \right] \f$
    ISWAPMax,

    /// ISwap gate with extra `Rz`s on each qubit
    // TODO: Matrix description
    PhasedISWAP,

    /// N-controlled [`OpType::Rx`]
    CnRx,

    /// N-controlled [`OpType::Ry`]
    CnRy,

    /// N-controlled [`OpType::Rz`]
    CnRz,

    /// Multiply-controlled [`OpType::X`]
    CnX,

    /// Multiply-controlled [`OpType::Y`]
    CnY,

    /// Multiply-controlled [`OpType::Z`]
    CnZ,

    /// GPi gate
    ///
    /// \f$ (\\phi) \\mapsto \\left[ \\begin{array}{cc} 0 &
    /// e^{-i\\pi\\phi} \\\\ e^{i\\pi\\phi} & 0 \\end{array} \\right] \f$
    GPI,

    /// GPi2 gate
    ///
    /// \f$
    /// (\\phi) \\mapsto \\frac{1}{\\sqrt 2} \\left[
    /// \\begin{array}{cc} 1 & -ie^{-i\\pi\\phi} \\\\ -ie^{i\\pi\\phi} &
    /// 1 \\end{array} \\right]
    /// \f$
    GPI2,

    /// AAMS gate
    ///
    /// \f$
    ///
    /// (\\theta, \\phi_0, \\phi_1) \\mapsto \\left[ "
    /// \\begin{array}{cccc} \\cos\\frac{\\pi\\theta}{2} & 0 & 0 &
    /// -ie^{-i\\pi(\\phi_0+\\phi_1)}\\sin\\frac{\\pi\\theta}{2} \\\\
    /// 0 &
    /// \\cos\\frac{\\pi\\theta}{2} &
    /// -ie^{i\\pi(\\phi_1-\\phi_0)}\\sin\\frac{\\pi\\theta}{2} & 0 \\\\ 0
    /// &
    /// -ie^{i\\pi(\\phi_0-\\phi_1)}\\sin\\frac{\\pi\\theta}{2} &
    /// \\cos\\frac{\\pi\\theta}{2} & 0 \\\\
    /// -ie^{i\\pi(\\phi_0+\\phi_1)}\\sin\\frac{\\pi\\theta}{2} & 0 & 0 &
    /// \\cos\\frac{\\pi\\theta}{2} \\end{array} \\right]`
    /// \f$
    AAMS,

    /// See [`CircBox`]
    ///
    ///   [`CircBox`]: crate::opbox::OpBox::CircBox
    CircBox,

    /// See [`Unitary1qBox`]
    ///
    ///   [`Unitary1qBox`]: crate::opbox::OpBox::Unitary1qBox
    Unitary1qBox,

    /// See [`Unitary2qBox`]
    ///
    ///   [`Unitary2qBox`]: crate::opbox::OpBox::Unitary2qBox
    Unitary2qBox,

    /// See [`Unitary3qBox`]
    ///
    ///   [`Unitary3qBox`]: crate::opbox::OpBox::Unitary3qBox
    Unitary3qBox,

    /// See [`ExpBox`]
    ///
    ///   [`ExpBox`]: crate::opbox::OpBox::ExpBox
    ExpBox,

    /// See [`PauliExpBox`]
    ///
    ///   [`PauliExpBox`]: crate::opbox::OpBox::PauliExpBox
    PauliExpBox,

    /// See [`PauliExpPairBox`]
    ///
    ///   [`PauliExpPairBox`]: crate::opbox::OpBox::PauliExpPairBox
    PauliExpPairBox,

    /// See [`PauliExpCommutingSetBox`]
    ///
    ///   [`PauliExpCommutingSetBox`]: crate::opbox::OpBox::PauliExpCommutingSetBox
    PauliExpCommutingSetBox,

    /// See [`TermSequenceBox`]
    ///
    ///   [`TermSequenceBox`]: crate::opbox::OpBox::TermSequenceBox
    TermSequenceBox,

    /// NYI
    CliffBox,

    /// See [`PhasePolyBox`]
    ///
    ///   [`PhasePolyBox`]: crate::opbox::OpBox::PhasePolyBox
    PhasePolyBox,

    /// NYI
    #[serde(alias = "Condition")]
    Conditional,

    /// See [`StabiliserAssertionBox`]
    ///
    ///   [`StabiliserAssertionBox`]: crate::opbox::OpBox::StabiliserAssertionBox
    StabiliserAssertionBox,

    /// See [`ProjectorAssertionBox`]
    ///
    ///   [`ProjectorAssertionBox`]: crate::opbox::OpBox::ProjectorAssertionBox
    ProjectorAssertionBox,

    /// See [`CustomGate`]
    ///
    ///   [`CustomGate`]: crate::opbox::OpBox::CustomGate
    #[serde(alias = "Composite")]
    CustomGate,

    /// See [`QControlBox`]
    ///
    ///   [`QControlBox`]: crate::opbox::OpBox::QControlBox
    QControlBox,

    /// See [`UnitaryTableauBox`]
    ///
    ///  [`UnitaryTableauBox`]: crate::opbox::OpBox::UnitaryTableauBox
    UnitaryTableauBox,

    /// See [`ClassicalExpBox`]
    ///
    /// Deprecated. Use [`OpType::ClExpr`] instead.
    ///
    ///   [`ClassicalExpBox`]: crate::opbox::OpBox::ClassicalExpBox
    ClassicalExpBox,

    /// See [`MultiplexorBox`]
    ///
    ///   [`MultiplexorBox`]: crate::opbox::OpBox::MultiplexorBox
    MultiplexorBox,

    /// See [`MultiplexedRotationBox`]
    ///
    ///  [`MultiplexedRotationBox`]: crate::opbox::OpBox::MultiplexedRotationBox
    MultiplexedRotationBox,

    /// See [`MultiplexedU2Box`]
    ///
    ///   [`MultiplexedU2Box`]: crate::opbox::OpBox::MultiplexedU2Box
    MultiplexedU2Box,

    /// See [`MultiplexedTensoredU2Box`]
    ///
    ///   [`MultiplexedTensoredU2Box`]: crate::opbox::OpBox::MultiplexedTensoredU2Box
    MultiplexedTensoredU2Box,

    /// See [`ToffoliBox`]
    ///
    ///   [`ToffoliBox`]: crate::opbox::OpBox::ToffoliBox
    ToffoliBox,

    /// See [`ConjugationBox`]
    ///
    ///   [`ConjugationBox`]: crate::opbox::OpBox::ConjugationBox
    ConjugationBox,

    /// See [`DummyBox`]
    ///
    ///   [`DummyBox`]: crate::opbox::OpBox::DummyBox
    DummyBox,

    /// See [`StatePreparationBox`]
    ///
    ///   [`StatePreparationBox`]: crate::opbox::OpBox::StatePreparationBox
    StatePreparationBox,

    /// See [`DiagonalBox`]
    ///
    ///   [`DiagonalBox`]: crate::opbox::OpBox::DiagonalBox
    DiagonalBox,

    /// Classical expression.
    ///
    /// An operation of this type is accompanied by a [`ClExpr`] object.
    ///
    /// This is a replacement of the deprecated [`OpType::ClassicalExpBox`].
    ///
    ///   [`ClExpr`]: crate::clexpr::ClExpr
    ClExpr,
}

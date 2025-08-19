# /// script
# requires-python = ">=3.13"
# dependencies = [
#     "pytket>=1.34",
# ]
# ///

import json

from pytket import Circuit
from pytket.qasm import circuit_from_qasm_str


def qasm_circuit() -> Circuit:
    qasm = """OPENQASM 2.0;
        include "hqslib1.inc";
        qreg q[3];
        creg a[3];
        creg b[3];
        creg c[3];
        creg d[3];
        d = (((a + b) / 2) - c);

        h q[0];
        z q[2];
        cx q[2], q[1];
        """
    return circuit_from_qasm_str(qasm)


if __name__ == "__main__":
    print(json.dumps(qasm_circuit().to_dict(), indent=2))

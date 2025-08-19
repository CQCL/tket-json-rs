# /// script
# requires-python = ">=3.13"
# dependencies = [
#     "pytket>=2.9.1",
# ]
# ///

import json

from pytket import Circuit


def rng_circuit() -> Circuit:
    circ = Circuit(1, 1)
    seed = circ.add_c_register("seed", 64)
    bound = circ.add_c_register("bound", 32)
    index = circ.add_c_register("index", 32)
    num = circ.add_c_register("num", 32)
    circ.add_c_setbits([True, True, True], [seed[1], seed[3], seed[5]])
    circ.set_rng_seed(seed)
    circ.add_c_setbits([True], [bound[1]])
    circ.set_rng_bound(bound)
    circ.set_rng_index(index)
    circ.get_rng_num(num)
    circ.H(0, condition_bits=[num[0]], condition_value=1)
    circ.measure_all()
    assert circ.n_gates == 8

    return circ


if __name__ == "__main__":
    print(json.dumps(rng_circuit().to_dict(), indent=4))

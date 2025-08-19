# Release notes

## 0.7.5 (2025-08-19)

### Features

- Implement `Display` and `Copy` for `OpType` ([#145](https://github.com/CQCL/tket-json-rs/issues/145))
- Add missing RNG ops ([#148](https://github.com/CQCL/tket-json-rs/issues/148))


## 0.7.4 (2025-06-10)

### Documentation

- Clearup clexpr argument index documentation ([#138](https://github.com/CQCL/tket-json-rs/issues/138))

### Features

- Method for generating BoxIDs ([#139](https://github.com/CQCL/tket-json-rs/issues/139))


## 0.7.3 (2025-06-02)

### Features

- Add schemars optional feature ([#129](https://github.com/CQCL/tket-json-rs/issues/129))


## 0.7.2 (2025-02-24)

### Features

- Updated dependencies


## 0.7.1 (2024-11-29)

### Features

- Update to pyo3 23 (allow free-threaded python) ([#102](https://github.com/CQCL/tket-json-rs/pull/102))


## 0.7.0 (2024-11-13)

### ⚠ BREAKING CHANGES

- Made `SerialCircuit` non exhaustive.
- `Register` renamed to `ElementId`, `Qubit` and `Bit`
- Moved some definitions from `::circuit_json` to `::register`
- Bumped MSRV to rust 1.75
- Renamed `circuit_json::CompositeGate` to `CustomGate`

### Features

- [**breaking**] `created/discarded_qubits` circuit attribute ([#87](https://github.com/CQCL/tket-json-rs/pull/87))
- Support classical expressions ([#86](https://github.com/CQCL/tket-json-rs/pull/86))
- [**breaking**] Rename `Register` and cleanup definitions ([#89](https://github.com/CQCL/tket-json-rs/pull/89))
- [**breaking**] Support old `Composite` alias for `CustomGate` ([#91](https://github.com/CQCL/tket-json-rs/pull/91))



## 0.6.2 (2024-10-21)

### Features

- Updated pyo3 dependency to `0.22` ([#73](https://github.com/CQCL/tket-json-rs/pull/73))


## 0.6.1 (2024-08-07)

### Features

- `map_params` helpers on the parametric structs ([#65](https://github.com/CQCL/tket-json-rs/pull/65))


## 0.6.0 (2024-08-06)

### Bug Fixes

- [**breaking**] Update ToffoliBox permutation type ([#64](https://github.com/CQCL/tket-json-rs/pull/64))

### Features

- [**breaking**] Add support for WASM operations ([#61](https://github.com/CQCL/tket-json-rs/pull/61))


## 0.5.1 (2024-07-24)

### Features

- Handle legacy condition optype ([#59](https://github.com/CQCL/tket-json-rs/pull/59))


## 0.5.0 (2024-07-09)

### Features

- Handle legacy tk1 optype  ([#54](https://github.com/CQCL/tket-json-rs/pull/54))
- [**breaking**] Add missing classical operation params ([#56](https://github.com/CQCL/tket-json-rs/pull/56))
- [**breaking**] Add missing `data` operation param ([#56](https://github.com/CQCL/tket-json-rs/pull/56))
- [**breaking**] Make more things `non_exhaustive` ([#56](https://github.com/CQCL/tket-json-rs/pull/56))


## 0.4.2 (2024-07-05)

### Bug Fixes

- Add missing `CnRx` `CnRz` ops ([#49](https://github.com/CQCL/tket-json-rs/pull/49))
- Matrix encoding roundtrip losing precision. Use `f64` instead of `f32`s. ([#48](https://github.com/CQCL/tket-json-rs/pull/48))


## 0.4.1 (2024-04-16)

### Bug Fixes

- Make some missing fields pub ([#43](https://github.com/CQCL/tket-json-rs/pull/43))

### Features

- Add GPI, GPI2, and AAMS gates ([#42](https://github.com/CQCL/tket-json-rs/pull/42))


## 0.4.0 (2024-04-08)

This release adds various missing `OpType` definitions, and fixes some existing box definitions with incorrect parameters.

### Bug Fixes

- Add missing OpTypes ([#37](https://github.com/CQCL/tket-json-rs/pull/37))

### Miscellaneous Tasks

- [**breaking**] Update pyo3 to 0.21 ([#33](https://github.com/CQCL/tket-json-rs/pull/33))

### Refactor

- Move `OpBox` to a new module ([#35](https://github.com/CQCL/tket-json-rs/pull/35))


## 0.3.1 (2024-02-26)

### Features

- ToffoliBox encoding ([#28](https://github.com/CQCL/tket-json-rs/pull/28))

## 0.3.0 (2023-11-09)

### Features

- [**breaking**] Non-locking python conversion calls ([#25](https://github.com/CQCL/tket-json-rs/pull/25))

## 0.2.0 (2023-10-16)

### Features

- Expand box types and match order ([#15](https://github.com/CQCL/tket-json-rs/pull/15))

## 0.1.1 (2023-09-19)

### Features

- Add tk2 op type ([#13](https://github.com/CQCL/tket-json-rs/pull/13))

## v0.1.0 (2023-08-18)

-   Initial release.

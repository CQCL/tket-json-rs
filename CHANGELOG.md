# Release notes

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

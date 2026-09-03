# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-09-03

Initial public release. Bridge crate that turns the
[schema-bridge](https://github.com/ynishi/schema-bridge) `Schema` IR into
[lshape](https://github.com/ynishi/lshape) (Lua Schema-as-Data) type
definitions — plain Lua source text built from `T.shape` / `T.array_of` /
`T.one_of` combinators, loadable in any environment where
`require("lshape")` resolves (e.g. via
[`mlua-lshape`](https://crates.io/crates/mlua-lshape)).

### Added

- `schema_to_lshape(&Schema) -> Result<String, LshapeError>` — single
  schema to lshape expression.
- `generate_lshape_file(&[(&str, Schema)])` — complete Lua module text
  (one `M.<Name>` entry per exported type, `return M`).
- `export_to_lua_file` / `export_lshape_types!` macro — write the module
  to disk straight from `#[derive(SchemaBridge)]` types.
- Type mapping: primitives, `Array` → `T.array_of`, `Record` → `T.map_of`,
  `Object` → `T.shape` (optional fields get `:is_optional()`),
  `Enum` → `T.one_of`, `Union` → `T.any_of` with `Null` variants folded
  into `:is_optional()`, `Ref` → `T.ref`, `Constraints.one_of` →
  `T.one_of`.
- Errors: standalone `Null` and null-only `Union` are rejected with a
  path-annotated `LshapeError` instead of emitting invalid Lua.
- Round-trip tests: generated modules are loaded into a real Lua VM
  (via `mlua-lshape`) and exercised with `lshape.check.check`.

### Known limitations

- `Integer` lowers to `T.number` (integer-ness not checked by lshape).
- `Tuple` lowers to `T.table` with the element list kept as
  `:describe(...)` doc text.
- `min` / `max` / `min_len` / `max_len` constraints are preserved as
  `:describe(...)` doc text only — not enforced at check time.

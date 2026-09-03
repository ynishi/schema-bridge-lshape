//! Round-trip tests: generated Lua source is loaded into a real Lua VM
//! (via mlua-lshape's vendored lshape) and exercised with `check.check`.

use mlua::{Lua, Table};
use schema_bridge_core::{Constraints, Field, Schema};
use schema_bridge_lshape::generate_lshape_file;

fn lua_with_module(src: &str) -> Lua {
    let lua = Lua::new();
    mlua_lshape::install(&lua).unwrap();
    let module: Table = lua.load(src).eval().unwrap();
    lua.globals().set("M", module).unwrap();
    lua
}

#[test]
fn generated_module_validates_with_lshape() {
    let user = Schema::Object(vec![
        Field::new("name", Schema::String),
        Field {
            name: "age".into(),
            schema: Schema::Integer,
            required: true,
            constraints: Constraints {
                min: Some(0.0),
                max: Some(150.0),
                ..Default::default()
            },
        },
        Field::optional("email", Schema::String),
        Field::new("tags", Schema::Array(Box::new(Schema::String))),
        Field {
            name: "status".into(),
            schema: Schema::String,
            required: true,
            constraints: Constraints {
                one_of: Some(vec!["active".into(), "inactive".into()]),
                ..Default::default()
            },
        },
    ]);
    let src = generate_lshape_file(&[("User", user)]).unwrap();
    let lua = lua_with_module(&src);

    lua.load(
        r#"
        local check = require("lshape").check

        local ok, why = check.check(
            { name = "a", age = 30, tags = { "x" }, status = "active" }, M.User)
        assert(ok, tostring(why))

        assert(not check.check(
            { age = 30, tags = {}, status = "active" }, M.User),
            "missing required name must fail")

        assert(check.check(
            { name = "a", age = 30, email = "x@y", tags = {}, status = "active" }, M.User),
            "optional field present must pass")

        assert(not check.check(
            { name = "a", age = 30, tags = {}, status = "bogus" }, M.User),
            "one_of violation must fail")

        assert(not check.check(
            { name = 42, age = 30, tags = {}, status = "active" }, M.User),
            "wrong field type must fail")
        "#,
    )
    .exec()
    .unwrap();
}

#[test]
fn nullable_union_and_map_round_trip() {
    let config = Schema::Object(vec![
        Field::new(
            "label",
            // Option<String> as produced by schema-bridge-core:
            Schema::Union(vec![Schema::String, Schema::Null]),
        ),
        Field::new(
            "env",
            Schema::Record {
                key: Box::new(Schema::String),
                value: Box::new(Schema::String),
            },
        ),
        Field::new("value", Schema::Union(vec![Schema::String, Schema::Number])),
    ]);
    let src = generate_lshape_file(&[("Config", config)]).unwrap();
    let lua = lua_with_module(&src);

    lua.load(
        r#"
        local check = require("lshape").check

        local ok, why = check.check(
            { label = "x", env = { HOME = "/root" }, value = 42 }, M.Config)
        assert(ok, tostring(why))

        local ok2, why2 = check.check(
            { env = {}, value = "s" }, M.Config)
        assert(ok2, tostring(why2))

        assert(not check.check(
            { env = { HOME = 1 }, value = "s" }, M.Config),
            "map value type violation must fail")

        assert(not check.check(
            { env = {}, value = true }, M.Config),
            "union variant violation must fail")
        "#,
    )
    .exec()
    .unwrap();
}

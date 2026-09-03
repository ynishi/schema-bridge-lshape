//! End-to-end: #[derive(SchemaBridge)] → export_lshape_types! → Lua file.

use schema_bridge::SchemaBridge;
use schema_bridge_lshape::export_lshape_types;

#[derive(SchemaBridge)]
#[allow(dead_code)]
struct User {
    name: String,
    age: u8,
    email: Option<String>,
}

#[derive(SchemaBridge)]
#[allow(dead_code)]
struct Point {
    x: f64,
    y: f64,
}

#[test]
fn export_macro_writes_lua_module() {
    let path = std::env::temp_dir().join("schema_bridge_lshape_derive_test.lua");
    let path_str = path.to_str().unwrap();

    export_lshape_types!(path_str, User, Point).unwrap();

    let content = std::fs::read_to_string(&path).unwrap();
    assert!(content.contains("M.User = T.shape({"));
    assert!(content.contains("name = T.string,"));
    assert!(content.contains("age = T.number,"));
    assert!(content.contains("email = T.string:is_optional(),"));
    assert!(content.contains("M.Point = T.shape({"));
    assert!(content.ends_with("return M\n"));

    std::fs::remove_file(&path).ok();
}

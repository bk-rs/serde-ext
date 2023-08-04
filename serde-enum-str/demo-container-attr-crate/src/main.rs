#[derive(
    serde_enum_str_demo_utils::reexport::serde_enum_str::Deserialize_enum_str,
    serde_enum_str_demo_utils::reexport::serde_enum_str::Serialize_enum_str,
    Debug,
    PartialEq,
)]
#[serde(crate = "serde_enum_str_demo_utils::reexport::serde")]
enum Foo {
    A,
}

fn main() {}

#[test]
fn simple() {
    assert_eq!(serde_json::to_string(&Foo::A).unwrap(), r#""A""#);
    assert_eq!(serde_json::from_str::<Foo>(r#""A""#).unwrap(), Foo::A);
}
